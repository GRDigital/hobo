#![allow(unused_variables)]

use crate::prelude::*;
use std::collections::BTreeSet;
use owning_ref::{OwningRef, OwningRefMut, OwningHandle};
use crate::{StorageRef, StorageRefMut};

pub trait Query {
	type Fetch;

	fn filter(world: &mut World, entities: &mut BTreeSet<Entity>) {}
	fn populate(world: &mut World) -> BTreeSet<Entity> { BTreeSet::new() }
	fn fetch(world: &mut World, entity: Entity) -> Self::Fetch;
}

impl Query for Entity {
	type Fetch = Entity;

	fn fetch(world: &mut World, entity: Entity) -> Self::Fetch { entity }
}

// same search as &Component, but fetch is a noop
pub struct With<Component: 'static>(std::marker::PhantomData<Component>);
impl<Component: 'static> Query for With<Component> {
	type Fetch = ();

	fn filter(world: &mut World, entities: &mut BTreeSet<Entity>) { <&Component as Query>::filter(world, entities); }
	fn populate(world: &mut World) -> BTreeSet<Entity> { <&Component as Query>::populate(world) }
	fn fetch(world: &mut World, entity: Entity) -> Self::Fetch {}
}

impl<Component: 'static> Query for &Component {
	type Fetch = OwningRef<Box<dyn owning_ref::Erased>, Component>;

	fn filter(world: &mut World, entities: &mut BTreeSet<Entity>) {
		let storage = world.storage::<Component>();
		entities.retain(|entity| storage.has(entity));
	}
	fn populate(world: &mut World) -> BTreeSet<Entity> {
		let storage = world.storage::<Component>();
		storage.data.keys().copied().collect()
	}
	fn fetch(world: &mut World, entity: Entity) -> Self::Fetch {
		let storage: StorageRef<Component> = OwningRef::new(OwningHandle::new(world.dyn_storage::<Component>()))
			.map(|x| x.as_any().downcast_ref().unwrap());

		storage
			.map(|x| x.get(entity).unwrap())
			.map_owner_box().erase_owner()
	}
}

impl<Component: 'static> Query for &mut Component {
	type Fetch = OwningRefMut<Box<dyn owning_ref::Erased>, Component>;

	fn filter(world: &mut World, entities: &mut BTreeSet<Entity>) { <&Component as Query>::filter(world, entities); }
	fn populate(world: &mut World) -> BTreeSet<Entity> { <&Component as Query>::populate(world) }
	fn fetch(world: &mut World, entity: Entity) -> Self::Fetch {
		let storage: StorageRefMut<Component> = OwningRefMut::new(OwningHandle::new_mut(world.dyn_storage::<Component>()))
			.map_mut(|x| x.as_any_mut().downcast_mut().unwrap());

		storage
			.map_mut(|x| x.get_mut(entity).unwrap())
			.map_owner_box().erase_owner()
	}
}

macro_rules! impl_for_tuples {
	(($($_:ident),*)) => {};
	(($first:ident, $($old:ident),*) $curr:ident $($rest:tt)*) => {
		impl<$first: Query, $($old: Query,)* $curr: Query> Query for ($first, $($old,)* $curr) {
			type Fetch = ($first::Fetch, $($old::Fetch,)* $curr::Fetch);

			fn filter(world: &mut World, entities: &mut BTreeSet<Entity>) {
				*entities = Self::populate(world);
				$($old::filter(world, entities);)*
				$curr::filter(world, entities);
			}
			fn populate(world: &mut World) -> BTreeSet<Entity> {
				$first::populate(world)
			}
			fn fetch(world: &mut World, entity: Entity) -> Self::Fetch {
				($first::fetch(world, entity), $($old::fetch(world, entity),)* $curr::fetch(world, entity))
			}
		}

		impl_for_tuples![($first, $($old,)* $curr) $($rest)*];
	};
	($first:ident $($rest:tt)*) => {
		impl_for_tuples![($first, ) $($rest)*];
	};
}

impl_for_tuples![A B C D E F G H I J K L M N O P Q R S T U V W X Y Z];
