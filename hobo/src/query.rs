//! Queries allow finding entities across the hierarchy
//! # Examples
//!
//! ```
//! struct Foo {
//! 	// some fields
//! }
//!
//! // find the first (presumably only) entity with some component Foo
//! let (entity, _) = hobo::find_one::<(Entity, With<Foo>)>();
//! let element = SomeElement(entity);
//! element.set_text("This entity has Foo");
//! ```
//!
//! ```
//! struct Frobnicator {
//! 	num_fraculations: u32,
//! 	// other fields
//! }
//!
//! // find all entities with a Frobnicator component and mutate it
//! // perhaps as a result of some combined transformation
//! for frobnicator in hobo::find::<&mut Frobnicator>() {
//! 	frobnicator.num_fraculations += 1;
//! }
//! ```

#![allow(unused_variables)]

use crate::{prelude::*, StorageRef, StorageRefMut};
use owning_ref::{OwningHandle, OwningRef, OwningRefMut};
use std::collections::BTreeSet;

pub trait Query {
	type Fetch;

	// either populate `entities` if it's None or filter out all entities not satisfying the predicate (like having a particular component)
	fn filter(world: &World, entities: &mut Option<BTreeSet<Entity>>) {}
	fn fetch(world: &World, entity: Entity) -> Self::Fetch;
}

impl Query for Entity {
	type Fetch = Entity;

	fn fetch(world: &World, entity: Entity) -> Self::Fetch { entity }
}

// same search as &Component, but fetch is a noop
pub struct With<Component: 'static>(std::marker::PhantomData<Component>);
impl<Component: 'static> Query for With<Component> {
	type Fetch = ();

	fn filter(world: &World, entities: &mut Option<BTreeSet<Entity>>) { <&Component as Query>::filter(world, entities); }

	fn fetch(world: &World, entity: Entity) -> Self::Fetch {}
}

impl<Component: 'static> Query for &Component {
	type Fetch = OwningRef<Box<dyn owning_ref::Erased>, Component>;

	fn filter(world: &World, entities: &mut Option<BTreeSet<Entity>>) {
		let storage = world.storage::<Component>();
		if let Some(entities) = entities {
			entities.retain(|entity| storage.has(entity));
		} else {
			*entities = Some(storage.data.keys().copied().collect());
		}
	}

	fn fetch(world: &World, entity: Entity) -> Self::Fetch {
		let storage: StorageRef<Component> = OwningRef::new(world.dyn_storage::<Component>())
			.map(|x| x.as_any().downcast_ref().unwrap());

		storage
			.map(|x| x.get(entity).unwrap())
			.map_owner_box().erase_owner()
	}
}

impl<Component: 'static> Query for &mut Component {
	type Fetch = OwningRefMut<Box<dyn owning_ref::Erased>, Component>;

	fn filter(world: &World, entities: &mut Option<BTreeSet<Entity>>) { <&Component as Query>::filter(world, entities); }

	fn fetch(world: &World, entity: Entity) -> Self::Fetch {
		let storage: StorageRefMut<Component> = OwningRefMut::new(world.dyn_storage::<Component>())
			.map_mut(|x| x.as_any_mut().downcast_mut().unwrap());

		storage
			.map_mut(|x| x.get_mut(entity).unwrap())
			.map_owner_box().erase_owner()
	}
}

macro_rules! impl_for_tuples {
	(($($_:ident),*)) => {};
	(($($old:ident),*) $curr:ident $($rest:tt)*) => {
		impl<$($old: Query,)* $curr: Query> Query for ($($old,)* $curr) {
			type Fetch = ($($old::Fetch,)* $curr::Fetch);

			fn filter(world: &World, entities: &mut Option<BTreeSet<Entity>>) {
				$($old::filter(world, entities);)*
				$curr::filter(world, entities);
			}
			fn fetch(world: &World, entity: Entity) -> Self::Fetch {
				($($old::fetch(world, entity),)* $curr::fetch(world, entity))
			}
		}

		impl_for_tuples![($($old,)* $curr) $($rest)*];
	};
	($first:ident $($rest:tt)*) => {
		impl_for_tuples![($first) $($rest)*];
	};
}

impl_for_tuples![A B C D E F G H I J K L M N O P Q R S T U V W X Y Z];
