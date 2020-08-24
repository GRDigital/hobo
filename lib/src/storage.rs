use super::*;

pub trait DynStorage: as_any::AsAny {
	fn has(&self, entity: Entity) -> bool;
	fn remove(&mut self, entity: Entity);
	fn flush(&mut self);
}

pub trait Storage<Component: 'static>: DynStorage {
	type Item;

	fn get(&self, entity: Entity) -> Option<&Self::Item>;
	fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Item>;
	fn get_removed(&self, entity: Entity) -> Option<&Self::Item>;
	fn get_removed_mut(&mut self, entity: Entity) -> Option<&mut Self::Item>;
	fn take_removed(&mut self, entity: Entity) -> Option<Self::Item>;
	fn get_mut_or(&mut self, entity: Entity, f: impl FnOnce() -> Self::Item) -> &mut Self::Item;
	fn add(&mut self, entity: Entity, component: Component);
}

pub struct SimpleStorage<Component: 'static> {
	pub data: HashMap<Entity, Component>,
	pub added: HashSet<Entity>,
	pub modified: HashSet<Entity>,
	pub removed: HashMap<Entity, Component>,
}

impl<Component> Default for SimpleStorage<Component> {
	fn default() -> Self {
		Self { data: Default::default(), added: Default::default(), removed: Default::default(), modified: Default::default() }
	}
}

impl<Component: 'static> DynStorage for SimpleStorage<Component> {
	fn has(&self, entity: Entity) -> bool {
		self.data.contains_key(&entity)
	}

	fn remove(&mut self, entity: Entity) {
		if let Some(x) = self.data.remove(&entity) {
			self.removed.insert(entity, x);
		}
	}

	fn flush(&mut self) {
		std::mem::take(&mut self.added);
		std::mem::take(&mut self.modified);
		std::mem::take(&mut self.removed);
	}
}

impl<Component: 'static> Storage<Component> for SimpleStorage<Component> {
	type Item = Component;

	fn add(&mut self, entity: Entity, component: Component) {
		if self.has(entity) {
			self.modified.insert(entity);
		} else {
			self.added.insert(entity);
		}
		self.data.insert(entity, component);
	}

	fn get(&self, entity: Entity) -> Option<&Component> {
		self.data.get(&entity)
	}

	fn get_removed(&self, entity: Entity) -> Option<&Component> {
		self.removed.get(&entity)
	}

	fn get_mut(&mut self, entity: Entity) -> Option<&mut Component> {
		self.modified.insert(entity);
		self.data.get_mut(&entity)
	}

	fn get_removed_mut(&mut self, entity: Entity) -> Option<&mut Component> {
		self.removed.get_mut(&entity)
	}

	fn take_removed(&mut self, entity: Entity) -> Option<Component> {
		self.removed.remove(&entity)
	}

	fn get_mut_or(&mut self, entity: Entity, f: impl FnOnce() -> Component) -> &mut Component {
		if self.has(entity) {
			self.modified.insert(entity);
		} else {
			self.add(entity, f());
		}
		self.data.get_mut(&entity).unwrap()
	}
}

pub struct StorageGuard<'a, Component: 'static, Inner: std::ops::DerefMut<Target = SimpleStorage<Component>>>(pub &'a World, pub Option<Inner>);

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
		drop(inner.take());

		let set = {
			let mut storages = world.storages.borrow_mut();
			let mut storage = storages.get_mut(&TypeId::of::<SimpleStorage<Component>>()).unwrap().borrow_mut();
			let storage = storage.as_any_mut().downcast_mut::<SimpleStorage<Component>>().unwrap();
			storage.added.iter().chain(storage.modified.iter()).chain(storage.removed.keys()).copied().collect::<HashSet<_>>()
		};

		let systems = world.schedule_systems(set.into_iter().map(|entity| (entity, TypeId::of::<Component>())));

		{
			let mut storages = world.storages.borrow_mut();
			let mut storage = storages.get_mut(&TypeId::of::<SimpleStorage<Component>>()).unwrap().borrow_mut();
			storage.flush();
		}

		world.run_systems(systems);
	}
}
