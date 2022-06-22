use crate::{prelude::*, storage::StorageGuard, StorageRef, StorageRefMut};
use owning_ref::{OwningRef, OwningRefMut};

pub trait Resource: 'static {
	#[inline] fn register_resource(self) where Self: Sized {
		World::register_resource(&WORLD, self);
	}
	#[inline] fn resource<'a>() -> OwningRef<StorageRef<'a, Self>, Self> where Self: Sized {
		let res = World::resource::<Self>(&WORLD);
		res
	}
	#[inline] fn resource_mut<'a>() -> OwningRefMut<StorageGuard<'a, Self, StorageRefMut<'a, Self>>, Self> where Self: Sized {
		let res = World::resource_mut::<Self>(&WORLD);
		res
	}
	#[inline] fn try_resource<'a>() -> Option<OwningRef<StorageRef<'a, Self>, Self>> where Self: Sized {
		let res = World::try_resource::<Self>(&WORLD);
		res
	}
	#[inline] fn try_resource_mut<'a>() -> Option<OwningRefMut<StorageGuard<'a, Self, StorageRefMut<'a, Self>>, Self>> where Self: Sized {
		let res = World::try_resource_mut::<Self>(&WORLD);
		res
	}
}
impl<T: 'static + Sized> Resource for T {}

pub trait DefaultResource: Default + 'static {
	#[inline] fn resource_or_default<'a>() -> OwningRef<StorageRef<'a, Self>, Self> where Self: Sized {
		if !World::resource_exists::<Self>(&WORLD) {
			World::register_resource(&WORLD, Self::default());
		}
		let res = World::resource::<Self>(&WORLD);
		res
	}
	#[inline] fn resource_mut_or_default<'a>() -> OwningRefMut<StorageGuard<'a, Self, StorageRefMut<'a, Self>>, Self> where Self: Sized {
		if !World::resource_exists::<Self>(&WORLD) {
			World::register_resource(&WORLD, Self::default());
		}
		let res = World::resource_mut::<Self>(&WORLD);
		res
	}
}
impl<T: Default + 'static + Sized> DefaultResource for T {}
