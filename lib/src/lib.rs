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
mod query;
mod hierarchy;

pub use hobo_css as css;
pub use hobo_derive::*;
pub use web_sys;
pub use paste;
use crate::prelude::*;
use std::collections::{HashMap, BTreeSet};
use std::any::TypeId;
use std::rc::Rc;
use once_cell::sync::Lazy;
use std::cell::{RefCell, Ref, RefMut};
use storage::*;
use owning_ref::{OwningRef, OwningRefMut, OwningHandle};
use style_storage::{STYLE_STORAGE, StyleStorage};
pub use element::{Element, Classes, SomeElement};
use racy_cell::RacyCell;
use sugars::hash;
pub use prelude::{Parent, Children};

// NOTES:
// queries to be able to find entities with/by components in children/parent/ancestor/family - done
// * optionaly specify depth?
// resources stay, resources could be useful for caching/memoization/etc
// add a name component that sets data-name or smth
// * should be possible to find by name for those cases where you cba to set up a proper relationship
// could use an attribute macro over intostyle expressions to give them names and use names rather than hashes
// organise uses, itnernal prelude uses could be pub(crate)
// test shit ffs
// move World and AsEntity into separate files
// could remove all *_mut elements and specify whether you want mutable or immutable component with the same trick as in Query

fn dom() -> web_sys::Document { web_sys::window().expect("no window").document().expect("no document") }

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Entity(u64);

impl Entity {
	fn root() -> Self { Self(0) }
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
	#[inline] fn try_get_cmp_mut<'a, C: 'static>(&self) -> Option<OwningRefMut<StorageGuard<'a, C, StorageRefMut<'a, C>>, C>> where Self: Sized {
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
	#[inline] fn get_cmp_mut<'a, C: 'static>(&self) -> OwningRefMut<StorageGuard<'a, C, StorageRefMut<'a, C>>, C> where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = OwningRefMut::new(world.storage_mut::<C>()).map_mut(|x| x.get_mut(self).unwrap());
		World::unmark_borrow_mut();
		res
	}
	#[inline] fn get_cmp_mut_or<'a, C: 'static>(&self, f: impl FnOnce() -> C) -> OwningRefMut<StorageGuard<'a, C, StorageRefMut<'a, C>>, C> where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = OwningRefMut::new(world.storage_mut::<C>()).map_mut(move |x| x.get_mut_or(self, f));
		World::unmark_borrow_mut();
		res
	}
	#[inline] fn get_cmp_mut_or_default<'a, C: 'static + Default>(&self) -> OwningRefMut<StorageGuard<'a, C, StorageRefMut<'a, C>>, C> where Self: Sized {
		self.get_cmp_mut_or(Default::default)
	}
	fn find_in_ancestors<Q: query::Query>(&self) -> Vec<Q::Fetch> {
		let mut entities = Some(Parent::ancestors(self.as_entity()).into_iter().collect());
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		Q::filter(world, &mut entities);
		let res = entities.unwrap_or_default().into_iter().map(|entity| Q::fetch(world, entity)).collect::<Vec<_>>();
		World::unmark_borrow_mut();
		res
	}
	fn try_find_first_in_ancestors<Q: query::Query>(&self) -> Option<Q::Fetch> {
		let mut entities = Some(Parent::ancestors(self.as_entity()).into_iter().collect());
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		Q::filter(world, &mut entities);
		let res = Some(Q::fetch(world, entities.unwrap_or_default().into_iter().next()?));
		World::unmark_borrow_mut();
		res
	}
	#[inline] fn find_first_in_ancestors<Q: query::Query>(&self) -> Q::Fetch { self.try_find_first_in_ancestors::<Q>().unwrap() }
	fn find_in_descendants<Q: query::Query>(&self) -> Vec<Q::Fetch> {
		let mut entities = Some(Children::descendants(self.as_entity()).into_iter().collect());
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		Q::filter(world, &mut entities);
		let res = entities.unwrap_or_default().into_iter().map(|entity| Q::fetch(world, entity)).collect::<Vec<_>>();
		World::unmark_borrow_mut();
		res
	}
	fn find_in_children<Q: query::Query>(&self) -> Vec<Q::Fetch> {
		let mut entities = Some(self.as_entity().try_get_cmp::<Children>().map_or_else(default, |x| x.0.iter().copied().collect()));
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		Q::filter(world, &mut entities);
		let res = entities.unwrap_or_default().into_iter().map(|entity| Q::fetch(world, entity)).collect::<Vec<_>>();
		World::unmark_borrow_mut();
		res
	}
	fn try_find_first_in_descendants<Q: query::Query>(&self) -> Option<Q::Fetch> {
		let mut entities = Some(Children::descendants(self.as_entity()).into_iter().collect());
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		Q::filter(world, &mut entities);
		let res = Some(Q::fetch(world, entities.unwrap_or_default().into_iter().next()?));
		World::unmark_borrow_mut();
		res
	}
	fn try_find_first_in_children<Q: query::Query>(&self) -> Option<Q::Fetch> {
		let mut entities = Some(self.as_entity().try_get_cmp::<Children>().map_or_else(default, |x| x.0.iter().copied().collect()));
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		Q::filter(world, &mut entities);
		let res = Some(Q::fetch(world, entities.unwrap_or_default().into_iter().next()?));
		World::unmark_borrow_mut();
		res
	}
	#[inline] fn find_first_in_descendants<Q: query::Query>(&self) -> Q::Fetch { self.try_find_first_in_descendants::<Q>().unwrap() }
	#[inline] fn find_first_in_children<Q: query::Query>(&self) -> Q::Fetch { self.try_find_first_in_children::<Q>().unwrap() }
	#[inline] fn has_cmp<C: 'static>(&self) -> bool where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = world.storage::<C>().has(self.as_entity());
		World::unmark_borrow_mut();
		res
	}

	#[inline] fn remove(&self) {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = world.remove_entity(self.as_entity());
		World::unmark_borrow_mut();
		res
	}
	#[inline] fn is_dead(&self)  -> bool {
		World::mark_borrow();
		let world = unsafe { &*WORLD.get() as &World };
		let res = world.is_dead(self.as_entity());
		World::unmark_borrow();
		res
	}
	#[inline] fn add_component<T: 'static>(&self, component: T) {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = world.storage_mut::<T>().add(self.as_entity(), component);
		World::unmark_borrow_mut();
		res
	}
	#[inline] fn component<T: 'static>(self, component: T) -> Self where Self: Sized { self.add_component(component); self }
}

impl AsEntity for Entity {
	fn as_entity(&self) -> Entity { *self }
}

type StorageRc = Rc<RefCell<Box<dyn DynStorage>>>;

// @Awpteamoose: I think this could have all members as non-cells and the World itself can be in a cell
// since in practice it turns out that WORLD isn't used, most methods are instead more conveniently called from AsEntity or smth
// maybe World doesn't even have to be pub
#[derive(Default)]
pub struct World {
	storages: HashMap<TypeId, StorageRc>,
	// this is used to remove components for when an entity has been removed
	component_ownership: HashMap<Entity, BTreeSet<TypeId>>,
	next_entity: u64,
	alive_entities: BTreeSet<Entity>,
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

			let mut res = String::new();
			{
				let classes = storage.get(entity).unwrap();

				if let Some(id) = &classes.type_tag {
					write!(&mut res, "t-{:x} ", hash!(id)).unwrap();
				}

				let style_storage = unsafe { &mut *STYLE_STORAGE.get() as &mut StyleStorage };
				for style in classes.styles.values() {
					write!(&mut res, "{} ", style_storage.fetch(style.clone())).unwrap();
				}
			}

			let elements = world.storage::<web_sys::Element>();
			elements.get(entity).unwrap().set_attribute(web_str::class(), res.trim()).expect("can't set class attribute");
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
type StorageRefMut<'a, Component> = OwningRefMut<OwningHandle<Rc<RefCell<Box<(dyn storage::DynStorage + 'static)>>>, RefMut<'a, Box<dyn storage::DynStorage>>>, SimpleStorage<Component>>;

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

	pub fn storage_mut<Component: 'static>(&mut self) -> StorageGuard<Component, StorageRefMut<Component>> {
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

	pub fn resource_mut<T: 'static>(&mut self) -> OwningRefMut<StorageGuard<T, StorageRefMut<T>>, T> {
		OwningRefMut::new(self.storage_mut()).map_mut(|x| x.get_mut(Entity::root()).unwrap())
	}

	pub fn resource_exists<T: 'static>(&mut self) -> bool {
		self.storage::<T>().has(Entity::root())
	}

	pub fn try_resource<T: 'static>(&mut self) -> Option<OwningRef<StorageRef<T>, T>> {
		if !self.storage::<T>().has(Entity::root()) { return None; }
		Some(OwningRef::new(self.storage()).map(|x| x.get(Entity::root()).unwrap()))
	}

	pub fn try_resource_mut<T: 'static>(&mut self) -> Option<OwningRefMut<StorageGuard<T, StorageRefMut<T>>, T>> {
		if !self.storage::<T>().has(Entity::root()) { return None; }
		Some(OwningRefMut::new(self.storage_mut()).map_mut(|x| x.get_mut(Entity::root()).unwrap()))
	}

	pub fn new_entity(&mut self) -> Entity {
		let entity = Entity(self.next_entity);
		self.next_entity += 1;
		self.alive_entities.insert(entity);
		entity
	}

	pub fn remove_entity(&mut self, entity: impl AsEntity) {
		let entity = entity.as_entity();
		if self.is_dead(entity) { log::warn!("remove entity already dead {:?}", entity); return; }

		let children = self.storage::<Children>().get(entity).map(|x| x.0.clone());
		if let Some(children) = children {
			for child in children { self.remove_entity(child); }
		}

		self.alive_entities.remove(&entity);

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
		!self.alive_entities.contains(&entity)
	}
}

impl<T: AsEntity> AsEntity for &T {
	fn as_entity(&self) -> Entity { T::as_entity(*self) }
}
impl<T: AsEntity> AsEntity for &mut T {
	fn as_entity(&self) -> Entity { T::as_entity(*self) }
}

pub fn fetch_classname(style: impl Into<css::Style>) -> String {
	let style_storage = unsafe { &mut *STYLE_STORAGE.get() as &mut StyleStorage };
	style_storage.fetch(style.into())
}

pub fn register_window(window: &web_sys::Window) {
	let style_storage = unsafe { &mut *STYLE_STORAGE.get() as &mut StyleStorage };
	style_storage.register_window(window);
}

#[extend::ext(pub, name = TypeClassString)]
impl<T: 'static> T {
	fn type_class_string() -> String {
		format!("t-{:x}", hash!(TypeId::of::<Self>()))
	}
}

pub fn find<Q: query::Query>() -> Vec<Q::Fetch> {
	World::mark_borrow_mut();
	let world = unsafe { &mut *WORLD.get() as &mut World };
	let mut entities = None;
	Q::filter(world, &mut entities);
	let res = entities.unwrap_or_default().into_iter().map(|entity| Q::fetch(world, entity)).collect::<Vec<_>>();
	World::unmark_borrow_mut();
	res
}

pub fn try_find_one<Q: query::Query>() -> Option<Q::Fetch> {
	World::mark_borrow_mut();
	let world = unsafe { &mut *WORLD.get() as &mut World };
	let mut entities = None;
	Q::filter(world, &mut entities);
	let res = entities.unwrap_or_default().into_iter().next().map(|entity| Q::fetch(world, entity));
	World::unmark_borrow_mut();
	res
}

pub fn find_one<Q: query::Query>() -> Q::Fetch { try_find_one::<Q>().unwrap() }

pub trait Resource: 'static {
	#[inline] fn register_resource(self) where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		World::register_resource(world, self);
		World::unmark_borrow_mut();
	}
	#[inline] fn resource<'a>() -> OwningRef<StorageRef<'a, Self>, Self> where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = World::resource::<Self>(world);
		World::unmark_borrow_mut();
		res
	}
	#[inline] fn resource_mut<'a>() -> OwningRefMut<StorageGuard<'a, Self, StorageRefMut<'a, Self>>, Self> where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = World::resource_mut::<Self>(world);
		World::unmark_borrow_mut();
		res
	}
	#[inline] fn try_resource<'a>() -> Option<OwningRef<StorageRef<'a, Self>, Self>> where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = World::try_resource::<Self>(world);
		World::unmark_borrow_mut();
		res
	}
	#[inline] fn try_resource_mut<'a>() -> Option<OwningRefMut<StorageGuard<'a, Self, StorageRefMut<'a, Self>>, Self>> where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = World::try_resource_mut::<Self>(world);
		World::unmark_borrow_mut();
		res
	}
}
impl<T: 'static + Sized> Resource for T {}

pub trait DefaultResource: Default + 'static {
	#[inline] fn resource_mut_or_default<'a>() -> OwningRefMut<StorageGuard<'a, Self, StorageRefMut<'a, Self>>, Self> where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		if !World::resource_exists::<Self>(world) {
			World::register_resource(world, Self::default());
		}
		let res = World::resource_mut::<Self>(world);
		World::unmark_borrow_mut();
		res
	}
}
impl<T: Default + 'static + Sized> DefaultResource for T {}
