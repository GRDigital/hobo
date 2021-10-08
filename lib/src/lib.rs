pub mod prelude;
pub mod web_str;
mod style_storage;
mod enclose;
mod storage;
pub mod state;
pub mod create;
mod dom_events;
pub mod events;
mod element;
mod racy_cell;

pub use hobo_css as css;
pub use hobo_derive::*;
pub use web_sys;
pub use paste;
use crate::prelude::*;
use std::collections::{HashMap, HashSet};
use std::any::TypeId;
use std::rc::Rc;
use once_cell::sync::Lazy;
use std::cell::{RefCell, Ref, RefMut};
use storage::*;
use owning_ref::{OwningRef, OwningRefMut, OwningHandle};
use style_storage::STYLE_STORAGE;
pub use element::{Element, Classes, Parent, Children, SomeElement};
use racy_cell::RacyCell;

// NOTES:
// remove systems - handle deletion of web_sys stuff directly and handle classes changes directly
// remove entity generations
// queries to be able to find entities with/by components
// queries to be able to find entities with/by components in children/parent/ancestor/family
// * optionaly specify depth?
// maybe can register event handlers (add, remove, change) for storages for each storage
// resources stay, resources could be useful for caching/memoization/etc

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
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let entity = self.as_entity();
		let storage = world.storage::<C>();
		if !storage.has(entity) {
			World::unmark_borrow_mut();
			return None;
		}
		let res = Some(OwningRef::new(storage).map(|x| x.get(entity).unwrap()));
		World::unmark_borrow_mut();
		res
	}
	#[inline] fn try_get_cmp_mut<'a, C: 'static>(&self) -> Option<OwningRefMut<StorageMutRef<'a, C>, C>> where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let entity = self.as_entity();
		if !world.storage::<C>().has(entity) {
			World::unmark_borrow_mut();
			return None;
		}
		let res = Some(OwningRefMut::new(world.storage_mut::<C>()).map_mut(|x| x.get_mut(entity).unwrap()));
		World::unmark_borrow_mut();
		res
	}
	#[inline] fn get_cmp<'a, C: 'static>(&self) -> OwningRef<StorageRef<'a, C>, C> where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = OwningRef::new(world.storage::<C>()).map(|x| x.get(self).unwrap());
		World::unmark_borrow_mut();
		res
	}
	#[inline] fn get_cmp_mut<'a, C: 'static>(&self) -> OwningRefMut<StorageMutRef<'a, C>, C> where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = OwningRefMut::new(world.storage_mut::<C>()).map_mut(|x| x.get_mut(self).unwrap());
		World::unmark_borrow_mut();
		res
	}
	#[inline] fn get_cmp_mut_or<'a, C: 'static>(&self, f: impl FnOnce() -> C) -> OwningRefMut<StorageMutRef<'a, C>, C> where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = OwningRefMut::new(world.storage_mut::<C>()).map_mut(move |x| x.get_mut_or(self, f));
		World::unmark_borrow_mut();
		res
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
	#[inline] fn has_cmp<'a, C: 'static>(&self) -> bool where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = world.storage::<C>().has(self.as_entity());
		World::unmark_borrow_mut();
		res
	}

	fn remove(&self) {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = world.remove_entity(self.as_entity());
		World::unmark_borrow_mut();
		res
	}
	fn is_dead(&self)  -> bool {
		World::mark_borrow();
		let world = unsafe { &*WORLD.get() as &World };
		let res = world.is_dead(self.as_entity());
		World::unmark_borrow();
		res
	}
	fn add_component<T: 'static>(&self, component: T) {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = world.storage_mut::<T>().add(self.as_entity(), component);
		World::unmark_borrow_mut();
		res
	}
	fn component<T: 'static>(self, component: T) -> Self where Self: Sized { self.add_component(component); self }
}

impl AsEntity for Entity {
	fn as_entity(&self) -> Entity { *self }
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

// @Awpteamoose: I think this could have all members as non-cells and the World itself can be in a cell
// since in practice it turns out that WORLD isn't used, most methods are instead more conveniently called from AsEntity or smth
// maybe World doesn't even have to be pub
#[derive(Default)]
pub struct World {
	storages: HashMap<TypeId, StorageRc>,
	// this is used to remove components for when an entity has been removed
	component_ownership: HashMap<Entity, HashSet<TypeId>>,
	entities: Entities,
}

// super turbo unsafe and dangerous, in debug checked at runtime via a global scope pseudo-refcell refcount
// worth it tho since it's being constantly hit and wrapping everything in RefCells introduces a nontrivial perf cost
// but also convenience/efficiency cost since you don't actually get a reference from a RefCell
#[cfg(debug_assertions)] pub(crate) static WORLD_BORROWED: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
#[cfg(debug_assertions)] pub(crate) static WORLD_BORROWED_MUT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub(crate) static WORLD: Lazy<RacyCell<World>> = Lazy::new(|| RacyCell::new({
	let mut world = World::default();

	{
		fn update_classes(storage: &mut SimpleStorage<Classes>, world: &mut World, entity: Entity) {
			use std::fmt::Write;

			let mut res = format!("e{}g{}", entity.id, entity.generation);
			{
				let classes = storage.get(entity).unwrap();

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
			}

			let elements = world.storage::<web_sys::Element>();
			elements.get(entity).unwrap().set_attribute(web_str::class(), &res).expect("can't set class attribute");
		}

		let mut classes = world.storage_mut::<Classes>();
		classes.on_added = Some(update_classes);
		classes.on_modified = Some(update_classes);
	}

	create::register_handlers(&mut world);

	world
}));

// this is not necessary, but it makes it convenient to further remap to some OwningRef or whatever
type StorageRef<'a, Component> = OwningRef<OwningHandle<Rc<RefCell<Box<(dyn storage::DynStorage + 'static)>>>, Ref<'a, Box<dyn storage::DynStorage>>>, SimpleStorage<Component>>;
type StorageMutRef<'a, Component> = StorageGuard<'a, Component, OwningRefMut<OwningHandle<Rc<RefCell<Box<(dyn storage::DynStorage + 'static)>>>, RefMut<'a, Box<dyn storage::DynStorage>>>, SimpleStorage<Component>>>;

impl World {
	#[track_caller]
	#[inline]
	pub(crate) fn mark_borrow() {
		#[cfg(debug_assertions)] {
			if WORLD_BORROWED_MUT.load(std::sync::atomic::Ordering::Relaxed) { panic!("trying to borrow World while it's mutably borrowed") }
			WORLD_BORROWED.store(WORLD_BORROWED.load(std::sync::atomic::Ordering::Relaxed) + 1, std::sync::atomic::Ordering::Relaxed);
		}
	}

	#[track_caller]
	#[inline]
	pub(crate) fn unmark_borrow() {
		#[cfg(debug_assertions)] {
			if WORLD_BORROWED_MUT.load(std::sync::atomic::Ordering::Relaxed) { panic!("trying to return borrow World but it's mutably borrowed") }
			if WORLD_BORROWED.load(std::sync::atomic::Ordering::Relaxed) == 0 { panic!("trying to return borrow World but it's not borrowed") }
			WORLD_BORROWED.store(WORLD_BORROWED.load(std::sync::atomic::Ordering::Relaxed) - 1, std::sync::atomic::Ordering::Relaxed);
		}
	}

	#[track_caller]
	#[inline]
	pub(crate) fn mark_borrow_mut() {
		#[cfg(debug_assertions)] {
			if WORLD_BORROWED.load(std::sync::atomic::Ordering::Relaxed) > 0 { panic!("trying to mutably borrow World while it's already got a borrow") }
			if WORLD_BORROWED_MUT.load(std::sync::atomic::Ordering::Relaxed) { panic!("trying to mutably borrow World while it's mutably borrowed") }
			WORLD_BORROWED_MUT.store(true, std::sync::atomic::Ordering::Relaxed);
		}
	}

	#[track_caller]
	#[inline]
	pub(crate) fn unmark_borrow_mut() {
		#[cfg(debug_assertions)] {
			if WORLD_BORROWED.load(std::sync::atomic::Ordering::Relaxed) > 0 { panic!("trying to return mutable borrow World but it's got a borrow") }
			if !WORLD_BORROWED_MUT.load(std::sync::atomic::Ordering::Relaxed) { panic!("trying to return mutable borrow World but it's not mutably borrowed") }
			WORLD_BORROWED_MUT.store(false, std::sync::atomic::Ordering::Relaxed);
		}
	}

	fn dyn_storage<Component: 'static>(&mut self) -> Rc<RefCell<Box<dyn DynStorage>>> {
		Rc::clone(self.storages
			.entry(TypeId::of::<Component>())
			.or_insert_with(|| Rc::new(RefCell::new(Box::new(SimpleStorage::<Component>::default())))))
	}

	pub fn storage_mut<Component: 'static>(&mut self) -> StorageMutRef<Component> {
		let storage = OwningRefMut::new(OwningHandle::new_mut(self.dyn_storage::<Component>()))
			.map_mut(|x| x.as_any_mut().downcast_mut().unwrap());
		StorageGuard(self, Some(storage))
	}

	pub fn storage<Component: 'static>(&mut self) -> StorageRef<Component> {
		OwningRef::new(OwningHandle::new(self.dyn_storage::<Component>()))
			.map(|x| x.as_any().downcast_ref().unwrap())
	}

	pub fn register_resource<T: 'static>(&mut self, resource: T) { self.storage_mut().add(Entity::root(), resource); }

	// resources are just components attached to Entity(0, 0)
	pub fn resource<T: 'static>(&mut self) -> OwningRef<StorageRef<T>, T> {
		OwningRef::new(self.storage()).map(|x| x.get(Entity::root()).unwrap())
	}

	pub fn resource_mut<T: 'static>(&mut self) -> OwningRefMut<StorageMutRef<T>, T> {
		OwningRefMut::new(self.storage_mut()).map_mut(|x| x.get_mut(Entity::root()).unwrap())
	}

	pub fn try_resource<T: 'static>(&mut self) -> Option<OwningRef<StorageRef<T>, T>> {
		if !self.storage::<T>().has(Entity::root()) { return None; }
		Some(OwningRef::new(self.storage()).map(|x| x.get(Entity::root()).unwrap()))
	}

	pub fn try_resource_mut<T: 'static>(&mut self) -> Option<OwningRefMut<StorageMutRef<T>, T>> {
		if !self.storage::<T>().has(Entity::root()) { return None; }
		Some(OwningRefMut::new(self.storage_mut()).map_mut(|x| x.get_mut(Entity::root()).unwrap()))
	}

	// if there's a removed entity, take that one
	// otherwise the next id would be the one with unassigned generation,
	// i.e. index of last generation + 1
	pub fn new_entity(&mut self) -> Entity {
		if let Some(id) = self.entities.free_ids.pop() {
			Entity { id, generation: self.entities.generations[id as usize] }
		} else {
			let id = self.entities.generations.len() as u32;
			self.entities.generations.push(0);
			Entity { id, generation: 0 }
		}
	}

	pub fn remove_entity(&mut self, entity: impl AsEntity) {
		let entity = entity.as_entity();
		if self.is_dead(entity) { log::warn!("remove entity already dead {:?}", entity); return; }

		// if let Some(children) = entity.try_get_cmp::<Children>().map(|x| x.0.clone()) {
		let children = self.storage::<Children>().get(entity).map(|x| x.0.clone());
		if let Some(children) = children {
			for child in children { self.remove_entity(child); }
		}

		{
			self.entities.free_ids.push(entity.id);
			self.entities.generations[entity.id as usize] += 1;
		}

		let parent = self.storage::<Parent>().get(entity).copied();
		if let Some(parent) = parent {
			let mut children_store = self.storage_mut::<Children>();
			let children = children_store.get_mut(parent).unwrap();
			if let Some(child_pos) = children.0.iter().position(|&x| x == entity) { children.0.remove(child_pos); }
		}

		if let Some(component_ids) = self.component_ownership.remove(&entity) {
			for component_id in component_ids {
				let storage = Rc::clone(&self.storages[&component_id]);
				let mut storage = storage.try_borrow_mut().expect("remove_entity storages -> storage.try_borrow_mut .. remove");
				storage.dyn_remove(entity);
				storage.flush(self);
			}
		}
	}

	pub fn is_dead(&self, entity: impl AsEntity) -> bool {
		let entity = entity.as_entity();
		self.entities.generations[entity.id as usize] != entity.generation
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

// pub trait Component: 'static {
//     #[inline] fn register_resource(self) where Self: Sized { WORLD.with(move |world| World::register_resource(&*world.borrow(), self)) }
//     #[inline] fn resource<'a>() -> OwningRef<StorageRef<'a, Self>, Self> where Self: Sized { WORLD.with(move |world| World::resource::<Self>(&*world.borrow())) }
//     #[inline] fn resource_mut<'a>() -> OwningRefMut<StorageMutRef<'a, Self>, Self> where Self: Sized { WORLD.with(move |world| World::resource_mut::<Self>(&*world.borrow())) }
//     #[inline] fn try_resource<'a>() -> Option<OwningRef<StorageRef<'a, Self>, Self>> where Self: Sized { WORLD.with(move |world| World::try_resource::<Self>(&*world.borrow())) }
//     #[inline] fn try_resource_mut<'a>() -> Option<OwningRefMut<StorageMutRef<'a, Self>, Self>> where Self: Sized { WORLD.try_resource_mut::<Self>() }
// }
// impl<T: 'static + Sized> Component for T {}
