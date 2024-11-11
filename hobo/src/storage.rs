use crate::{prelude::default, AsEntity, Entity};
use std::collections::{HashMap, HashSet};

pub trait DynStorage: as_any::AsAny {
	fn dyn_has(&self, entity: Entity) -> bool;
	fn dyn_remove(&mut self, entity: Entity);
	fn flush(&mut self);
}

#[allow(clippy::type_complexity)]
pub struct Storage<Component: 'static> {
	pub data: HashMap<Entity, Component>,

	pub modified: HashSet<Entity>,

	pub on_added: Option<fn(&mut Storage<Component>, Entity)>,
	pub on_modified: Option<fn(&mut Storage<Component>, Entity)>,
	pub on_removed: Option<fn(&mut Storage<Component>, Entity, Component)>,
}

impl<Component> Default for Storage<Component> {
	fn default() -> Self {
		Self {
			data: default(),

			modified: default(),

			on_added: default(),
			on_modified: default(),
			on_removed: default(),
		}
	}
}

impl<Component: 'static> DynStorage for Storage<Component> {
	fn dyn_has(&self, entity: Entity) -> bool {
		self.data.contains_key(&entity)
	}

	fn dyn_remove(&mut self, entity: Entity) {
		if let Some(cmp) = self.data.remove(&entity) {
			if let Some(on_removed) = self.on_removed {
				if let Some(ownership) = crate::WORLD.component_ownership.borrow_mut().get_mut(&entity) {
					ownership.remove(&std::any::TypeId::of::<Component>());
				}
				on_removed(self, entity, cmp);
			}
		}
	}

	// On *storage drop* (not component), trigger modified callbacks
	fn flush(&mut self) {
		let entities = std::mem::take(&mut self.modified);
		if let Some(f) = self.on_modified {
			for &entity in &entities {
				f(self, entity);
			}
		}
	}
}

impl<Component: 'static> Storage<Component> {
	pub fn has(&self, entity: impl AsEntity) -> bool { self.dyn_has(entity.as_entity()) }
	pub fn remove(&mut self, entity: impl AsEntity) { self.dyn_remove(entity.as_entity()) }
	pub fn get_mut_or_default(&mut self, entity: impl AsEntity) -> &mut Component where Component: Default { self.get_mut_or(entity, Component::default) }

	pub fn add(&mut self, entity: impl AsEntity, component: Component) {
		let entity = entity.as_entity();
		let already_present = self.has(entity);

		self.data.insert(entity, component);

		if already_present {
			if cfg!(debug_assertions) { log::warn!("overwriting {:?} in entity {}", std::any::type_name::<Component>(), entity.0); }
			if let Some(on_modified) = self.on_modified { on_modified(self, entity); }
		} else {
			crate::WORLD.component_ownership.borrow_mut().get_mut(&entity).unwrap().insert(std::any::TypeId::of::<Component>());
			if let Some(on_added) = self.on_added { on_added(self, entity); }
		}
	}

	pub fn get(&self, entity: impl AsEntity) -> Option<&Component> {
		self.data.get(&entity.as_entity())
	}

	pub fn get_mut(&mut self, entity: impl AsEntity) -> Option<&mut Component> {
		let entity = entity.as_entity();
		let cmp = self.data.get_mut(&entity);
		if cmp.is_some() {
			self.modified.insert(entity);
		}
		cmp
	}

	pub fn get_mut_or(&mut self, entity: impl AsEntity, f: impl FnOnce() -> Component) -> &mut Component {
		let entity = entity.as_entity();
		if !self.has(entity) {
			crate::WORLD.component_ownership.borrow_mut().get_mut(&entity).unwrap().insert(std::any::TypeId::of::<Component>());
			self.data.insert(entity, f());
		}
		self.modified.insert(entity);
		self.data.get_mut(&entity).unwrap()
	}
}

pub struct StorageGuard<Component: 'static, Inner: std::ops::Deref<Target = Storage<Component>>> {
	pub inner: Inner,
	#[cfg(debug_assertions)]
	pub location: std::panic::Location<'static>,
}

pub struct StorageGuardMut<Component: 'static, Inner: std::ops::DerefMut<Target = Storage<Component>>> {
	pub inner: Option<Inner>,
	#[cfg(debug_assertions)]
	pub location: std::panic::Location<'static>,
}

unsafe impl<Component: 'static, Inner: std::ops::Deref<Target = Storage<Component>>> owning_ref::StableAddress for StorageGuard<Component, Inner> {}
unsafe impl<Component: 'static, Inner: std::ops::DerefMut<Target = Storage<Component>>> owning_ref::StableAddress for StorageGuardMut<Component, Inner> {}

impl<Component, Inner> std::ops::Deref for StorageGuard<Component, Inner> where
	Component: 'static,
	Inner: std::ops::Deref<Target = Storage<Component>>,
{
	type Target = Storage<Component>;

	fn deref(&self) -> &Self::Target { &self.inner }
}

#[cfg(debug_assertions)]
impl<Component, Inner> Drop for StorageGuard<Component, Inner> where
	Component: 'static,
	Inner: std::ops::Deref<Target = Storage<Component>>,
{
	fn drop(&mut self) {
		let type_id = std::any::TypeId::of::<Component>();

		crate::backtrace::STORAGE_MAP.0.borrow_mut()
			.entry(type_id)
			.and_modify(|map| {
				assert!(map.values().all(|mutable| !mutable),
					"Trying to drop immutably borrowed {} storage while a mutable borrow of it exists.
					\n{map}
					\nThis is a bug in hobo, please report it at `https://github.com/zygomedia/hobo/issues`",
					std::any::type_name::<Component>().to_owned(),
				);
				map.remove(&self.location);
			});
	}
}

impl<Component, Inner> std::ops::Deref for StorageGuardMut<Component, Inner> where
	Component: 'static,
	Inner: std::ops::DerefMut<Target = Storage<Component>>,
{
	type Target = Storage<Component>;

	fn deref(&self) -> &Self::Target { self.inner.as_ref().unwrap() }
}

impl<Component, Inner> std::ops::DerefMut for StorageGuardMut<Component, Inner> where
	Component: 'static,
	Inner: std::ops::DerefMut<Target = Storage<Component>>,
{
	fn deref_mut(&mut self) -> &mut Self::Target { self.inner.as_mut().unwrap() }
}

// dropping StorageGuard should trigger updates of relevant systems
// right now it's pooling all entities that were involved in changes, additions or removals
impl<Component, Inner> Drop for StorageGuardMut<Component, Inner> where
	Component: 'static,
	Inner: std::ops::DerefMut<Target = Storage<Component>>,
{
	#[cfg(debug_assertions)]
	fn drop(&mut self) {
		let StorageGuardMut { inner, location } = self;
		let type_id = std::any::TypeId::of::<Component>();

		crate::backtrace::STORAGE_MAP.0.borrow_mut()
			.entry(type_id)
			.and_modify(|map| {
				assert!(map.len() <= 1,
					"Trying to drop mutably borrowed {} storage while more than 1 borrow of it exists.
					\n{map}
					\nThis is a bug in hobo, please report it at `https://github.com/zygomedia/hobo/issues`",
					std::any::type_name::<Component>().to_owned(),
				);
				map.remove(location);
			});

		let storage = &mut *inner.take().unwrap();
		storage.flush();
	}

	#[cfg(not(debug_assertions))]
	fn drop(&mut self) {
		let StorageGuardMut { inner } = self;
		let storage = &mut *inner.take().unwrap();
		storage.flush();
	}
}
