use crate::{prelude::*, storage::StorageGuard, StorageRef, StorageRefMut};
use owning_ref::{OwningRef, OwningRefMut};

pub trait Resource: 'static {
	#[inline] fn register_resource(self) where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		World::register_resource(world, self);
		World::unmark_borrow_mut();
	}
	#[inline] fn resource<'a>() -> OwningRef<StorageRef<'a, Self>, Self> where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = World::resource::<Self>(world);
		World::unmark_borrow_mut();
		res
	}
	#[inline] fn resource_mut<'a>() -> OwningRefMut<StorageGuard<'a, Self, StorageRefMut<'a, Self>>, Self> where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = World::resource_mut::<Self>(world);
		World::unmark_borrow_mut();
		res
	}
	#[inline] fn try_resource<'a>() -> Option<OwningRef<StorageRef<'a, Self>, Self>> where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = World::try_resource::<Self>(world);
		World::unmark_borrow_mut();
		res
	}
	#[inline] fn try_resource_mut<'a>() -> Option<OwningRefMut<StorageGuard<'a, Self, StorageRefMut<'a, Self>>, Self>> where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = World::try_resource_mut::<Self>(world);
		World::unmark_borrow_mut();
		res
	}
}
impl<T: 'static + Sized> Resource for T {}

pub trait DefaultResource: Default + 'static {
	#[inline] fn resource_mut_or_default<'a>() -> OwningRefMut<StorageGuard<'a, Self, StorageRefMut<'a, Self>>, Self> where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		if !World::resource_exists::<Self>(world) {
			World::register_resource(world, Self::default());
		}
		let res = World::resource_mut::<Self>(world);
		World::unmark_borrow_mut();
		res
	}
}
impl<T: Default + 'static + Sized> DefaultResource for T {}
