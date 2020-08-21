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
use std::any::{Any, TypeId};
use std::rc::Rc;
use once_cell::sync::Lazy;
use std::cell::{RefCell, Cell};
use std::marker::PhantomData;
use storage::*;
use owning_ref::{OwningRef, OwningRefMut, OwningHandle};
use std::borrow::Cow;

fn dom() -> web_sys::Document { web_sys::window().expect("no window").document().expect("no document") }

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Entity(u64);

pub struct System {
	f: Box<dyn Fn(Entity) + 'static>,
	query: fn(&World, Entity) -> bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Interest {
	Entity(Entity),
	Component(TypeId),
}

// systems register entities they care about upon creation
// identical systems have their entities merged
#[derive(Default)]
pub struct World {
	pub next_entity_id: Cell<u64>,
	pub storages: RefCell<HashMap<TypeId, Rc<RefCell<Box<dyn DynStorage>>>>>,
	pub resources: RefCell<HashMap<TypeId, RefCell<Box<dyn Any>>>>,
	pub systems_interests: RefCell<HashMap<Interest, Vec<Rc<System>>>>,
	pub dead_entities: RefCell<HashSet<Entity>>,
}

unsafe impl Send for World {}
unsafe impl Sync for World {}

pub static WORLD: Lazy<World> = Lazy::new(|| {
	let world = World::default();
	world.register_resource(crate::style_storage::StyleStorage::default());
	world
});

impl World {
	fn assure_storage<Component: 'static>(&self) {
		self.storages.borrow_mut().entry(TypeId::of::<SimpleStorage<Component>>()).or_insert_with(|| Rc::new(RefCell::new(Box::new(SimpleStorage::<Component>::default()))));
	}

	pub fn storage_mut<Component: 'static>(&self) -> StorageGuard<'_, Component, impl std::ops::DerefMut<Target = SimpleStorage<Component>> + '_> {
		self.assure_storage::<Component>();

		let storage_cell = OwningRefMut::new(OwningHandle::new_mut(self.storages.borrow().get(&TypeId::of::<SimpleStorage<Component>>()).unwrap().clone()));
		let storage = storage_cell.map_mut(|x| x.as_any_mut().downcast_mut::<SimpleStorage<Component>>().unwrap());
		StorageGuard(self, Some(storage))
	}

	pub fn storage<Component: 'static>(&self) -> impl std::ops::Deref<Target = SimpleStorage<Component>> + '_ {
		self.assure_storage::<Component>();

		let storage_cell = OwningRef::new(OwningHandle::new(self.storages.borrow().get(&TypeId::of::<SimpleStorage<Component>>()).unwrap().clone()));
		storage_cell.map(|x| x.as_any().downcast_ref::<SimpleStorage<Component>>().unwrap())
	}

	pub fn register_resource<T: 'static>(&self, resource: T) {
		self.resources.borrow_mut().entry(TypeId::of::<T>()).or_insert_with(|| RefCell::new(Box::new(resource)));
	}

	pub fn resource<T: 'static>(&self) -> Option<impl std::ops::Deref<Target = T> + '_> {
		let resources = OwningRef::new(self.resources.borrow());
		resources.get(&TypeId::of::<T>())?;
		let resource_cell = OwningRef::new(OwningHandle::new(resources.map(|x| x.get(&TypeId::of::<T>()).unwrap())));
		Some(resource_cell.map(|x| x.downcast_ref::<T>().unwrap()))
	}

	pub fn resource_mut<T: 'static>(&self) -> Option<impl std::ops::DerefMut<Target = T> + '_> {
		let resources = OwningRef::new(self.resources.borrow());
		resources.get(&TypeId::of::<T>())?;
		let resource_cell = OwningRefMut::new(OwningHandle::new_mut(resources.map(|x| x.get(&TypeId::of::<T>()).unwrap())));
		Some(resource_cell.map_mut(|x| x.downcast_mut::<T>().unwrap()))
	}

	pub fn new_entity(&self) -> Entity {
		let entity = Entity(self.next_entity_id.get());
		self.next_entity_id.set(self.next_entity_id.get() + 1);
		entity
	}

	pub fn new_system(&self, sys: System, interests: impl IntoIterator<Item = Interest>) {
		let sys = Rc::new(sys);
		for interest in interests.into_iter() {
			let mut systems_interests = self.systems_interests.borrow_mut();
			systems_interests.entry(interest).or_insert_with(Vec::new).push(Rc::clone(&sys));
		}
	}

	pub fn remove_entity(&self, entity: Entity) {
		self.dead_entities.borrow_mut().insert(entity);
		for storage in self.storages.borrow().values() {
			storage.borrow_mut().remove(entity);
		}

		let systems = self.schedule_systems(vec![entity]);

		for storage in self.storages.borrow().values() {
			storage.borrow_mut().flush();
		}

		self.run_systems(systems);

		self.systems_interests.borrow_mut().remove(&entity);
	}

	pub fn is_dead(&self, entity: Entity) -> bool {
		self.dead_entities.borrow().contains(&entity)
	}

	fn schedule_systems(&self, interests: impl IntoIterator<Item = (Entity, TypeId)>) -> Vec<(Entity, Rc<System>)> {
		let systems_interests = self.systems_interests.borrow();

		let mut v = vec![];
		for (entity, type_id) in interests.into_iter() {
			if let Some(systems) = systems_interests.get(&Interest::Entity(entity)) {
				for system in systems.iter() {
					if (system.query)(self, entity) {
						v.push((entity, Rc::clone(&system)));
					}
				}
			}

			if let Some(systems) = systems_interests.get(&Interest::Component(type_id)) {
				for system in systems.iter() {
					if (system.query)(self, entity) {
						v.push((entity, Rc::clone(&system)));
					}
				}
			}
		}
		v
	}

	fn run_systems(&self, v: Vec<(Entity, Rc<System>)>) {
		for (entity, system) in v {
			(system.f)(entity);
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

	pub fn add_child(self, child: Element) {
		if WORLD.is_dead(self.entity) || WORLD.is_dead(child.entity) { return; }
		WORLD.storage_mut::<Children>().get_mut_or(self.entity, Children::default).0.push(child.entity);
		WORLD.storage_mut::<Parent>().get_mut_or(child.entity, Parent::default).0 = self.entity;

		let storage = WORLD.storage::<web_sys::Node>();
		if let (Some(parent_node), Some(child_node)) = (storage.get(self.entity), storage.get(child.entity)) {
			parent_node.append_child(child_node).expect("can't append child");
		}
	}
	pub fn child(self, child: Element) -> Self { self.add_child(child); self }

	pub fn set_class_tagged<'a, Tag: std::hash::Hash + 'static>(self, tag: Tag, style: impl Into<Cow<'a, css::Style>>) {
		if WORLD.is_dead(self.entity) { return; }
		let mut storage = WORLD.storage_mut::<Classes>();
		let classes = storage.get_mut_or(self.entity, Classes::default);

		// tested and different types with same byte-level representation hash to the same thing (not surprising)
		// i.e. the type is not taken into account when hashing so I have to do it manually
		let tag_hash = {
			use std::hash::{Hash, Hasher};
			let mut hasher = std::collections::hash_map::DefaultHasher::new();
			std::any::TypeId::of::<Tag>().hash(&mut hasher);
			tag.hash(&mut hasher);
			hasher.finish()
		};

		classes.styles.insert(tag_hash, style.into().into_owned());
	}
	pub fn set_class<'a>(self, style: impl Into<Cow<'a, css::Style>>) { self.set_class_tagged(0u64, style); }
	pub fn add_class<'a>(self, style: impl Into<Cow<'a, css::Style>>) {
		let id = WORLD.storage::<Classes>().get(self.entity).map(|x| x.styles.len() as u64).unwrap_or(0);
		self.set_class_tagged(id, style);
	}
	pub fn class<'a>(self, style: impl Into<Cow<'a, css::Style>>) -> Self { self.add_class(style); self }

	pub fn add_component<T: 'static>(self, component: T) { WORLD.storage_mut::<T>().add(self.entity, component); }
	pub fn component<T: 'static>(self, component: T) -> Self { self.add_component(component); self }

	pub fn add_system(self, system: System) {
		// TODO:
		// WORLD.new_system(system, vec![self.entity]);
	}
	pub fn system(self, system: System) -> Self { self.add_system(system); self }

	pub fn set_attr<'a>(self, key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) {
		if WORLD.is_dead(self.entity) { return; }
		WORLD.storage::<web_sys::Element>().get(self.entity).expect("missing web_sys::Element").set_attribute(&key.into(), &value.into()).expect("can't set attribute");
	}
	pub fn attr<'a>(self, key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> Self { self.set_attr(key, value); self }
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
		let classes = storage.get_mut_or(self.entity, Classes::default);
		classes.type_tag = Some(TypeId::of::<T>());
		self
	}
}

pub fn fetch_classname<'a>(style: impl Into<Cow<'a, css::Style>>) -> String {
	let mut style_storage = WORLD.resource_mut::<crate::style_storage::StyleStorage>().unwrap();
	style_storage.fetch(style.into().into_owned())
}

pub fn default_systems(entity: Entity) {
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

			let mut style_storage = WORLD.resource_mut::<crate::style_storage::StyleStorage>().unwrap();
			for style in classes.styles.values() {
				write!(&mut res, " {}", style_storage.fetch(style.clone())).unwrap();
			}

			element.set_attribute(web_str::class(), &res).expect("can't set class attribute");
		}
	});
	// TODO:
	// WORLD.new_system(sys, vec![entity]);
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
