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
use smart_default::SmartDefault;
use style_storage::STYLE_STORAGE;

fn dom() -> web_sys::Document { web_sys::window().expect("no window").document().expect("no document") }

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Entity(pub u64);

pub struct System {
	f: RefCell<Box<dyn FnMut(Entity) + 'static>>,
	query: fn(&World, Entity) -> bool,
	interests: fn() -> Vec<Interest>,
	scheduled: Cell<bool>,
}

impl System {
	pub fn f(&self, entity: Entity) { (self.f.borrow_mut())(entity) }
	pub fn interests(&self) -> Vec<Interest> { (self.interests)() }
	pub fn query(&self, world: &World, entity: Entity) -> bool { (self.query)(world, entity) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Interest {
	Entity(Entity),
	Component(TypeId),
}

type StorageRc = Rc<RefCell<Box<dyn DynStorage>>>;

// Entity(0) is for holding resources
#[derive(SmartDefault)]
pub struct World {
	#[default(Cell::new(1))]
	pub next_entity_id: Cell<u64>,
	pub storages: RefCell<HashMap<TypeId, StorageRc>>,
	pub dead_entities: RefCell<HashSet<Entity>>,

	// TODO: should keep weak refs and have a separate map with strong refs so systems can be deregistered
	pub systems_interests: RefCell<HashMap<Interest, Vec<Rc<System>>>>,
}

unsafe impl Send for World {}
unsafe impl Sync for World {}

pub static WORLD: Lazy<World> = Lazy::new(|| {
	let world = World::default();

	let sys = <Or<Added<(Classes,)>, Modified<(Classes,)>>>::run(move |entity| {
		if let Some(element) = WORLD.storage::<web_sys::Element>().get(entity) {
			use std::fmt::Write;

			let storage = WORLD.storage::<Classes>();
			let classes = storage.get(entity).unwrap();
			let mut res = format!("e{}", entity.0);

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

	world.new_system(sys.interests(), sys);

	create::register_systems(&world);

	world
});

type StorageRef<'a, Component> = OwningRef<OwningHandle<Rc<RefCell<Box<(dyn storage::DynStorage + 'static)>>>, Ref<'a, Box<dyn storage::DynStorage>>>, SimpleStorage<Component>>;
type StorageMutRef<'a, Component> = StorageGuard<'a, Component, OwningRefMut<OwningHandle<Rc<RefCell<Box<(dyn storage::DynStorage + 'static)>>>, RefMut<'a, Box<dyn storage::DynStorage>>>, SimpleStorage<Component>>>;

impl World {
	fn assure_storage<Component: 'static>(&self) {
		self.storages.try_borrow_mut().expect("trying to borrow_mut storages to assure that one exists")
			.entry(TypeId::of::<Component>())
			.or_insert_with(|| Rc::new(RefCell::new(Box::new(SimpleStorage::<Component>::default()))));
	}

	// pub fn storage_mut<Component: 'static>(&self) -> StorageGuard<'_, Component, impl std::ops::DerefMut<Target = SimpleStorage<Component>> + owning_ref::StableAddress + '_> {
	pub fn storage_mut<Component: 'static>(&self) -> StorageMutRef<Component> {
		self.assure_storage::<Component>();

		let storage_cell = OwningRefMut::new(OwningHandle::new_mut(
			self.storages.try_borrow().expect("trying to borrow storages to get a mut storage")
				.get(&TypeId::of::<Component>()).unwrap().clone()
		));
		let storage = storage_cell.map_mut(|x| x.as_any_mut().downcast_mut::<SimpleStorage<Component>>().unwrap());
		StorageGuard(self, Some(storage))
	}

	// pub fn storage<Component: 'static>(&self) -> impl std::ops::Deref<Target = SimpleStorage<Component>> + owning_ref::StableAddress + '_ {
	pub fn storage<Component: 'static>(&self) -> StorageRef<Component> {
		self.assure_storage::<Component>();

		let storage_cell = OwningRef::new(OwningHandle::new(
			self.storages.try_borrow().expect("trying to borrow storages to get a storage")
				.get(&TypeId::of::<Component>()).unwrap().clone()
		));
		storage_cell.map(|x| x.as_any().downcast_ref::<SimpleStorage<Component>>().unwrap())
	}

	pub fn register_resource<T: 'static>(&self, resource: T) { self.storage_mut::<T>().add(Entity(0), resource); }

	pub fn resource<T: 'static>(&self) -> OwningRef<StorageRef<T>, T> {
		OwningRef::new(self.storage::<T>()).map(|x| x.get(Entity(0)).unwrap())
	}

	pub fn resource_mut<T: 'static>(&self) -> OwningRefMut<StorageMutRef<T>, T> {
		OwningRefMut::new(self.storage_mut::<T>()).map_mut(|x| x.get_mut(Entity(0)).unwrap())
	}

	pub fn try_resource<T: 'static>(&self) -> Option<OwningRef<StorageRef<T>, T>> {
		if !self.storage::<T>().has(Entity(0)) { return None; }
		Some(OwningRef::new(self.storage::<T>()).map(|x| x.get(Entity(0)).unwrap()))
	}

	pub fn try_resource_mut<T: 'static>(&self) -> Option<OwningRefMut<StorageMutRef<T>, T>> {
		if !self.storage::<T>().has(Entity(0)) { return None; }
		Some(OwningRefMut::new(self.storage_mut::<T>()).map_mut(|x| x.get_mut(Entity(0)).unwrap()))
	}

	pub fn new_entity(&self) -> Entity {
		let entity = Entity(self.next_entity_id.get());
		self.next_entity_id.set(self.next_entity_id.get() + 1);
		entity
	}

	pub fn new_system(&self, interests: impl IntoIterator<Item = Interest>, sys: System) {
		let sys = Rc::new(sys);
		for interest in interests.into_iter() {
			let mut systems_interests = self.systems_interests.try_borrow_mut().expect("trying to borrow_mut systems_interests to add a new system");
			systems_interests.entry(interest).or_insert_with(Vec::new).push(Rc::clone(&sys));
		}
	}

	pub fn watch_resource(&self, sys: System) {
		self.new_system(vec![Interest::Entity(Entity(0))], sys);
	}

	pub fn remove_entity(&self, entity: Entity) {
		self.dead_entities.try_borrow_mut().expect("trying to borrow_mut dead_entities to remove one").insert(entity);

		if let Some(children) = Children::try_get(entity).map(|x| x.0.clone()) {
			for child in children {
				self.remove_entity(child);
			}
		}

		let mut set: HashSet<TypeId> = HashSet::new();

		for (&component_id, storage) in self.storages.try_borrow().expect("trying to borrow storages to remove an entity").iter() {
			if storage.try_borrow().expect("trying to borrow a storage to check if it has an entity to remove").has(entity) {
				set.insert(component_id);
				storage.try_borrow_mut().expect("trying to borrow_mut storage to remove an entity").remove(entity);
			}
		}

		let systems = self.schedule_systems(set.iter().map(|&component_id| (entity, component_id)));

		{
			let storages = self.storages.try_borrow().expect("trying to borrow storages to flush one after removing an entity");
			for component_id in &set {
				storages[component_id].try_borrow_mut().expect("trying to borrow_mut a storage to flush after removing an entity").flush();
			}
		}

		self.run_systems(systems);

		{
			let storages = self.storages.try_borrow().expect("trying to borrow storages to flush removed one after removing an entity");
			for component_id in &set {
				storages[component_id].try_borrow_mut().expect("trying to borrow_mut a storage to flush removed after removing an entity").flush_removed();
			}
		}

		self.systems_interests.try_borrow_mut().expect("trying to borrow systems_interests to get rid of an entity interest").remove(&Interest::Entity(entity));
	}

	pub fn is_dead(&self, entity: Entity) -> bool {
		self.dead_entities.borrow().contains(&entity)
	}

	fn schedule_systems(&self, interests: impl IntoIterator<Item = (Entity, TypeId)>) -> Vec<(Entity, Rc<System>)> {
		let systems_interests = self.systems_interests.try_borrow().expect("trying to borrow systems_interests to schedule");

		let mut v = vec![];
		for (entity, type_id) in interests.into_iter() {
			if let Some(systems) = systems_interests.get(&Interest::Entity(entity)) {
				for system in systems.iter() {
					if !system.scheduled.get() && system.query(self, entity) {
						v.push((entity, Rc::clone(&system)));
						system.scheduled.set(true);
					}
				}
			}

			if let Some(systems) = systems_interests.get(&Interest::Component(type_id)) {
				for system in systems.iter() {
					if !system.scheduled.get() && system.query(self, entity) {
						v.push((entity, Rc::clone(&system)));
						system.scheduled.set(true);
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

#[derive(Default)]
pub struct Parent(Entity);

#[derive(Default)]
pub struct Children(Vec<Entity>);

impl Parent {
	pub fn ancestors(entity: Entity) -> Vec<Entity> {
		if let Some(parent) = WORLD.storage::<Parent>().get(entity) {
			let mut v = Self::ancestors(parent.0);
			v.push(parent.0);
			v
		} else {
			Vec::new()
		}
	}
}

impl std::ops::Deref for Parent {
	type Target = Entity;

	fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Default)]
pub struct Classes {
	type_tag: Option<TypeId>,
	styles: HashMap<u64, css::Style>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Element {
	entity: Entity,
}

impl Element {
	pub fn entity(&self) -> Entity { self.entity }
	pub fn remove(self) { WORLD.remove_entity(self.entity) }
	pub fn new(entity: Entity) -> Self { Self { entity } }

	pub fn add_child(self, child: impl Into<Element>) {
		let child = child.into().entity();
		if WORLD.is_dead(self.entity) || WORLD.is_dead(child) { return; }
		WORLD.storage_mut::<Children>().get_mut_or_default(self.entity).0.push(child);
		WORLD.storage_mut::<Parent>().get_mut_or_default(child).0 = self.entity;

		let storage = WORLD.storage::<web_sys::Node>();
		if let (Some(parent_node), Some(child_node)) = (storage.get(self.entity), storage.get(child)) {
			parent_node.append_child(child_node).expect("can't append child");
		}
	}
	pub fn child(self, child: impl Into<Element>) -> Self { self.add_child(child); self }
	pub fn add_children(self, children: impl IntoIterator<Item = Element>) {
		for child in children.into_iter() {
			self.add_child(child);
		}
	}
	pub fn children(self, children: impl IntoIterator<Item = Element>) -> Self { self.add_children(children); self }

	pub fn set_class_tagged<'a, Tag: std::hash::Hash + 'static>(self, tag: Tag, style: impl Into<css::Style>) {
		if WORLD.is_dead(self.entity) { return; }
		let mut storage = WORLD.storage_mut::<Classes>();
		let classes = storage.get_mut_or_default(self.entity);

		// tested and different types with same byte-level representation hash to the same thing (not surprising)
		// i.e. the type is not taken into account when hashing so I have to do it manually
		let tag_hash = {
			use std::hash::{Hash, Hasher};
			let mut hasher = std::collections::hash_map::DefaultHasher::new();
			std::any::TypeId::of::<Tag>().hash(&mut hasher);
			tag.hash(&mut hasher);
			hasher.finish()
		};

		classes.styles.insert(tag_hash, style.into());
	}
	pub fn set_class(self, style: impl Into<css::Style>) { self.set_class_tagged(0u64, style); }
	pub fn add_class(self, style: impl Into<css::Style>) {
		let id = WORLD.storage::<Classes>().get(self.entity).map(|x| x.styles.len() as u64).unwrap_or(0);
		self.set_class_tagged(id, style);
	}
	pub fn class(self, style: impl Into<css::Style>) -> Self { self.add_class(style); self }

	pub fn add_component<T: 'static>(self, component: T) { WORLD.storage_mut::<T>().add(self.entity, component); }
	pub fn component<T: 'static>(self, component: T) -> Self { self.add_component(component); self }

	pub fn add_system(self, system: System) {
		WORLD.new_system(vec![Interest::Entity(self.entity)], system);
	}
	pub fn system(self, system: System) -> Self { self.add_system(system); self }

	pub fn set_attr<'a>(self, key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) {
		if WORLD.is_dead(self.entity) { return; }
		WORLD.storage::<web_sys::Element>().get(self.entity).expect("missing web_sys::Element").set_attribute(&key.into(), &value.into()).expect("can't set attribute");
	}
	pub fn attr<'a>(self, key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> Self { self.set_attr(key, value); self }
	pub fn set_bool_attr<'a>(self, key: impl Into<Cow<'a, str>>, value: bool) { if value { self.set_attr(key, "") } else { self.remove_attr(key) } }
	pub fn bool_attr<'a>(self, key: impl Into<Cow<'a, str>>, value: bool) -> Self { self.set_bool_attr(key, value); self }
	pub fn remove_attr<'a>(self, key: impl Into<Cow<'a, str>>) {
		if WORLD.is_dead(self.entity) { return; }
		WORLD.storage::<web_sys::Element>().get(self.entity).expect("missing web_sys::Element").remove_attribute(&key.into()).expect("can't remove attribute");
	}

	pub fn set_text<'a>(self, text: impl Into<std::borrow::Cow<'a, str>>) {
		if WORLD.is_dead(self.entity) { return; }
		if let Some(x) = WORLD.storage::<web_sys::HtmlElement>().get(self.entity) {
			x.set_inner_text(&text.into());
		}
	}
	pub fn text<'a>(self, x: impl Into<std::borrow::Cow<'a, str>>) -> Self { self.set_text(x); self }

	pub fn set_style<'a>(self, style: impl Into<Cow<'a, [css::Property]>>) { self.set_attr(web_str::style(), style.into().iter().map(std::string::ToString::to_string).collect::<String>()); }
	pub fn style<'a>(self, style: impl Into<Cow<'a, [css::Property]>>) -> Self { self.set_style(style); self }
	pub fn remove_style(self) { self.remove_attr(web_str::style()); }

	pub fn mark<T: 'static>(self) -> Self {
		if WORLD.is_dead(self.entity) { return self; }
		let mut storage = WORLD.storage_mut::<Classes>();
		let classes = storage.get_mut_or_default(self.entity);
		classes.type_tag = Some(TypeId::of::<T>());
		self
	}

	// TODO: this should steal components from other and delete it
	// instead of deleting self
	// this would cause a lot less issue with invalidating stuff
	// !!!!!! NOT TRUE - any handler that was created with the new entity will be busted, so this is fine
	pub fn replace_with(&mut self, other: Self) {
		if let (Some(this), Some(other)) = (web_sys::Element::try_get(self.entity()), web_sys::Node::try_get(other.entity())) {
			this.replace_with_with_node_1(&other).unwrap();
		}

		// Fix up reference in parent
		if let Some(parent) = Parent::try_get(self.entity()) {
			let mut children = Children::try_get_mut(parent.0).expect("Parent without Children");
			let position = children.0.iter().position(|&x| x == self.entity()).expect("entity claims to be a child while missing in parent");
			children.0[position] = other.entity();
		}

		WORLD.remove_entity(self.entity());
		*self = other;
	}

	pub fn with(self, f: impl FnOnce(Self)) -> Self { f(self); self }
}

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
	fn try_get<'a>(entity: Entity) -> Option<OwningRef<StorageRef<'a, Self>, Self>> where Self: Sized {
		let storage = Self::storage();
		if !storage.has(entity) { return None; }
		Some(OwningRef::new(storage).map(|x| x.get(entity).unwrap()))
	}
	fn try_get_mut<'a>(entity: Entity) -> Option<OwningRefMut<StorageMutRef<'a, Self>, Self>> where Self: Sized {
		let storage = Self::storage_mut();
		if !storage.has(entity) { return None; }
		Some(OwningRefMut::new(storage).map_mut(|x| x.get_mut(entity).unwrap()))
	}
	fn get<'a>(entity: Entity) -> OwningRef<StorageRef<'a, Self>, Self> where Self: Sized {
		OwningRef::new(Self::storage()).map(|x| x.get(entity).unwrap())
	}
	fn get_mut<'a>(entity: Entity) -> OwningRefMut<StorageMutRef<'a, Self>, Self> where Self: Sized {
		OwningRefMut::new(Self::storage_mut()).map_mut(|x| x.get_mut(entity).unwrap())
	}
	fn get_mut_or<'a>(entity: Entity, f: impl FnOnce() -> Self) -> OwningRefMut<StorageMutRef<'a, Self>, Self> where Self: Sized {
		OwningRefMut::new(Self::storage_mut()).map_mut(move |x| x.get_mut_or(entity, f))
	}
	fn get_mut_or_default<'a>(entity: Entity) -> OwningRefMut<StorageMutRef<'a, Self>, Self> where Self: Default + Sized { Self::get_mut_or(entity, Self::default) }
	fn storage<'a>() -> StorageRef<'a, Self> where Self: Sized { WORLD.storage::<Self>() }
	fn storage_mut<'a>() -> StorageMutRef<'a, Self> where Self: Sized { WORLD.storage_mut::<Self>() }
	fn register_resource(self) where Self: Sized { World::register_resource(&WORLD, self) }
	fn resource<'a>() -> OwningRef<StorageRef<'a, Self>, Self> where Self: Sized { WORLD.resource::<Self>() }
	fn resource_mut<'a>() -> OwningRefMut<StorageMutRef<'a, Self>, Self> where Self: Sized { WORLD.resource_mut::<Self>() }
	fn try_resource<'a>() -> Option<OwningRef<StorageRef<'a, Self>, Self>> where Self: Sized { WORLD.try_resource::<Self>() }
	fn try_resource_mut<'a>() -> Option<OwningRefMut<StorageMutRef<'a, Self>, Self>> where Self: Sized { WORLD.try_resource_mut::<Self>() }
}
impl<T: 'static + Sized> Component for T {}
