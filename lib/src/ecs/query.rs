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
					// world.storage::<$first>().0.borrow().downcast_ref::<SimpleStorage<$first>>().map_or(false, |x| x.data.contains_key(&entity)) &&
					// $(world.storage::<$id>().0.borrow().downcast_ref::<SimpleStorage<$id>>().map_or(false, |x| x.data.contains_key(&entity))&&)*
					true
				}

				fn added(world: &World, entity: Entity) -> bool {
					// world.storage::<$first>().0.borrow().downcast_ref::<SimpleStorage<$first>>().map_or(false, |x| x.added.contains(&entity)) &&
					// $(world.storage::<$id>().0.borrow().downcast_ref::<SimpleStorage<$id>>().map_or(false, |x| x.added.contains(&entity))&&)*
					true
				}

				fn modified(world: &World, entity: Entity) -> bool {
					// world.storage::<$first>().0.borrow().downcast_ref::<SimpleStorage<$first>>().map_or(false, |x| x.modified.contains(&entity)) &&
					// $(world.storage::<$id>().0.borrow().downcast_ref::<SimpleStorage<$id>>().map_or(false, |x| x.modified.contains(&entity))&&)*
					true
				}

				fn removed(world: &World, entity: Entity) -> bool {
					// world.storage::<$first>().0.borrow().downcast_ref::<SimpleStorage<$first>>().map_or(false, |x| x.removed.contains(&entity)) &&
					// $(world.storage::<$id>().0.borrow().downcast_ref::<SimpleStorage<$id>>().map_or(false, |x| x.removed.contains(&entity))&&)*
					true
				}
			}
		}
		tuple_query! {$($id)*}
	};
}

tuple_query! {A B C D E F G H I J K L M N O P Q R S T U V W X Y Z}

pub struct Added<T: Query>(PhantomData<T>);
impl<T: Query> Query for Added<T> {
	fn query(world: &World, entity: Entity) -> bool { T::query(world, entity) && T::added(world, entity) }
}

pub struct Removed<T: Query>(PhantomData<T>);
impl<T: Query> Query for Removed<T> {
	fn query(world: &World, entity: Entity) -> bool { T::query(world, entity) && T::removed(world, entity) }
}

pub struct Modified<T: Query>(PhantomData<T>);
impl<T: Query> Query for Modified<T> {
	fn query(world: &World, entity: Entity) -> bool { T::query(world, entity) && T::modified(world, entity) }
}
