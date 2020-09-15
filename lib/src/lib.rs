pub mod prelude;
pub mod web_str;
mod style_storage;
mod enclose;
mod query;
mod storage;
pub mod state;
pub mod create;
mod events;

pub use hobo_css as css;
pub use hobo_derive::*;
pub use web_sys;
pub use paste;
use crate::prelude::*;
use std::collections::{HashMap, HashSet};
use std::any::TypeId;
use std::rc::Rc;
use once_cell::sync::Lazy;
use std::cell::{RefCell, Cell, Ref, RefMut};
use std::marker::PhantomData;
use storage::*;
use owning_ref::{OwningRef, OwningRefMut, OwningHandle};
use std::borrow::Cow;
use style_storage::STYLE_STORAGE;

fn dom() -> web_sys::Document { web_sys::window().expect("no window").document().expect("no document") }

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Entity {
	id: u32,
	generation: u32
}

impl Entity {
	fn root() -> Self { Self { id: 0, generation: 0 } }
}

pub trait AsEntity {
	fn as_entity(&self) -> Entity;
}

impl AsEntity for Entity {
	fn as_entity(&self) -> Entity { *self }
}

pub struct System {
	f: RefCell<Box<dyn FnMut(Entity) + 'static>>,
	query: fn(&World, Entity) -> bool,
	interests: fn() -> HashSet<TypeId>,
	scheduled: Cell<bool>,
}

impl System {
	pub fn f(&self, entity: Entity) { (self.f.borrow_mut())(entity) }
	pub fn interests(&self) -> HashSet<TypeId> { (self.interests)() }
	pub fn query(&self, world: &World, entity: Entity) -> bool { (self.query)(world, entity) }
}

type StorageRc = Rc<RefCell<Box<dyn DynStorage>>>;

#[derive(Debug)]
struct Entities {
	free_ids: Vec<u32>,
	generations: Vec<u32>,
}

impl Default for Entities {
	fn default() -> Self { Self {
		free_ids: Default::default(),
		generations: vec![0],
	} }
}

// Entity(0) is for holding resources
#[derive(Default)]
pub struct World {
	pub storages: RefCell<HashMap<TypeId, StorageRc>>,
	entities: RefCell<Entities>,

	// TODO: should keep weak refs and have a separate map with strong refs so systems can be deregistered
	pub systems_interests: RefCell<HashMap<TypeId, Vec<Rc<System>>>>,
}

unsafe impl Send for World {}
unsafe impl Sync for World {}

pub static WORLD: Lazy<World> = Lazy::new(|| {
	let world = World::default();

	let sys = <Or<Added<(Classes,)>, Modified<(Classes,)>>>::run(move |entity| {
		if let Some(element) = web_sys::Element::try_get(entity) {
			use std::fmt::Write;

			let storage = WORLD.storage::<Classes>();
			let classes = storage.get(entity).unwrap();
			let mut res = format!("e{}g{}", entity.id, entity.generation);

			if let Some(id) = &classes.type_tag {
				use std::hash::{Hash, Hasher};

				let mut hasher = std::collections::hash_map::DefaultHasher::new();
				id.hash(&mut hasher);
				let id = hasher.finish();
				write!(&mut res, " t{}", id).unwrap();
			}

			STYLE_STORAGE.with(|x| {
				let mut style_storage = x.borrow_mut();
				for style in classes.styles.values() {
					write!(&mut res, " {}", style_storage.fetch(style.clone())).unwrap();
				}
			});

			element.set_attribute(web_str::class(), &res).expect("can't set class attribute");
		}
	});

	world.new_system(sys);

	create::register_systems(&world);

	world
});

type StorageRef<'a, Component> = OwningRef<OwningHandle<Rc<RefCell<Box<(dyn storage::DynStorage + 'static)>>>, Ref<'a, Box<dyn storage::DynStorage>>>, SimpleStorage<Component>>;
type StorageMutRef<'a, Component> = StorageGuard<'a, Component, OwningRefMut<OwningHandle<Rc<RefCell<Box<(dyn storage::DynStorage + 'static)>>>, RefMut<'a, Box<dyn storage::DynStorage>>>, SimpleStorage<Component>>>;

impl World {
	fn dyn_storage<Component: 'static>(&self) -> Rc<RefCell<Box<dyn DynStorage>>> {
		Rc::clone(self.storages.try_borrow_mut().expect("dyn_storage -> storages.try_borrow_mut")
			.entry(TypeId::of::<Component>())
			.or_insert_with(|| Rc::new(RefCell::new(Box::new(SimpleStorage::<Component>::default())))))
	}

	pub fn storage_mut<Component: 'static>(&self) -> StorageMutRef<Component> {
		let storage = OwningRefMut::new(OwningHandle::new_mut(self.dyn_storage::<Component>()))
			.map_mut(|x| x.as_any_mut().downcast_mut::<SimpleStorage<Component>>().unwrap());
		StorageGuard(self, Some(storage))
	}

	pub fn storage<Component: 'static>(&self) -> StorageRef<Component> {
		OwningRef::new(OwningHandle::new(self.dyn_storage::<Component>()))
			.map(|x| x.as_any().downcast_ref::<SimpleStorage<Component>>().unwrap())
	}

	pub fn register_resource<T: 'static>(&self, resource: T) { self.storage_mut::<T>().add(Entity::root(), resource); }

	pub fn resource<T: 'static>(&self) -> OwningRef<StorageRef<T>, T> {
		OwningRef::new(self.storage::<T>()).map(|x| x.get(Entity::root()).unwrap())
	}

	pub fn resource_mut<T: 'static>(&self) -> OwningRefMut<StorageMutRef<T>, T> {
		OwningRefMut::new(self.storage_mut::<T>()).map_mut(|x| x.get_mut(Entity::root()).unwrap())
	}

	pub fn try_resource<T: 'static>(&self) -> Option<OwningRef<StorageRef<T>, T>> {
		if !self.storage::<T>().has(Entity::root()) { return None; }
		Some(OwningRef::new(self.storage::<T>()).map(|x| x.get(Entity::root()).unwrap()))
	}

	pub fn try_resource_mut<T: 'static>(&self) -> Option<OwningRefMut<StorageMutRef<T>, T>> {
		if !self.storage::<T>().has(Entity::root()) { return None; }
		Some(OwningRefMut::new(self.storage_mut::<T>()).map_mut(|x| x.get_mut(Entity::root()).unwrap()))
	}

	pub fn new_entity(&self) -> Entity {
		let mut entities = self.entities.try_borrow_mut().expect("new_entity entities.try_borrow_mut");
		if let Some(id) = entities.free_ids.pop() {
			Entity { id, generation: entities.generations[id as usize] }
		} else {
			let id = entities.generations.len() as u32;
			entities.generations.push(0);
			Entity { id, generation: 0 }
		}
	}

	pub fn new_system(&self, sys: System) {
		let sys = Rc::new(sys);
		for interest in sys.interests().into_iter() {
			let mut systems_interests = self.systems_interests.try_borrow_mut().expect("new_system systems_interests.try_borrrow_mut");
			systems_interests.entry(interest).or_insert_with(Vec::new).push(Rc::clone(&sys));
		}
	}

	// TODO: add check systems in entity components
	pub fn remove_entity(&self, entity: impl AsEntity) {
		let entity = entity.as_entity();
		if WORLD.is_dead(entity) { log::warn!("remove entity already dead {:?}", entity); return; }

		if let Some(children) = Children::try_get(entity).map(|x| x.0.clone()) {
			for child in children { self.remove_entity(child); }
		}

		{
			let mut entities = self.entities.try_borrow_mut().expect("remove_entity entities.try_borrow_mut");
			entities.free_ids.push(entity.id);
			entities.generations[entity.id as usize] += 1;
		}

		if let Some(parent) = Parent::try_get(entity).map(|x| x.0) {
			let mut children = Children::get_mut(parent);
			if let Some(child_pos) = children.0.iter().position(|&x| x == entity) { children.0.remove(child_pos); }
		}

		let mut set: HashSet<TypeId> = HashSet::new();

		for (&component_id, storage) in self.storages.try_borrow().expect("remove_entity storages.try_borrow .. remove").iter() {
			if storage.try_borrow().expect("remove_entity storages -> storage.try_borrow .. remove").dyn_has(entity) {
				set.insert(component_id);
				storage.try_borrow_mut().expect("remove_entity storages -> storage.try_borrow_mut .. remove").dyn_remove(entity);
			}
		}

		let systems = self.schedule_systems(std::iter::once(entity), set.iter().copied());

		{
			let storages = self.storages.try_borrow().expect("remove_entity storages.try_borrow .. flush");
			for component_id in &set {
				storages[component_id].try_borrow_mut().expect("remove_entity storages -> storage.try_borrow_mut .. flush").flush();
			}
		}

		self.run_systems(systems);

		{
			let storages = self.storages.try_borrow().expect("remove_entity storages try_borrow .. flush_removed");
			for component_id in &set {
				storages[component_id].try_borrow_mut().expect("remove_entity storages -> storage.try_borrow_mut .. flush_removed").flush_removed();
			}
		}
	}

	pub fn is_dead(&self, entity: impl AsEntity) -> bool {
		let entity = entity.as_entity();
		self.entities.try_borrow().expect("is_dead entities.try_borrow").generations[entity.id as usize] != entity.generation
	}

	fn schedule_systems(&self, entities: impl IntoIterator<Item = Entity> + Clone, components: impl IntoIterator<Item = TypeId>) -> Vec<(Entity, Rc<System>)> {
		let systems_interests = self.systems_interests.try_borrow().expect("schedule_systems systems_interests.try_borrow");

		let mut v = vec![];
		for type_id in components.into_iter() {
			if let Some(systems) = systems_interests.get(&type_id) {
				for entity in entities.clone().into_iter() {
					for system in systems.iter() {
						if !system.scheduled.get() && system.query(self, entity) {
							v.push((entity, Rc::clone(&system)));
							system.scheduled.set(true);
						}
					}
				}
			}
		}
		v
	}

	fn run_systems(&self, v: Vec<(Entity, Rc<System>)>) {
		for (entity, system) in v {
			system.scheduled.set(false);
			system.f(entity);
		}
	}
}

#[derive(Default, Debug, shrinkwraprs::Shrinkwrap, Clone, Copy, PartialEq, Eq)]
pub struct Parent(Entity);

#[derive(Default, Debug, shrinkwraprs::Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct Children(pub Vec<Entity>);

impl Parent {
	pub fn ancestors(entity: impl AsEntity) -> Vec<Entity> {
		if let Some(parent) = WORLD.storage::<Parent>().get(entity) {
			let mut v = Self::ancestors(parent.0);
			v.push(parent.0);
			v
		} else {
			Vec::new()
		}
	}
}

impl Children {
	pub fn clear(entity: impl AsEntity) {
		if let Some(mut children) = Children::try_get_mut(entity) {
			let to_remove = children.drain(..).collect::<Vec<_>>();
			drop(children);
			for child in to_remove {
				WORLD.remove_entity(child);
			}
		}
	}
}

#[derive(Default)]
pub struct Classes {
	type_tag: Option<TypeId>,
	styles: HashMap<u64, css::Style>,
}

pub trait Element: AsEntity + Sized {
	fn remove(&self) { WORLD.remove_entity(self) }

	fn add_child(&self, child: impl Element) {
		if WORLD.is_dead(self) { log::warn!("add_child parent dead {:?}", self.as_entity()); return; }
		if WORLD.is_dead(&child) { log::warn!("add_child child dead {:?}", child.as_entity()); return; }
		Children::get_mut_or_default(self).0.push(child.as_entity());
		Parent::get_mut_or_default(&child).0 = self.as_entity();

		if let (Some(parent_node), Some(child_node)) = (web_sys::Node::try_get(self), web_sys::Node::try_get(&child)) {
			parent_node.append_child(&child_node).expect("can't append child");
		}
	}
	fn child(self, child: impl Element) -> Self { self.add_child(child); self }
	fn add_children<Item: Element>(&self, children: impl IntoIterator<Item = Item>) { for child in children.into_iter() { self.add_child(child); } }
	fn children<Item: Element>(self, children: impl IntoIterator<Item = Item>) -> Self { self.add_children(children); self }

	fn set_class_tagged<Tag: std::hash::Hash + 'static>(&self, tag: Tag, style: impl Into<css::Style>) {
		if WORLD.is_dead(self) { log::warn!("set_class_tagged dead {:?}", self.as_entity()); return; }

		// tested and different types with same byte-level representation hash to the same thing (not surprising)
		// i.e. the type is not taken into account when hashing so I have to do it manually
		let tag_hash = {
			use std::hash::{Hash, Hasher};
			let mut hasher = std::collections::hash_map::DefaultHasher::new();
			TypeId::of::<Tag>().hash(&mut hasher);
			tag.hash(&mut hasher);
			hasher.finish()
		};

		Classes::get_mut_or_default(self).styles.insert(tag_hash, style.into());
	}
	fn set_class(&self, style: impl Into<css::Style>) { self.set_class_tagged(0u64, style); }
	fn add_class(&self, style: impl Into<css::Style>) {
		let id = Classes::try_get(self).map(|x| x.styles.len() as u64).unwrap_or(0);
		self.set_class_tagged(id, style);
	}
	fn class(self, style: impl Into<css::Style>) -> Self { self.add_class(style); self }
	fn class_tagged<Tag: std::hash::Hash + 'static>(self, tag: Tag, style: impl Into<css::Style>) -> Self { self.set_class_tagged(tag, style); self }

	fn add_component<T: 'static>(&self, component: T) { T::storage_mut().add(self, component); }
	fn component<T: 'static>(self, component: T) -> Self { self.add_component(component); self }

	fn set_attr<'a>(&self, key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) {
		if WORLD.is_dead(self) { log::warn!("set_attr dead {:?}", self.as_entity()); return; }
		web_sys::Element::get(self).set_attribute(&key.into(), &value.into()).expect("can't set attribute");
	}
	fn attr<'a>(self, key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> Self { self.set_attr(key, value); self }
	fn set_bool_attr<'a>(&self, key: impl Into<Cow<'a, str>>, value: bool) { if value { self.set_attr(key, "") } else { self.remove_attr(key) } }
	fn bool_attr<'a>(self, key: impl Into<Cow<'a, str>>, value: bool) -> Self { self.set_bool_attr(key, value); self }
	fn remove_attr<'a>(&self, key: impl Into<Cow<'a, str>>) {
		if WORLD.is_dead(self) { log::warn!("remove_attr dead {:?}", self.as_entity()); return; }
		web_sys::Element::get(self).remove_attribute(&key.into()).expect("can't remove attribute");
	}

	fn set_text<'a>(&self, text: impl Into<std::borrow::Cow<'a, str>>) {
		if WORLD.is_dead(self) { log::warn!("set_text dead entity {:?}", self.as_entity()); return; }
		web_sys::HtmlElement::get(self).set_inner_text(&text.into());
	}
	fn text<'a>(self, x: impl Into<std::borrow::Cow<'a, str>>) -> Self { self.set_text(x); self }

	fn set_style(&self, style: impl AppendProperty) {
		let mut props = Vec::new();
		style.append_property(&mut props);
		self.set_attr(web_str::style(), props.iter().map(std::string::ToString::to_string).collect::<String>());
	}
	fn style(self, style: impl AppendProperty) -> Self { self.set_style(style); self }
	fn remove_style(&self) { self.remove_attr(web_str::style()); }

	fn mark<T: 'static>(self) -> Self {
		if WORLD.is_dead(&self) { log::warn!("mark dead {:?}", self.as_entity()); return self; }
		Classes::get_mut_or_default(&self).type_tag = Some(TypeId::of::<T>());
		self
	}

	// TODO: this should steal components from other and delete it
	// instead of deleting self
	// this would cause a lot less issue with invalidating stuff
	// !!!!!! NOT TRUE - any handler that was created with the new entity will be busted, so this is fine
	fn replace_with<T: AsEntity>(&self, other: T) -> T {
		let other_entity = other.as_entity();
		if WORLD.is_dead(self) { log::warn!("replace_with dead {:?}", self.as_entity()); return other; }
		if let (Some(this), Some(other)) = (web_sys::Element::try_get(self), web_sys::Node::try_get(other_entity)) {
			this.replace_with_with_node_1(&other).unwrap();
		}

		// Fix up reference in parent
		{
			let storage = Parent::storage();
			if let Some(parent) = storage.get(self).cloned() {
				drop(storage);
				if WORLD.is_dead(parent.0) { log::warn!("replace_with parent dead {:?}", parent.0); return other; }
				let mut children = Children::get_mut(parent.0);
				let position = children.0.iter().position(|&x| x == self.as_entity()).expect("entity claims to be a child while missing in parent");
				children.0[position] = other.as_entity();
				*Parent::get_mut_or_default(other_entity) = parent;
			}
		}

		WORLD.remove_entity(self);
		other
	}

	fn with(self, f: impl FnOnce(&Self)) -> Self { f(&self); self }

	fn erase(&self) -> SomeElement { SomeElement(self.as_entity()) }
}

impl<T: AsEntity> AsEntity for &T {
	fn as_entity(&self) -> Entity { (*self).as_entity() }
}
impl<T: Element> Element for &T { }

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Element)]
pub struct SomeElement(Entity);

pub fn fetch_classname(style: impl Into<css::Style>) -> String {
	STYLE_STORAGE.with(|x| x.borrow_mut().fetch(style.into()))
}

#[extend::ext(pub, name = TypeClassString)]
impl<T: 'static> T {
	fn type_class_string() -> String {
		use std::hash::{Hash, Hasher};
		let mut hasher = std::collections::hash_map::DefaultHasher::new();
		TypeId::of::<Self>().hash(&mut hasher);
		let id = hasher.finish();
		format!("t{}", id)
	}
}

pub trait Component: 'static {
	fn try_get<'a>(entity: impl AsEntity) -> Option<OwningRef<StorageRef<'a, Self>, Self>> where Self: Sized {
		let entity = entity.as_entity();
		let storage = Self::storage();
		if !storage.has(entity) { return None; }
		Some(OwningRef::new(storage).map(|x| x.get(entity).unwrap()))
	}
	fn try_get_mut<'a>(entity: impl AsEntity) -> Option<OwningRefMut<StorageMutRef<'a, Self>, Self>> where Self: Sized {
		let entity = entity.as_entity();
		let storage = Self::storage_mut();
		if !storage.has(entity) { return None; }
		Some(OwningRefMut::new(storage).map_mut(|x| x.get_mut(entity).unwrap()))
	}
	fn get<'a>(entity: impl AsEntity) -> OwningRef<StorageRef<'a, Self>, Self> where Self: Sized {
		OwningRef::new(Self::storage()).map(|x| x.get(entity).unwrap())
	}
	fn get_mut<'a>(entity: impl AsEntity) -> OwningRefMut<StorageMutRef<'a, Self>, Self> where Self: Sized {
		let entity = entity.as_entity();
		OwningRefMut::new(Self::storage_mut()).map_mut(|x| x.get_mut(entity).unwrap())
	}
	fn get_mut_or<'a>(entity: impl AsEntity, f: impl FnOnce() -> Self) -> OwningRefMut<StorageMutRef<'a, Self>, Self> where Self: Sized {
		OwningRefMut::new(Self::storage_mut()).map_mut(move |x| x.get_mut_or(entity, f))
	}
	fn get_mut_or_default<'a>(entity: impl AsEntity) -> OwningRefMut<StorageMutRef<'a, Self>, Self> where Self: Default + Sized { Self::get_mut_or(entity, Self::default) }
	fn storage<'a>() -> StorageRef<'a, Self> where Self: Sized { WORLD.storage::<Self>() }
	fn storage_mut<'a>() -> StorageMutRef<'a, Self> where Self: Sized { WORLD.storage_mut::<Self>() }

	fn register_resource(self) where Self: Sized { World::register_resource(&WORLD, self) }
	fn resource<'a>() -> OwningRef<StorageRef<'a, Self>, Self> where Self: Sized { WORLD.resource::<Self>() }
	fn resource_mut<'a>() -> OwningRefMut<StorageMutRef<'a, Self>, Self> where Self: Sized { WORLD.resource_mut::<Self>() }
	fn try_resource<'a>() -> Option<OwningRef<StorageRef<'a, Self>, Self>> where Self: Sized { WORLD.try_resource::<Self>() }
	fn try_resource_mut<'a>() -> Option<OwningRefMut<StorageMutRef<'a, Self>, Self>> where Self: Sized { WORLD.try_resource_mut::<Self>() }

	// fn register_state(self) where Self: Sized { state::State::new(self).register_resource() }
	// fn state<'a>() -> OwningRef<StorageRef<'a, state::State<Self>>, state::State<Self>> where Self: Sized { <state::State<Self>>::resource() }
	// fn try_state<'a>() -> Option<OwningRef<StorageRef<'a, state::State<Self>>, state::State<Self>>> where Self: Sized { <state::State<Self>>::try_resource() }
}
impl<T: 'static + Sized> Component for T {}
