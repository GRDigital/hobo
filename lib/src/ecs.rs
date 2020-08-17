use crate::prelude::*;
use std::collections::{HashMap, BTreeMap, BTreeSet, HashSet};
use std::any::{Any, TypeId};
use std::rc::Rc;
use once_cell::sync::Lazy;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Entity(u64);

trait Storage<'a, Component: 'static> {
	type Item;

	fn add(&mut self, entity: Entity, component: Component);
	fn get<'b: 'a>(&'b mut self, entity: Entity) -> Option<Self::Item>;
	fn remove(&mut self, entity: Entity);
}

struct SimpleStorage<Component: 'static> {
	data: HashMap<Entity, Component>,
	added: HashSet<Entity>,
	removed: HashSet<Entity>,
	modified: HashSet<Entity>,
}

impl<Component> Default for SimpleStorage<Component> {
	fn default() -> Self {
		Self { data: Default::default(), added: Default::default(), removed: Default::default(), modified: Default::default() }
	}
}

impl<'a, Component: 'static> Storage<'a, Component> for SimpleStorage<Component> {
	type Item = &'a mut Component;

	fn add(&mut self, entity: Entity, component: Component) {
		self.added.insert(entity);
		self.data.insert(entity, component);
	}

	fn get<'b: 'a>(&'b mut self, entity: Entity) -> Option<Self::Item> {
		self.modified.insert(entity);
		self.data.get_mut(&entity)
	}

	fn remove(&mut self, entity: Entity) {
		self.removed.insert(entity);
		self.data.remove(&entity);
	}
}

macro_rules! tuple_storage {
	($_1:lifetime $_2:ident) => {};
	($first_lt:lifetime $first_id:ident, $($lt:lifetime $id:ident),*) => {
		paste::item! {
			impl<'item, $first_lt, $($lt),*, [<$first_id C>], $([<$id C>]),*, [<$first_id S>], $([<$id S>]),*> Storage<'item, ([<$first_id C>], $([<$id C>]),*)> for (&$first_lt mut [<$first_id S>], $(&$lt mut [<$id S>]),*) where
				[<$first_id C>]: 'static, [<$first_id S>]: Storage<'item, [<$first_id C>]> + Copy,
				$([<$id C>]: 'static, [<$id S>]: Storage<'item, [<$id C>]> + Copy),*
			{
				type Item = ([<$first_id S>]::Item, $([<$id S>]::Item),*);

				fn add(&mut self, entity: Entity, component: ([<$first_id C>], $([<$id C>]),*)) {
					let ([<s_ $first_id:snake>], $([<s_ $id:snake>]),*) = self;
					let ([<c_ $first_id:snake>], $([<c_ $id:snake>]),*) = component;
					[<s_ $first_id:snake>].add(entity, [<c_ $first_id:snake>]);
					$([<s_ $id:snake>].add(entity, [<c_ $id:snake>]);)*
				}

				fn get<'storage: 'item>(&'storage mut self, entity: Entity) -> Option<Self::Item> {
					let ([<s_ $first_id:snake>], $([<s_ $id:snake>]),*) = self;
					Some(([<s_ $first_id:snake>].get(entity)?, $([<s_ $id:snake>].get(entity)?),*))
				}

				fn remove(&mut self, entity: Entity) {
					let ([<s_ $first_id:snake>], $([<s_ $id:snake>]),*) = self;
					[<s_ $first_id:snake>].remove(entity);
					$([<s_ $id:snake>].remove(entity);)*
				}
			}
		}
		tuple_storage! {$($lt $id),*}
	};
}

tuple_storage!('a A, 'b B, 'c C, 'd D, 'e E, 'f F, 'g G, 'h H, 'i I, 'j J, 'k K, 'l L, 'm M, 'n N, 'o O, 'p P, 'q Q, 'r R, 's S, 't T, 'u U, 'v V, 'w W, 'x X, 'y Y, 'z Z);

impl<'item, 'z, ZC, ZS> Storage<'item, (ZC,)> for (&'z mut ZS,) where
	ZC: 'static,
	ZS: Storage<'item, ZC> + Copy,
{
	type Item = (ZS::Item,);

	fn add(&mut self, entity: Entity, component: (ZC,)) { self.0.add(entity, component.0) }
	fn get<'storage: 'item>(&'storage mut self, entity: Entity) -> Option<Self::Item> { Some((self.0.get(entity)?,)) }
	fn remove(&mut self, entity: Entity) { self.0.remove(entity) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ComponentInterest {
	// a set of TypeId, run system if become true as a result of some component insertion
	// i.e. if any component's TypeId is missing - disqualified
	EnteredSet(HashSet<TypeId>),
	// a set of TypeId, run system if become true as a result of some component removal
	// i.e. if any component's TypeId is in set - disqualified
	LeftSet(HashSet<TypeId>),
	// a set of TypeId, run system if any component in the first set was mutated,
	// the component of the second set must be present
	Mutated(HashSet<TypeId>, HashSet<TypeId>),
}

struct System {
	f: fn(&mut World, Entity),
	// interests: HashSet<ComponentInterest>,
}

macro_rules! system {
	($($tt:tt)*) => {{
		static SYSTEM: Lazy<Arc<System>> = Lazy::new(|| Arc::new($($tt)*));

		Arc::clone(&SYSTEM as &Arc<_>)
	}};
}

// systems register entities they care about upon creation
// identical systems have their entities merged
#[derive(Default)]
struct World {
	next_entity_id: u64,
	storages: HashMap<TypeId, Box<dyn Any>>,
	systems: HashMap<Entity, Vec<Arc<System>>>,
}

struct StorageGuard<'a, Component: 'static>(&'a World, std::marker::PhantomData<Component>);

impl<'a, Component: 'static> std::convert::AsMut<SimpleStorage<Component>> for StorageGuard<'a, Component> {
	fn as_mut(&mut self) -> &mut SimpleStorage<Component> {
		self.0.storages
			.entry(TypeId::of::<SimpleStorage<Component>>())
			.or_insert_with(|| Box::new(SimpleStorage::<Component>::default()))
			.downcast_mut::<SimpleStorage<Component>>().unwrap()
	}
}

impl<'a, Component: 'static> Drop for StorageGuard<'a, Component> {
	fn drop(&mut self) {
		// TODO: maintain world
	}
}

impl World {
	fn storage<Component: 'static>(&self) -> StorageGuard<Component> {
		StorageGuard(self, std::marker::PhantomData)
	}

	fn new_entity(&mut self) -> Entity {
		let entity = Entity(self.next_entity_id);
		self.next_entity_id += 1;
		entity
	}

	// fn new_system
}

fn fuck() {
	let mut world = World::default();
	let entity = world.new_entity();

	let sys = system!(System {
		f: |world, entity| {},
	});

	world.systems.entry(entity).or_insert_with(Vec::new).push(sys);
}
