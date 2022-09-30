use crate::{prelude::default, AsEntity, Entity, World};
use std::collections::{HashMap, HashSet};

pub trait DynStorage: as_any::AsAny {
	fn dyn_has(&self, entity: Entity) -> bool;
	fn dyn_remove(&mut self, entity: Entity);
	fn flush(&mut self, world: &World);
}

pub trait Storage<Component: 'static>: DynStorage {
	fn has(&self, entity: impl AsEntity) -> bool { self.dyn_has(entity.as_entity()) }
	fn remove(&mut self, entity: impl AsEntity) { self.dyn_remove(entity.as_entity()) }
	fn get(&self, entity: impl AsEntity) -> Option<&Component>;
	fn get_mut(&mut self, entity: impl AsEntity) -> Option<&mut Component>;
	fn get_removed(&self, entity: impl AsEntity) -> Option<&Component>;
	fn get_removed_mut(&mut self, entity: impl AsEntity) -> Option<&mut Component>;
	fn take_removed(&mut self, entity: impl AsEntity) -> Option<Component>;
	fn get_mut_or(&mut self, entity: impl AsEntity, f: impl FnOnce() -> Component) -> &mut Component;
	fn add(&mut self, entity: impl AsEntity, component: Component);

	fn get_mut_or_default(&mut self, entity: impl AsEntity) -> &mut Component where Component: Default { self.get_mut_or(entity, Component::default) }
}

pub struct SimpleStorage<Component: 'static> {
	pub data: HashMap<Entity, Component>,
	pub data_removed: HashMap<Entity, Component>,

	pub added: HashSet<Entity>,
	pub modified: HashSet<Entity>,
	pub removed: HashSet<Entity>,

	pub on_added: Option<fn(&mut SimpleStorage<Component>, &World, Entity)>,
	pub on_modified: Option<fn(&mut SimpleStorage<Component>, &World, Entity)>,
	pub on_removed: Option<fn(&mut SimpleStorage<Component>, &World, Entity, Component)>,
}

impl<Component> Default for SimpleStorage<Component> {
	fn default() -> Self {
		Self {
			data: default(),
			data_removed: default(),

			added: default(),
			removed: default(),
			modified: default(),

			on_added: default(),
			on_modified: default(),
			on_removed: default(),
		}
	}
}

impl<Component: 'static> DynStorage for SimpleStorage<Component> {
	fn dyn_has(&self, entity: Entity) -> bool {
		self.data.contains_key(&entity)
	}

	fn dyn_remove(&mut self, entity: Entity) {
		// log::info!("removing {:?}", std::any::type_name::<Component>());
		if let Some(cmp) = self.data.remove(&entity) {
			self.data_removed.insert(entity, cmp);
			self.removed.insert(entity);
		}
	}

	/// On *storage drop* (not component), update component_ownership and trigger added/modified/removed.
	///
	/// We get storage -> we add/remove/modify a component -> storage gets dropped -> flush updates
	fn flush(&mut self, world: &World) {
		for added in &self.added {
			world.component_ownership.borrow_mut().get_mut(added).unwrap().insert(std::any::TypeId::of::<Component>());
		}

		for removed in &self.removed {
			// won't be present if entity was deleted
			if let Some(ownership) = world.component_ownership.borrow_mut().get_mut(removed) {
				ownership.remove(&std::any::TypeId::of::<Component>());
			}
		}

		let entities = std::mem::take(&mut self.added);
		if let Some(f) = self.on_added {
			for &entity in &entities {
				f(self, world, entity);
			}
		}

		let entities = std::mem::take(&mut self.modified);
		if let Some(f) = self.on_modified {
			for &entity in &entities {
				f(self, world, entity);
			}
		}

		let entities = std::mem::take(&mut self.removed);
		let mut data_removed = std::mem::take(&mut self.data_removed);
		if let Some(f) = self.on_removed {
			for entity in &entities {
				f(self, world, *entity, data_removed.remove(entity).unwrap());
			}
		}
	}
}

impl<Component: 'static> Storage<Component> for SimpleStorage<Component> {
	fn add(&mut self, entity: impl AsEntity, component: Component) {
		let entity = entity.as_entity();
		if self.has(entity) {
			#[cfg(debug_assertions)] {
				log::warn!("overwriting {:?} in entity {}", std::any::type_name::<Component>(), entity.0);
			}
			self.modified.insert(entity);
		} else {
			self.added.insert(entity);
		}
		self.data.insert(entity, component);
	}

	fn get(&self, entity: impl AsEntity) -> Option<&Component> {
		self.data.get(&entity.as_entity())
	}

	fn get_mut(&mut self, entity: impl AsEntity) -> Option<&mut Component> {
		let entity = entity.as_entity();
		let cmp = self.data.get_mut(&entity);
		if cmp.is_some() {
			self.modified.insert(entity);
		}
		cmp
	}

	fn get_removed(&self, entity: impl AsEntity) -> Option<&Component> {
		self.data_removed.get(&entity.as_entity())
	}

	fn get_removed_mut(&mut self, entity: impl AsEntity) -> Option<&mut Component> {
		self.data_removed.get_mut(&entity.as_entity())
	}

	fn take_removed(&mut self, entity: impl AsEntity) -> Option<Component> {
		let entity = entity.as_entity();
		self.removed.remove(&entity);
		self.data_removed.remove(&entity)
	}

	fn get_mut_or(&mut self, entity: impl AsEntity, f: impl FnOnce() -> Component) -> &mut Component {
		let entity = entity.as_entity();
		if self.has(entity) {
			self.modified.insert(entity);
		} else {
			self.add(entity, f());
		}
		self.data.get_mut(&entity).unwrap()
	}
}

pub struct StorageGuard<Component: 'static, Inner: std::ops::Deref<Target = SimpleStorage<Component>>> {
	pub inner: Inner,
	#[cfg(debug_assertions)]
	pub location: std::panic::Location<'static>,
}

pub struct StorageGuardMut<'a, Component: 'static, Inner: std::ops::DerefMut<Target = SimpleStorage<Component>>> {
	pub world: &'a World,
	pub inner: Option<Inner>,
	#[cfg(debug_assertions)]
	pub location: std::panic::Location<'static>,
}

unsafe impl<Component: 'static, Inner: std::ops::Deref<Target = SimpleStorage<Component>>> owning_ref::StableAddress for StorageGuard<Component, Inner> {}
unsafe impl<'a, Component: 'static, Inner: std::ops::DerefMut<Target = SimpleStorage<Component>>> owning_ref::StableAddress for StorageGuardMut<'a, Component, Inner> {}

impl<Component, Inner> std::ops::Deref for StorageGuard<Component, Inner> where
	Component: 'static,
	Inner: std::ops::Deref<Target = SimpleStorage<Component>>,
{
	type Target = SimpleStorage<Component>;

	fn deref(&self) -> &Self::Target { &self.inner }
}

#[cfg(debug_assertions)]
impl<Component, Inner> Drop for StorageGuard<Component, Inner> where
	Component: 'static,
	Inner: std::ops::Deref<Target = SimpleStorage<Component>>,
{
	fn drop(&mut self) {
		let type_id = std::any::TypeId::of::<Component>();

		crate::backtrace::STORAGE_MAP.0.borrow_mut()
			.entry(type_id)
			.and_modify(|map| {
				assert!(!map.values().any(|x| *x), 
					"Trying to drop immutably borrowed {} storage while a mutable borrow of it exists. {:#?}
					This is a bug in hobo, please report it at `https://github.com/GRDigital/hobo/issues`", 
					std::any::type_name::<Component>().to_owned(),
					crate::backtrace::STORAGE_MAP.0.borrow_mut().get(&type_id)
				);
				map.remove(&self.location);
			});
	}
}

impl<'a, Component, Inner> std::ops::Deref for StorageGuardMut<'a, Component, Inner> where
	Component: 'static,
	Inner: std::ops::DerefMut<Target = SimpleStorage<Component>>,
{
	type Target = SimpleStorage<Component>;

	fn deref(&self) -> &Self::Target { self.inner.as_ref().unwrap() }
}

impl<'a, Component, Inner> std::ops::DerefMut for StorageGuardMut<'a, Component, Inner> where
	Component: 'static,
	Inner: std::ops::DerefMut<Target = SimpleStorage<Component>>,
{
	fn deref_mut(&mut self) -> &mut Self::Target { self.inner.as_mut().unwrap() }
}

// dropping StorageGuard should trigger updates of relevant systems
// right now it's pooling all entities that were involved in changes, additions or removals
impl<'a, Component, Inner> Drop for StorageGuardMut<'a, Component, Inner> where
	Component: 'static,
	Inner: std::ops::DerefMut<Target = SimpleStorage<Component>>,
{
	#[cfg(debug_assertions)]
	fn drop(&mut self) {
		let StorageGuardMut { world, inner, location } = self;

		let type_id = std::any::TypeId::of::<Component>();
		let type_name = std::any::type_name::<Component>().to_owned();
		crate::backtrace::STORAGE_MAP.0.borrow_mut()
			.entry(type_id)
			.and_modify(|map| { 
				assert!(map.len() > 1, 
					"Trying to drop mutably borrowed {type_name} storage while more than 1 borrow of it exists. {:#?}
					This is a bug in hobo, please report it at `https://github.com/GRDigital/hobo/issues`", 
					crate::backtrace::STORAGE_MAP.0.borrow_mut().get(&type_id)
				);
				map.remove(location);
			});

		let storage = &mut *inner.take().unwrap();
		storage.flush(world);
	}

	#[cfg(not(debug_assertions))]
	fn drop(&mut self) {
		let StorageGuardMut { world, inner } = self;
		let storage = &mut *inner.take().unwrap();
		storage.flush(world);
	}
}
