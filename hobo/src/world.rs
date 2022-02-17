use crate::prelude::*;

use crate::{
	create,
	element::Classes,
	racy_cell::RacyCell,
	storage::{SimpleStorage, StorageGuard},
	style_storage::{StyleStorage, STYLE_STORAGE},
	StorageRef, StorageRefMut,
};
use once_cell::sync::Lazy;
use owning_ref::{OwningHandle, OwningRef, OwningRefMut};
use std::{
	any::TypeId,
	cell::RefCell,
	collections::{BTreeSet, HashMap},
	rc::Rc,
};
use sugars::hash;

type StorageRc = Rc<RefCell<Box<dyn DynStorage>>>;

// super turbo unsafe and dangerous, in debug checked at runtime via a global scope pseudo-refcell refcount
// worth it tho since it's being constantly hit and wrapping everything in RefCells introduces a nontrivial perf cost
// but also convenience/efficiency cost since you don't actually get a reference from a RefCell
#[cfg(debug_assertions)] pub(crate) static WORLD_BORROWED: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
#[cfg(debug_assertions)] pub(crate) static WORLD_BORROWED_MUT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub(crate) static WORLD: Lazy<RacyCell<World>> = Lazy::new(|| RacyCell::new({
	let mut world = World::default();
	world.next_entity = 1;

	{
		fn update_classes(storage: &mut SimpleStorage<Classes>, world: &mut World, entity: Entity) {
			use std::fmt::Write;

			let mut res = String::new();
			{
				let classes = storage.get(entity).unwrap();

				for id in &classes.marks {
					write!(&mut res, "t-{:x} ", hash!(id)).unwrap();
				}

				let style_storage = unsafe { &mut *STYLE_STORAGE.get() as &mut StyleStorage };
				for style in classes.styles.values() {
					write!(&mut res, "{} ", style_storage.fetch(style.clone())).unwrap();
				}
			}

			let elements = world.storage::<web_sys::Element>();
			let element = elements.get(entity).unwrap();
			let res = res.trim();
			if res.is_empty() {
				element.remove_attribute(web_str::class()).expect("can't remove class attribute");
			} else {
				element.set_attribute(web_str::class(), res).expect("can't set class attribute");
			}
		}

		let mut classes = world.storage_mut::<Classes>();
		classes.on_added = Some(update_classes);
		classes.on_modified = Some(update_classes);
	}

	create::register_handlers(&mut world);

	world
}));

// @Awpteamoose: I think this could have all members as non-cells and the World itself can be in a cell
// since in practice it turns out that WORLD isn't used, most methods are instead more conveniently called from AsEntity or smth
// maybe World doesn't even have to be pub
#[derive(Default)]
pub struct World {
	pub(crate) storages: HashMap<TypeId, StorageRc>,
	// this is used to remove components for when an entity has been removed
	pub(crate) component_ownership: HashMap<Entity, BTreeSet<TypeId>>,
	next_entity: u64,
	alive_entities: BTreeSet<Entity>,
}

impl World {
	#[doc(hidden)]
	#[inline]
	pub fn is_marked_borrow_mut() -> bool {
		#[cfg(debug_assertions)] {
			return WORLD_BORROWED_MUT.load(std::sync::atomic::Ordering::Relaxed)
		}
		false
	}

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

	#[doc(hidden)]
	#[track_caller]
	#[inline]
	pub fn mark_borrow_mut() {
		#[cfg(debug_assertions)] {
			if WORLD_BORROWED.load(std::sync::atomic::Ordering::Relaxed) > 0 { panic!("trying to mutably borrow World while it's already got a borrow") }
			if WORLD_BORROWED_MUT.load(std::sync::atomic::Ordering::Relaxed) { panic!("trying to mutably borrow World while it's mutably borrowed") }
			WORLD_BORROWED_MUT.store(true, std::sync::atomic::Ordering::Relaxed);
		}
	}

	#[doc(hidden)]
	#[track_caller]
	#[inline]
	pub fn unmark_borrow_mut() {
		#[cfg(debug_assertions)] {
			if WORLD_BORROWED.load(std::sync::atomic::Ordering::Relaxed) > 0 { panic!("trying to return mutable borrow World but it's got a borrow") }
			if !WORLD_BORROWED_MUT.load(std::sync::atomic::Ordering::Relaxed) { panic!("trying to return mutable borrow World but it's not mutably borrowed") }
			WORLD_BORROWED_MUT.store(false, std::sync::atomic::Ordering::Relaxed);
		}
	}

	pub(crate) fn dyn_storage<Component: 'static>(&mut self) -> Rc<RefCell<Box<dyn DynStorage>>> {
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
		if self.is_dead(entity) {
			log::warn!("remove entity already dead {:?}", entity);
			return;
		}

		let children = self.storage::<Children>().get(entity).map(|x| x.0.clone());
		if let Some(children) = children {
			for child in children { self.remove_entity(child); }
		}

		self.alive_entities.remove(&entity);

		let parent = self.storage::<Parent>().get(entity).copied();
		if let Some(parent) = parent {
			let mut children_store = self.storage_mut::<Children>();
			let children = children_store.get_mut(parent).unwrap();
			if let Some(child_pos) = children.0.iter().position(|&x| x == entity) {
				children.0.remove(child_pos);
			}
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

pub struct WorldMut(pub(crate) &'static mut World);
impl AsRef<World> for WorldMut {
	fn as_ref(&self) -> &World { &self.0 }
}

impl AsMut<World> for WorldMut {
	fn as_mut(&mut self) -> &mut World { &mut self.0 }
}

impl Drop for WorldMut {
	fn drop(&mut self) { World::unmark_borrow_mut(); }
}
