use super::*;
use sugars::*;

pub trait BasicQuery: 'static {
	fn exists(world: &World, entity: Entity) -> bool;
	fn added(world: &World, entity: Entity) -> bool;
	fn modified(world: &World, entity: Entity) -> bool;
	fn removed(world: &World, entity: Entity) -> bool;
}

pub trait Query: 'static {
	fn components() -> HashSet<TypeId>;
	fn query(world: &World, entity: Entity) -> bool;
	fn run<F: FnMut(Entity) + 'static>(f: F) -> System {
		System { f: RefCell::new(Box::new(f)), query: Self::query, scheduled: Cell::new(false), interests: Self::components }
	}
}

// Added<(T1, T2, T3)> implies that one of T1, T2, T3 was added
// the use-case of an archetype that was just entered would be Query<(Added<(T1, T2, T3)>, (T1, T2, T3))>
pub struct Added<T: BasicQuery>(PhantomData<T>);
// Removed<(T1, T2, T3)> implies that at least one of T1, T2, T3 was removed
// the use-case of an archetype having been left could be Query<(Removed<(T1, T2, T3)>)>
pub struct Removed<T: BasicQuery>(PhantomData<T>);
// Modified<(T1, T2, T3)> implies that one of T1, T2, T3 was changed
pub struct Modified<T: BasicQuery>(PhantomData<T>);
// Present<(T1, T2, T3)> implies that all of T1, T2, T3 are attached to the entity
pub struct Present<T: BasicQuery>(PhantomData<T>);

pub struct Or<Left: Query, Right: Query>(PhantomData<Left>, PhantomData<Right>);
impl<Left: Query, Right: Query> Query for Or<Left, Right> {
	fn query(world: &World, entity: Entity) -> bool {
		Left::query(world, entity) || Right::query(world, entity)
	}

	fn components() -> HashSet<TypeId> {
		let mut acc = Left::components();
		acc.extend(Right::components());
		acc
	}
}

impl<T: 'static> BasicQuery for T {
	fn exists(world: &World, entity: Entity) -> bool {
		world.storage::<Self>().has(entity)
	}

	fn added(world: &World, entity: Entity) -> bool {
		world.storage::<Self>().added.contains(&entity)
	}

	fn modified(world: &World, entity: Entity) -> bool {
		world.storage::<Self>().modified.contains(&entity)
	}

	fn removed(world: &World, entity: Entity) -> bool {
		world.storage::<Self>().removed.contains(&entity)
	}
}

macro_rules! tuple_query {
	() => {};
	($first:ident $($id:ident)*) => {
		paste::item! {
			impl<$first: Query, $($id: Query),*> Query for ($first, $($id),*) {
				fn query(world: &World, entity: Entity) -> bool {
					$first::query(world, entity)
					$(&& $id::query(world, entity))*
				}

				#[allow(unused_mut)]
				fn components() -> HashSet<TypeId> {
					let mut acc = $first::components();
					$(acc.extend($id::components());)*
					acc
				}
			}

			impl<$first: BasicQuery, $($id: BasicQuery),*> Query for Present<($first, $($id),*)> {
				fn query(world: &World, entity: Entity) -> bool {
					$first::exists(world, entity)
					$(&& $id::exists(world, entity))*
				}

				fn components() -> HashSet<TypeId> {
					hset![
						TypeId::of::<$first>(),
						$(TypeId::of::<$id>()),*
					]
				}
			}

			// TODO: could use clever bitmasking to achieve a similar effect to Removed
			impl<$first: BasicQuery, $($id: BasicQuery),*> Query for Added<($first, $($id),*)> {
				fn query(world: &World, entity: Entity) -> bool {
					$first::added(world, entity)
					$(|| $id::added(world, entity))*
				}

				fn components() -> HashSet<TypeId> {
					hset![
						TypeId::of::<$first>(),
						$(TypeId::of::<$id>()),*
					]
				}
			}

			impl<$first: BasicQuery, $($id: BasicQuery),*> Query for Modified<($first, $($id),*)> {
				fn query(world: &World, entity: Entity) -> bool {
					$first::modified(world, entity)
					$(|| $id::modified(world, entity))*
				}

				fn components() -> HashSet<TypeId> {
					hset![
						TypeId::of::<$first>(),
						$(TypeId::of::<$id>()),*
					]
				}
			}

			impl<$first: BasicQuery, $($id: BasicQuery),*> Query for Removed<($first, $($id),*)> {
				#[allow(unused_mut)]
				fn query(world: &World, entity: Entity) -> bool {
					// total - bitmask with 1s for every component queried
					// present - bitmask with 1s for every queried component that exists
					// missing - bitmask with 1s for every queried component that is marked as removed

					let mut total: u32;
					let mut present: u32;
					let mut missing: u32;

					total = 1;
					present = if $first::exists(world, entity) { 1 } else { 0 };
					missing = if $first::removed(world, entity) { 1 } else { 0 };

					$(
						total = (total << 1) + 1;
						present = (present << 1) + if $id::exists(world, entity) { 1 } else { 0 };
						missing = (missing << 1) + if $id::removed(world, entity) { 1 } else { 0 };
					)*

					(present != total) && ((present | missing) == total)
				}

				fn components() -> HashSet<TypeId> {
					hset![
						TypeId::of::<$first>(),
						$(TypeId::of::<$id>()),*
					]
				}
			}
		}
		tuple_query! {$($id)*}
	};
}

tuple_query! {A B C D E F G H I J K L M N O P Q R S T U V W X Y Z}
