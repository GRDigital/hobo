use crate::prelude::*;
use std::collections::{HashMap, BTreeMap, BTreeSet, HashSet};
use std::any::{Any, TypeId};
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Entity(u64);

trait Storage<'a, Component: 'static> {
	type Item;

	fn add(&mut self, entity: Entity, component: Component);
	fn get<'b: 'a>(&'b mut self, entity: Entity) -> Option<Self::Item>;
	fn remove(&mut self, entity: Entity);
}

struct SimpleStorage<Component: 'static> {
	data: HashMap<Entity, Component>
}

impl<Component> SimpleStorage<Component> {
	fn new() -> Self {
		Self { data: HashMap::new() }
	}
}

impl<'a, Component: 'static> Storage<'a, Component> for SimpleStorage<Component> {
	type Item = &'a mut Component;

	fn add(&mut self, entity: Entity, component: Component) {
		self.data.insert(entity, component);
	}

	fn get<'b: 'a>(&'b mut self, entity: Entity) -> Option<Self::Item> {
		self.data.get_mut(&entity)
	}

	fn remove(&mut self, entity: Entity) {
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

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct System {
//     f: fn(&mut World),
//     // interests: HashSet<ComponentInterest>,
// }

// systems register entities they care about upon creation
// identical systems have their entities merged
struct World {
	next_entity_id: u64,
	storages: HashMap<TypeId, Box<dyn Any>>,
	// systems: HashMap<Entity, Vec<Rc<System>>>,
}

impl World {
	fn storage<Component: 'static>(&mut self) -> &mut SimpleStorage<Component> {
		self.storages
			.entry(TypeId::of::<Component>())
			.or_insert_with(|| Box::new(<SimpleStorage<Component>>::new()))
			.downcast_mut::<SimpleStorage<Component>>().unwrap()
	}

	fn new_entity(&mut self) -> Entity {
		let entity = Entity(self.next_entity_id);
		self.next_entity_id += 1;
		entity
	}

	// fn new_system
}
