use super::*;

pub trait Query {
	fn query(world: &World, entity: Entity) -> bool { false }
	fn added(world: &World, entity: Entity) -> bool { false }
	fn modified(world: &World, entity: Entity) -> bool { false }
	fn removed(world: &World, entity: Entity) -> bool { false }
}

macro_rules! tuple_query {
	() => {};
	($first:ident $($id:ident)*) => {
		paste::item! {
			impl<$first: 'static, $($id: 'static),*> Query for ($first, $($id),*) {
				fn query(world: &World, entity: Entity) -> bool {
					world.storage::<$first>().has(entity) &&
					$(world.storage::<$id>().has(entity) &&)*
					true
				}

				fn added(world: &World, entity: Entity) -> bool {
					world.storage::<$first>().added.contains(&entity) &&
					$(world.storage::<$id>().added.contains(&entity) &&)*
					true
				}

				fn modified(world: &World, entity: Entity) -> bool {
					world.storage::<$first>().modified.contains(&entity) &&
					$(world.storage::<$id>().modified.contains(&entity) &&)*
					true
				}

				fn removed(world: &World, entity: Entity) -> bool {
					world.storage::<$first>().removed.contains(&entity) ||
					$(world.storage::<$id>().removed.contains(&entity) ||)*
					false
				}
			}
		}
		tuple_query! {$($id)*}
	};
}

tuple_query! {A B C D E F G H I J K L M N O P Q R S T U V W X Y Z}

pub trait Added<T: Query + ?Sized> {}
impl<T: Query + ?Sized> Query for dyn Added<T> {
	fn query(world: &World, entity: Entity) -> bool { T::added(world, entity) }
}

pub trait Removed<T: Query + ?Sized> {}
impl<T: Query + ?Sized> Query for dyn Removed<T> {
	fn query(world: &World, entity: Entity) -> bool { T::removed(world, entity) }
}

pub trait Modified<T: Query + ?Sized> {}
impl<T: Query + ?Sized> Query for dyn Modified<T> {
	fn query(world: &World, entity: Entity) -> bool { T::modified(world, entity) }
}
