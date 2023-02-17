use crate::{prelude::*, storage::StorageGuard, storage::StorageGuardMut, StorageRef, StorageRefMut};
use owning_ref::{OwningRef, OwningRefMut};

pub trait Resource: 'static {
	#[inline]
	#[track_caller]
	fn register_resource(self) where Self: Sized {
		World::register_resource(&WORLD, self);
	}
	#[inline]
	#[track_caller]
	fn resource<'a>() -> OwningRef<StorageGuard<Self, StorageRef<'a, Self>>, Self> where Self: Sized {
		World::resource::<Self>(&WORLD)
	}
	#[inline]
	#[track_caller]
	fn resource_mut<'a>() -> OwningRefMut<StorageGuardMut<Self, StorageRefMut<'a, Self>>, Self> where Self: Sized {
		World::resource_mut::<Self>(&WORLD)
	}

	#[inline]
	#[track_caller]
	fn try_resource<'a>() -> Option<OwningRef<StorageGuard<Self, StorageRef<'a, Self>>, Self>> where Self: Sized {
		World::try_resource::<Self>(&WORLD)
	}
	#[inline]
	#[track_caller]
	fn try_resource_mut<'a>() -> Option<OwningRefMut<StorageGuardMut<Self, StorageRefMut<'a, Self>>, Self>> where Self: Sized {
		World::try_resource_mut::<Self>(&WORLD)
	}

	#[inline]
	#[track_caller]
	fn remove_resource() where Self: Sized {
		World::remove_resource::<Self>(&WORLD)
	}
}
impl<T: 'static + Sized> Resource for T {}

pub trait DefaultResource: Default + 'static {
	#[inline]
	#[track_caller]
	fn resource_or_default<'a>() -> OwningRef<StorageGuard<Self, StorageRef<'a, Self>>, Self> where Self: Sized {
		if !World::resource_exists::<Self>(&WORLD) {
			World::register_resource(&WORLD, Self::default());
		}
		World::resource::<Self>(&WORLD)
	}
	#[inline]
	#[track_caller]
	fn resource_mut_or_default<'a>() -> OwningRefMut<StorageGuardMut<Self, StorageRefMut<'a, Self>>, Self> where Self: Sized {
		if !World::resource_exists::<Self>(&WORLD) {
			World::register_resource(&WORLD, Self::default());
		}
		World::resource_mut::<Self>(&WORLD)
	}
}
impl<T: Default + 'static + Sized> DefaultResource for T {}
