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
		self.added.insert(entity);
		self.data.insert(entity, component);
	}

	fn get(&self, entity: Entity) -> Option<&Component> {
		self.data.get(&entity)
	}

	fn get_mut(&mut self, entity: Entity) -> Option<&mut Component> {
		self.modified.insert(entity);
		self.data.get_mut(&entity)
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

		let systems = world.schedule_systems(set);

		{
			let mut storages = world.storages.borrow_mut();
			let mut storage = storages.get_mut(&TypeId::of::<SimpleStorage<Component>>()).unwrap().borrow_mut();
			storage.flush();
		}

		world.run_systems(systems);
	}
}

// macro_rules! tuple_storage {
//     ($_1:lifetime $_2:ident) => {};
//     ($first_lt:lifetime $first_id:ident, $($lt:lifetime $id:ident),*) => {
//         paste::item! {
//             impl<'item, $first_lt, $($lt),*, [<$first_id C>], $([<$id C>]),*, [<$first_id S>], $([<$id S>]),*> Storage<'item, ([<$first_id C>], $([<$id C>]),*)> for (&$first_lt mut [<$first_id S>], $(&$lt mut [<$id S>]),*) where
//                 [<$first_id C>]: 'static, [<$first_id S>]: Storage<'item, [<$first_id C>]> + Copy,
//                 $([<$id C>]: 'static, [<$id S>]: Storage<'item, [<$id C>]> + Copy),*
//             {
//                 type Item = ([<$first_id S>]::Item, $([<$id S>]::Item),*);

//                 fn add(&mut self, entity: Entity, component: ([<$first_id C>], $([<$id C>]),*)) {
//                     let ([<s_ $first_id:snake>], $([<s_ $id:snake>]),*) = self;
//                     let ([<c_ $first_id:snake>], $([<c_ $id:snake>]),*) = component;
//                     [<s_ $first_id:snake>].add(entity, [<c_ $first_id:snake>]);
//                     $([<s_ $id:snake>].add(entity, [<c_ $id:snake>]);)*
//                 }

//                 fn get<'storage: 'item>(&'storage mut self, entity: Entity) -> Option<Self::Item> {
//                     let ([<s_ $first_id:snake>], $([<s_ $id:snake>]),*) = self;
//                     Some(([<s_ $first_id:snake>].get(entity)?, $([<s_ $id:snake>].get(entity)?),*))
//                 }

//                 fn remove(&mut self, entity: Entity) {
//                     let ([<s_ $first_id:snake>], $([<s_ $id:snake>]),*) = self;
//                     [<s_ $first_id:snake>].remove(entity);
//                     $([<s_ $id:snake>].remove(entity);)*
//                 }
//             }
//         }
//         tuple_storage! {$($lt $id),*}
//     };
// }

// tuple_storage!('a A, 'b B, 'c C, 'd D, 'e E, 'f F, 'g G, 'h H, 'i I, 'j J, 'k K, 'l L, 'm M, 'n N, 'o O, 'p P, 'q Q, 'r R, 's S, 't T, 'u U, 'v V, 'w W, 'x X, 'y Y, 'z Z);

// impl<'item, 'z, ZC, ZS> Storage<'item, (ZC,)> for (&'z mut ZS,) where
//     ZC: 'static,
//     ZS: Storage<'item, ZC> + Copy,
// {
//     type Item = (ZS::Item,);

//     fn add(&mut self, entity: Entity, component: (ZC,)) { self.0.add(entity, component.0) }
//     fn get<'storage: 'item>(&'storage mut self, entity: Entity) -> Option<Self::Item> { Some((self.0.get(entity)?,)) }
//     fn remove(&mut self, entity: Entity) { self.0.remove(entity) }
// }
