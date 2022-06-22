use crate::{prelude::*, storage::StorageGuard, StorageRef, StorageRefMut};
use owning_ref::{OwningRef, OwningRefMut};

pub trait Resource: 'static {
	#[inline] fn register_resource(self) where Self: Sized {
		World::register_resource(&WORLD, self);
	}
	#[inline] fn resource<'a>() -> OwningRef<StorageRef<'a, Self>, Self> where Self: Sized {
		World::resource::<Self>(&WORLD)
	}
	#[inline] fn resource_mut<'a>() -> OwningRefMut<StorageGuard<'a, Self, StorageRefMut<'a, Self>>, Self> where Self: Sized {
		World::resource_mut::<Self>(&WORLD)
	}
	#[inline] fn try_resource<'a>() -> Option<OwningRef<StorageRef<'a, Self>, Self>> where Self: Sized {
		World::try_resource::<Self>(&WORLD)
	}
	#[inline] fn try_resource_mut<'a>() -> Option<OwningRefMut<StorageGuard<'a, Self, StorageRefMut<'a, Self>>, Self>> where Self: Sized {
		World::try_resource_mut::<Self>(&WORLD)
	}
}
impl<T: 'static + Sized> Resource for T {}

pub trait DefaultResource: Default + 'static {
	#[inline] fn resource_or_default<'a>() -> OwningRef<StorageRef<'a, Self>, Self> where Self: Sized {
		if !World::resource_exists::<Self>(&WORLD) {
			World::register_resource(&WORLD, Self::default());
		}
		World::resource::<Self>(&WORLD)
	}
	#[inline] fn resource_mut_or_default<'a>() -> OwningRefMut<StorageGuard<'a, Self, StorageRefMut<'a, Self>>, Self> where Self: Sized {
		if !World::resource_exists::<Self>(&WORLD) {
			World::register_resource(&WORLD, Self::default());
		}
		World::resource_mut::<Self>(&WORLD)
	}
}
impl<T: Default + 'static + Sized> DefaultResource for T {}
