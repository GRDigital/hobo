pub mod prelude;
pub mod web_str;
mod style_storage;
mod enclose;
mod query;
mod storage;
pub mod state;
pub mod create;
mod dom_events;
pub mod events;
mod element;

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
use style_storage::STYLE_STORAGE;
pub use element::{Element, Classes, Parent, Children, SomeElement};

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
	#[inline] fn try_get_cmp<'a, C: 'static>(&self) -> Option<OwningRef<StorageRef<'a, C>, C>> where Self: Sized {
		let entity = self.as_entity();
		let storage = WORLD.storage::<C>();
		if !storage.has(entity) { return None; }
		Some(OwningRef::new(storage).map(|x| x.get(entity).unwrap()))
	}
	#[inline] fn try_get_cmp_mut<'a, C: 'static>(&self) -> Option<OwningRefMut<StorageMutRef<'a, C>, C>> where Self: Sized {
		let entity = self.as_entity();
		if !WORLD.storage::<C>().has(entity) { return None; }
		Some(OwningRefMut::new(WORLD.storage_mut::<C>()).map_mut(|x| x.get_mut(entity).unwrap()))
	}
	#[inline] fn get_cmp<'a, C: 'static>(&self) -> OwningRef<StorageRef<'a, C>, C> where Self: Sized {
		OwningRef::new(WORLD.storage::<C>()).map(|x| x.get(self).unwrap())
	}
	#[inline] fn get_cmp_mut<'a, C: 'static>(&self) -> OwningRefMut<StorageMutRef<'a, C>, C> where Self: Sized {
		OwningRefMut::new(WORLD.storage_mut::<C>()).map_mut(|x| x.get_mut(self).unwrap())
	}
	#[inline] fn get_cmp_mut_or<'a, C: 'static>(&self, f: impl FnOnce() -> C) -> OwningRefMut<StorageMutRef<'a, C>, C> where Self: Sized {
		OwningRefMut::new(WORLD.storage_mut::<C>()).map_mut(move |x| x.get_mut_or(self, f))
	}
	#[inline] fn get_cmp_mut_or_default<'a, C: 'static + Default>(&self) -> OwningRefMut<StorageMutRef<'a, C>, C> where Self: Sized {
		self.get_cmp_mut_or(Default::default)
	}
	#[inline] fn get_cmp_from_ancestors<'a, C: 'static>(&self) -> OwningRef<StorageRef<'a, C>, C> where Self: Sized {
		Parent::ancestor_with_cmp::<C>(self.as_entity()).get_cmp::<C>()
	}
	#[inline] fn get_cmp_mut_from_ancestors<'a, C: 'static>(&self) -> OwningRefMut<StorageMutRef<'a, C>, C> where Self: Sized {
		Parent::ancestor_with_cmp::<C>(self.as_entity()).get_cmp_mut::<C>()
	}

	fn remove(&self) { WORLD.remove_entity(self.as_entity()) }
	fn is_dead(&self)  -> bool { WORLD.is_dead(self.as_entity()) }
	fn add_component<T: 'static>(&self, component: T) { WORLD.storage_mut::<T>().add(self.as_entity(), component); }
	fn component<T: 'static>(self, component: T) -> Self where Self: Sized { self.add_component(component); self }
}

impl AsEntity for Entity {
	fn as_entity(&self) -> Entity { *self }
}

pub struct System {
	f: Box<dyn Fn(Entity) + 'static>,
	query: fn(&World, Entity) -> bool,
	interests: fn() -> HashSet<TypeId>,
}

impl System {
	pub fn f(&self, entity: Entity) { (self.f)(entity) }
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
		free_ids: default(),
		// Entity(0, 0) is a fake entity for holding resources
		generations: vec![0],
	} }
}

#[derive(Default)]
pub struct World {
	storages: RefCell<HashMap<TypeId, StorageRc>>,
	component_ownership: RefCell<HashMap<Entity, HashSet<TypeId>>>,
	entities: RefCell<Entities>,

	// TODO: should keep weak refs and have a separate map with strong refs so systems can be deregistered
	systems_interests: RefCell<HashMap<TypeId, Vec<Rc<System>>>>,
	system_update_lock: Cell<bool>,
}

// only safe until threading becomes a thing
unsafe impl Send for World {}
unsafe impl Sync for World {}

pub static WORLD: Lazy<World> = Lazy::new(|| {
	let world = World::default();

	let sys = <(Or<Added<(Classes,)>, Modified<(Classes,)>>, Present<(web_sys::Element,)>)>::run(move |entity| {
		use std::fmt::Write;

		let classes = entity.get_cmp::<Classes>();
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

		entity.get_cmp::<web_sys::Element>().set_attribute(web_str::class(), &res).expect("can't set class attribute");
	});

	world.new_system(sys);

	create::register_systems(&world);

	world
});

// this is not necessary, but it makes it convenient to further remap to some OwningRef or whatever
type StorageRef<'a, Component> = OwningRef<OwningHandle<Rc<RefCell<Box<(dyn storage::DynStorage + 'static)>>>, Ref<'a, Box<dyn storage::DynStorage>>>, SimpleStorage<Component>>;
type StorageMutRef<'a, Component> = StorageGuard<'a, Component, OwningRefMut<OwningHandle<Rc<RefCell<Box<(dyn storage::DynStorage + 'static)>>>, RefMut<'a, Box<dyn storage::DynStorage>>>, SimpleStorage<Component>>>;

impl World {
	fn dyn_storage<Component: 'static>(&self) -> Rc<RefCell<Box<dyn DynStorage>>> {
		Rc::clone(self.storages.try_borrow_mut().unwrap_or_else(|e| panic!("{}: {} dyn_storage -> storages.try_borrow_mut", e, std::any::type_name::<Component>()))
			.entry(TypeId::of::<Component>())
			.or_insert_with(|| Rc::new(RefCell::new(Box::new(SimpleStorage::<Component>::default())))))
	}

	pub fn storage_mut<Component: 'static>(&self) -> StorageMutRef<Component> {
		let storage = OwningRefMut::new(OwningHandle::new_mut(self.dyn_storage::<Component>()))
			.map_mut(|x| x.as_any_mut().downcast_mut().unwrap());
		StorageGuard(self, Some(storage))
	}

	pub fn storage<Component: 'static>(&self) -> StorageRef<Component> {
		OwningRef::new(OwningHandle::new(self.dyn_storage::<Component>()))
			.map(|x| x.as_any().downcast_ref().unwrap())
	}

	pub fn register_resource<T: 'static>(&self, resource: T) { self.storage_mut().add(Entity::root(), resource); }

	// resources are just components attached to Entity(0, 0)
	pub fn resource<T: 'static>(&self) -> OwningRef<StorageRef<T>, T> {
		OwningRef::new(self.storage()).map(|x| x.get(Entity::root()).unwrap())
	}

	pub fn resource_mut<T: 'static>(&self) -> OwningRefMut<StorageMutRef<T>, T> {
		OwningRefMut::new(self.storage_mut()).map_mut(|x| x.get_mut(Entity::root()).unwrap())
	}

	pub fn try_resource<T: 'static>(&self) -> Option<OwningRef<StorageRef<T>, T>> {
		if !self.storage::<T>().has(Entity::root()) { return None; }
		Some(OwningRef::new(self.storage()).map(|x| x.get(Entity::root()).unwrap()))
	}

	pub fn try_resource_mut<T: 'static>(&self) -> Option<OwningRefMut<StorageMutRef<T>, T>> {
		if !self.storage::<T>().has(Entity::root()) { return None; }
		Some(OwningRefMut::new(self.storage_mut()).map_mut(|x| x.get_mut(Entity::root()).unwrap()))
	}

	// if there's a removed entity, take that one
	// otherwise the next id would be the one with unassigned generation,
	// i.e. index of last generation + 1
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
		for interest in sys.interests() {
			let mut systems_interests = self.systems_interests.try_borrow_mut().expect("new_system systems_interests.try_borrrow_mut");
			systems_interests.entry(interest).or_insert_with(Vec::new).push(Rc::clone(&sys));
		}
	}

	// TODO: add check systems in entity components
	pub fn remove_entity(&self, entity: impl AsEntity) {
		let entity = entity.as_entity();
		if WORLD.is_dead(entity) { log::warn!("remove entity already dead {:?}", entity); return; }

		if let Some(children) = entity.try_get_cmp::<Children>().map(|x| x.0.clone()) {
			for child in children { self.remove_entity(child); }
		}

		{
			let mut entities = self.entities.try_borrow_mut().expect("remove_entity entities.try_borrow_mut");
			entities.free_ids.push(entity.id);
			entities.generations[entity.id as usize] += 1;
		}

		if let Some(parent) = entity.try_get_cmp::<Parent>() {
			let mut children = parent.get_cmp_mut::<Children>();
			if let Some(child_pos) = children.0.iter().position(|&x| x == entity) { children.0.remove(child_pos); }
		}

		// TODO: just take component_ids
		let mut set: HashSet<TypeId> = HashSet::new();

		if let Some(component_ids) = self.component_ownership.borrow_mut().remove(&entity) {
			let storages = self.storages.try_borrow().expect("remove_entity storages.try_borrow .. remove");
			for component_id in component_ids {
				set.insert(component_id);
				storages[&component_id].try_borrow_mut().expect("remove_entity storages -> storage.try_borrow_mut .. remove").dyn_remove(entity);
			}
		}

		self.run_systems(std::iter::once(entity), set);
	}

	pub fn is_dead(&self, entity: impl AsEntity) -> bool {
		let entity = entity.as_entity();
		self.entities.try_borrow().expect("is_dead entities.try_borrow").generations[entity.id as usize] != entity.generation
	}

	fn run_systems(&self, entities: impl IntoIterator<Item = Entity> + Clone, components: impl IntoIterator<Item = TypeId> + Clone) {
		let topmost_update = if self.system_update_lock.get() { false } else { self.system_update_lock.set(true); true };

		let to_run = {
			let mut v = Vec::new();
			let systems_interests = self.systems_interests.try_borrow().expect("schedule_systems systems_interests.try_borrow");
			for type_id in components.clone() {
				if let Some(systems) = systems_interests.get(&type_id) {
					for entity in entities.clone() {
						for system in systems.iter() {
							v.push((entity, Rc::clone(&system)));
						}
					}
				}
			}
			v
		};

		for (entity, system) in to_run {
			if system.query(self, entity) {
				system.f(entity);
			}
		}

		if topmost_update {
			let storages = self.storages.try_borrow().expect("remove_entity storages.try_borrow .. flush");
			for component_id in components {
				let mut storage = storages[&component_id].try_borrow_mut().expect("remove_entity storages -> storage.try_borrow_mut .. flush_removed");
				storage.flush();
				storage.flush_removed();
			}
			self.system_update_lock.set(false);
		}
	}
}

impl<T: AsEntity> AsEntity for &T {
	fn as_entity(&self) -> Entity { T::as_entity(*self) }
}
impl<T: AsEntity> AsEntity for &mut T {
	fn as_entity(&self) -> Entity { T::as_entity(*self) }
}

pub fn fetch_classname(style: impl Into<css::Style>) -> String {
	STYLE_STORAGE.with(|x| x.borrow_mut().fetch(style.into()))
}

pub fn register_window(window: &web_sys::Window) {
	STYLE_STORAGE.with(|x| x.borrow_mut().register_window(window));
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
	#[inline] fn storage<'a>() -> StorageRef<'a, Self> where Self: Sized { WORLD.storage::<Self>() }
	#[inline] fn storage_mut<'a>() -> StorageMutRef<'a, Self> where Self: Sized { WORLD.storage_mut::<Self>() }

	#[inline] fn register_resource(self) where Self: Sized { World::register_resource(&WORLD, self) }
	#[inline] fn resource<'a>() -> OwningRef<StorageRef<'a, Self>, Self> where Self: Sized { WORLD.resource::<Self>() }
	#[inline] fn resource_mut<'a>() -> OwningRefMut<StorageMutRef<'a, Self>, Self> where Self: Sized { WORLD.resource_mut::<Self>() }
	#[inline] fn try_resource<'a>() -> Option<OwningRef<StorageRef<'a, Self>, Self>> where Self: Sized { WORLD.try_resource::<Self>() }
	#[inline] fn try_resource_mut<'a>() -> Option<OwningRefMut<StorageMutRef<'a, Self>, Self>> where Self: Sized { WORLD.try_resource_mut::<Self>() }

	// fn register_state(self) where Self: Sized { state::State::new(self).register_resource() }
	// fn state<'a>() -> OwningRef<StorageRef<'a, state::State<Self>>, state::State<Self>> where Self: Sized { <state::State<Self>>::resource() }
	// fn try_state<'a>() -> Option<OwningRef<StorageRef<'a, state::State<Self>>, state::State<Self>>> where Self: Sized { <state::State<Self>>::try_resource() }
}
impl<T: 'static + Sized> Component for T {}
