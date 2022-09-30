use crate::prelude::*;

use crate::{
	create,
	element::Classes,
	storage::{SimpleStorage, StorageGuard, StorageGuardMut},
	style_storage::{StyleStorage, STYLE_STORAGE},
	StorageRef, StorageRefMut,
};
use once_cell::sync::Lazy;
use owning_ref::{OwningRef, OwningRefMut};
use std::{
	any::TypeId,
	cell::RefCell,
	collections::{BTreeSet, HashMap},
	sync::atomic::{AtomicU64, Ordering},
};
use sugars::hash;

pub(crate) static WORLD: Lazy<World> = Lazy::new(|| {
	let mut world = World::default();
	world.next_entity = AtomicU64::new(1);
	world.component_ownership.borrow_mut().insert(Entity::root(), BTreeSet::default());

	{
		fn update_classes(storage: &mut SimpleStorage<Classes>, world: &World, entity: Entity) {
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

	create::register_handlers(&world);

	world
});

// @Awpteamoose: maybe World doesn't even have to be pub
#[derive(Default)]
pub struct World {
	pub(crate) storages: elsa::FrozenMap<TypeId, &'static RefCell<Box<dyn DynStorage>>>,
	// this is used to remove components for when an entity has been removed
	pub(crate) component_ownership: RefCell<HashMap<Entity, BTreeSet<TypeId>>>,
	next_entity: AtomicU64,
}

// safe because js is single-threaded (for now)
unsafe impl Send for World {}
unsafe impl Sync for World {}

impl World {
	#[cfg(debug_assertions)]
	#[track_caller]
	pub(crate) fn dyn_storage<Component: 'static>(&self) -> std::cell::Ref<'static, Box<dyn DynStorage>> {
		let caller = std::panic::Location::caller();

		if let Some(storage) = self.storages.map_get(&TypeId::of::<Component>(), |x| x.try_borrow()) {
			storage.unwrap_or_else(|e| panic!("'{e}': Immutably borrowed `{}` storage at `{caller}` while a mutable borrow to it already exists:\n\n{}\n",
				std::any::type_name::<Component>(),
				crate::backtrace::STORAGE_MAP.0.borrow()[&TypeId::of::<Component>()]
			))
		} else {
			let storage: RefCell<Box<dyn DynStorage>> = RefCell::new(Box::new(SimpleStorage::<Component>::default()));
			let storage: &'static _ = Box::leak(Box::new(storage));
			self.storages.insert(TypeId::of::<Component>(), storage);
			storage.borrow()
		}
	}

	#[cfg(not(debug_assertions))]
	#[track_caller]
	pub(crate) fn dyn_storage<Component: 'static>(&self) -> std::cell::Ref<'static, Box<dyn DynStorage>> {
		if let Some(storage) = self.storages.map_get(&TypeId::of::<Component>(), |x| x.borrow()) {
			storage
		} else {
			let storage: RefCell<Box<dyn DynStorage>> = RefCell::new(Box::new(SimpleStorage::<Component>::default()));
			let storage: &'static _ = Box::leak(Box::new(storage));
			self.storages.insert(TypeId::of::<Component>(), storage);
			storage.borrow()
		}
	}

	#[cfg(debug_assertions)]
	#[track_caller]
	pub(crate) fn dyn_storage_mut<Component: 'static>(&self) -> std::cell::RefMut<'static, Box<dyn DynStorage>> {
		let caller = std::panic::Location::caller();

		if let Some(storage) = self.storages.map_get(&TypeId::of::<Component>(), |x| x.try_borrow_mut()) {
			storage.unwrap_or_else(|e| panic!("'{e}': Mutably borrowed `{}` storage at `{caller}` while other borrows to it already exist:\n\n{}\n",
				std::any::type_name::<Component>(),
				crate::backtrace::STORAGE_MAP.0.borrow()[&TypeId::of::<Component>()]
			))
		} else {
			let storage: RefCell<Box<dyn DynStorage>> = RefCell::new(Box::new(SimpleStorage::<Component>::default()));
			let storage: &'static _ = Box::leak(Box::new(storage));
			self.storages.insert(TypeId::of::<Component>(), storage);
			storage.borrow_mut()
		}
	}

	#[cfg(not(debug_assertions))]
	#[track_caller]
	pub(crate) fn dyn_storage_mut<Component: 'static>(&self) -> std::cell::RefMut<'static, Box<dyn DynStorage>> {
		if let Some(storage) = self.storages.map_get(&TypeId::of::<Component>(), |x| x.borrow_mut()) {
			storage.unwrap()
		} else {
			let storage: RefCell<Box<dyn DynStorage>> = RefCell::new(Box::new(SimpleStorage::<Component>::default()));
			let storage: &'static _ = Box::leak(Box::new(storage));
			self.storages.insert(TypeId::of::<Component>(), storage);
			storage.borrow_mut()
		}
	}

	// INFO: Anything that calls storage or storage_mut should have track_caller.
	// This is so that the StorageGuard can have accurate location from user-code in debug.
	#[track_caller]
	pub fn storage<Component: 'static>(&self) -> StorageGuard<Component, StorageRef<Component>> {
		#[cfg(debug_assertions)]
		crate::backtrace::STORAGE_MAP.0.borrow_mut()
			.entry(TypeId::of::<Component>())
			// .entry(std::any::type_name::<Component>().to_owned())
			.or_default()
			.insert(*std::panic::Location::caller(), false);

		let storage = OwningRef::new(self.dyn_storage::<Component>())
			.map(|x| x.as_any().downcast_ref().unwrap());

		StorageGuard {
			inner: Some(storage),
			#[cfg(debug_assertions)]
			location: *std::panic::Location::caller()
		}
	}

	#[track_caller]
	pub fn storage_mut<Component: 'static>(&self) -> StorageGuardMut<Component, StorageRefMut<Component>> {
		#[cfg(debug_assertions)]
		crate::backtrace::STORAGE_MAP.0.borrow_mut()
			.entry(TypeId::of::<Component>())
			// .entry(std::any::type_name::<Component>().to_owned())
			.or_default()
			.insert(*std::panic::Location::caller(), true);

		let storage = OwningRefMut::new(self.dyn_storage_mut::<Component>())
			.map_mut(|x| x.as_any_mut().downcast_mut().unwrap());

		StorageGuardMut {
			world: self,
			inner: Some(storage),
			#[cfg(debug_assertions)]
			location: *std::panic::Location::caller()
		}
	}

	#[track_caller]
	pub fn register_resource<T: 'static>(&self, resource: T) { self.storage_mut().add(Entity::root(), resource); }

	/// Resources are just components attached to Entity(0)
	#[track_caller]
	pub fn resource<T: 'static>(&self) -> OwningRef<StorageGuard<T, StorageRef<T>>, T> {
		OwningRef::new(self.storage()).map(|x| x.get(Entity::root()).unwrap())
	}

	#[track_caller]
	pub fn resource_mut<T: 'static>(&self) -> OwningRefMut<StorageGuardMut<T, StorageRefMut<T>>, T> {
		OwningRefMut::new(self.storage_mut()).map_mut(|x| x.get_mut(Entity::root()).unwrap())
	}

	#[track_caller]
	pub fn resource_exists<T: 'static>(&self) -> bool {
		self.storage::<T>().has(Entity::root())
	}

	#[track_caller]
	pub fn try_resource<T: 'static>(&self) -> Option<OwningRef<StorageGuard<T, StorageRef<T>>, T>> {
		if !self.storage::<T>().has(Entity::root()) { return None; }
		Some(OwningRef::new(self.storage()).map(|x| x.get(Entity::root()).unwrap()))
	}

	#[track_caller]
	pub fn try_resource_mut<T: 'static>(&self) -> Option<OwningRefMut<StorageGuardMut<T, StorageRefMut<T>>, T>> {
		if !self.storage::<T>().has(Entity::root()) { return None; }
		Some(OwningRefMut::new(self.storage_mut()).map_mut(|x| x.get_mut(Entity::root()).unwrap()))
	}

	pub fn new_entity(&self) -> Entity {
		let entity = Entity(self.next_entity.fetch_add(1, Ordering::Relaxed));
		self.component_ownership.borrow_mut().insert(entity, BTreeSet::default());
		entity
	}

	#[track_caller]
	pub fn remove_entity(&self, entity: impl AsEntity) {
		let entity = entity.as_entity();
		if self.is_dead(entity) {
			log::warn!("remove entity already dead {:?}", entity);
			return;
		}

		let children = self.storage::<Children>().get(entity).map(|x| x.0.clone());
		if let Some(children) = children {
			for child in children { self.remove_entity(child); }
		}

		let parent = self.storage::<Parent>().get(entity).copied();
		if let Some(parent) = parent {
			let mut children_store = self.storage_mut::<Children>();
			let children = children_store.get_mut(parent).unwrap();
			if let Some(child_pos) = children.0.iter().position(|&x| x == entity) {
				children.0.remove(child_pos);
			}
		}

		let components = self.component_ownership.borrow_mut().remove(&entity).unwrap();
		for component_id in components {
			let mut storage = self.storages.map_get(&component_id, |x| x.try_borrow_mut().expect("remove_entity storages -> storage.try_borrow_mut .. remove")).unwrap();
			storage.dyn_remove(entity);
			storage.flush(self);
		}
	}

	pub fn is_dead(&self, entity: impl AsEntity) -> bool {
		let entity = entity.as_entity();
		!self.component_ownership.borrow().contains_key(&entity)
	}
}
