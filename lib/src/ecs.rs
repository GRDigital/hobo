use crate::prelude::*;
use std::collections::{HashMap, BTreeMap, BTreeSet};
use std::any::{Any, TypeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Entity(u64);

trait Storage<Component> {
	type Item;

	fn add(self, entity: Entity, component: Component);
	fn get(self, entity: Entity) -> Option<Self::Item>;
	fn remove(self, entity: Entity);
}

struct SimpleStorage<Component> {
	data: BTreeMap<Entity, Component>
}

impl<Component> SimpleStorage<Component> {
	fn new() -> Self {
		Self { data: BTreeMap::new() }
	}
}

impl<'a, Component> Storage<Component> for &'a mut SimpleStorage<Component> {
	type Item = &'a mut Component;

	fn add(self, entity: Entity, component: Component) {
		self.data.insert(entity, component);
	}

	fn get(self, entity: Entity) -> Option<Self::Item> {
		self.data.get_mut(&entity)
	}

	fn remove(self, entity: Entity) {
		self.data.remove(&entity);
	}
}

impl<'a, C1> Storage<(C1,)> for (&'a mut SimpleStorage<C1>,) {
	type Item = (&'a mut C1,);

	fn add(self, entity: Entity, component: (C1,)) { self.0.add(entity, component.0) }
	fn get(self, entity: Entity) -> Option<Self::Item> { Some((self.0.get(entity)?,)) }
	fn remove(self, entity: Entity) { self.0.remove(entity) }
}

impl<'a, C1, C2> Storage<(C1, C2)> for (&'a mut SimpleStorage<C1>, &'a mut SimpleStorage<C2>) {
	type Item = (&'a mut C1, &'a mut C2);

	fn add(self, entity: Entity, component: (C1, C2)) { self.0.add(entity, component.0); self.1.add(entity, component.1) }
	fn get(self, entity: Entity) -> Option<Self::Item> { Some((self.0.get(entity)?,self.1.get(entity)?)) }
	fn remove(self, entity: Entity) { self.0.remove(entity); self.1.remove(entity) }
}

// struct ComponentIter<'a, S> {
//     entities: BTreeSet<Entity>,
//     storage: S,
// }

// trait IntoComponentIter

// trait IntoComponentIter<'a> {
//     type Item;

//     fn iter(self) -> std::vec::IntoIter<Self::Item>;
// }

// impl<'a, C1, C2> ComponentIter<'a> for (&'a mut Storage<C1>, &'a mut Storage<C2>) {
//     type Item = (&'a mut C1, &'a mut C2);

//     fn iter(self) -> std::vec::IntoIter<Self::Item> {
//         let intersection = self.0.data.keys().copied().collect::<BTreeSet<_>>().intersection(&self.1.data.keys().copied().collect()).copied();
//         intersection
//             .map(|entity| (self.0.get(entity).unwrap(), self.1.get(entity).unwrap()))
//             .collect::<Vec<_>>().into_iter()
//         // todo!()
//     }
// }

struct World {
	next_entity_id: u64,
	storages: BTreeMap<TypeId, Box<dyn Any>>,
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
}
