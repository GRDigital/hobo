use crate::{prelude::default, AsEntity, Entity, World};
use std::collections::{HashMap, HashSet};

pub trait DynStorage: as_any::AsAny {
	fn dyn_has(&self, entity: Entity) -> bool;
	fn dyn_remove(&mut self, entity: Entity);
	fn flush(&mut self, world: &mut World);
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

	pub on_added: Option<fn(&mut SimpleStorage<Component>, &mut World, Entity)>,
	pub on_modified: Option<fn(&mut SimpleStorage<Component>, &mut World, Entity)>,
	pub on_removed: Option<fn(&mut SimpleStorage<Component>, &mut World, Entity, Component)>,
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

	fn flush(&mut self, world: &mut World) {
		for &added in &self.added {
			world.component_ownership.entry(added).or_default().insert(std::any::TypeId::of::<Component>());
		}

		for removed in &self.removed {
			if let Some(components) = world.component_ownership.get_mut(removed) {
				components.remove(&std::any::TypeId::of::<Component>());
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

pub struct StorageGuard<'a, Component: 'static, Inner: std::ops::DerefMut<Target = SimpleStorage<Component>>>(pub &'a mut World, pub Option<Inner>);
unsafe impl<'a, Component: 'static, Inner: std::ops::DerefMut<Target = SimpleStorage<Component>>> owning_ref::StableAddress for StorageGuard<'a, Component, Inner> {}

impl<'a, Component, Inner> std::ops::Deref for StorageGuard<'a, Component, Inner> where
	Component: 'static,
	Inner: std::ops::DerefMut<Target = SimpleStorage<Component>>,
{
	type Target = SimpleStorage<Component>;

	fn deref(&self) -> &Self::Target { self.1.as_ref().unwrap() }
}

impl<'a, Component, Inner> std::ops::DerefMut for StorageGuard<'a, Component, Inner> where
	Component: 'static,
	Inner: std::ops::DerefMut<Target = SimpleStorage<Component>>,
{
	fn deref_mut(&mut self) -> &mut Self::Target { self.1.as_mut().unwrap() }
}

// dropping StorageGuard should trigger updates of relevant systems
// right now it's pooling all entities that were involved in changes, additions or removals
impl<'a, Component, Inner> Drop for StorageGuard<'a, Component, Inner> where
	Component: 'static,
	Inner: std::ops::DerefMut<Target = SimpleStorage<Component>>,
{
	fn drop(&mut self) {
		let StorageGuard(world, inner) = self;
		let storage = &mut *inner.take().unwrap();
		storage.flush(world);
	}
}
