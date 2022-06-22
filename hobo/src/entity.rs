use crate::{prelude::*, query, storage::StorageGuard, StorageRef, StorageRefMut};
pub use hobo_derive::AsEntity;
use owning_ref::{OwningRef, OwningRefMut};
use std::any::type_name;

/// An opaque copyable identifier that is used to attach and fetch components
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Entity(pub(crate) u64);

impl Entity {
	pub(crate) fn root() -> Self { Self(0) }
}

pub trait AsEntity {
	fn as_entity(&self) -> Entity;
	#[inline] fn try_get_cmp<'a, C: 'static>(&self) -> Option<OwningRef<StorageRef<'a, C>, C>> where Self: Sized {
		let entity = self.as_entity();
		let storage = WORLD.storage::<C>();
		if !storage.has(entity) {
			return None;
		}
		let res = Some(OwningRef::new(storage).map(|x| x.get(entity).unwrap()));
		res
	}
	#[inline] fn try_get_cmp_mut<'a, C: 'static>(&self) -> Option<OwningRefMut<StorageGuard<'a, C, StorageRefMut<'a, C>>, C>> where Self: Sized {
		let entity = self.as_entity();
		if !WORLD.storage::<C>().has(entity) {
			return None;
		}
		let res = Some(OwningRefMut::new(WORLD.storage_mut::<C>()).map_mut(|x| x.get_mut(entity).unwrap()));
		res
	}
	#[inline]
	#[track_caller]
	fn get_cmp<'a, C: 'static>(&self) -> OwningRef<StorageRef<'a, C>, C> where Self: Sized {
		let res = OwningRef::new(WORLD.storage::<C>()).try_map(|x| x.get(self).ok_or_else(|| type_name::<C>())).expect("entity does not have component");
		res
	}
	#[inline]
	#[track_caller]
	fn get_cmp_mut<'a, C: 'static>(&self) -> OwningRefMut<StorageGuard<'a, C, StorageRefMut<'a, C>>, C> where Self: Sized {
		let res = OwningRefMut::new(WORLD.storage_mut::<C>()).try_map_mut(|x| x.get_mut(self).ok_or_else(|| type_name::<C>())).expect("entity does not have component");
		res
	}
	#[inline] fn get_cmp_mut_or<'a, C: 'static>(&self, f: impl FnOnce() -> C) -> OwningRefMut<StorageGuard<'a, C, StorageRefMut<'a, C>>, C> where Self: Sized {
		let res = OwningRefMut::new(WORLD.storage_mut::<C>()).map_mut(move |x| x.get_mut_or(self, f));
		res
	}
	#[inline] fn get_cmp_mut_or_default<'a, C: 'static + Default>(&self) -> OwningRefMut<StorageGuard<'a, C, StorageRefMut<'a, C>>, C> where Self: Sized {
		self.get_cmp_mut_or(Default::default)
	}
	#[inline] fn remove_cmp<C: 'static>(&self) where Self: Sized {
		WORLD.storage_mut::<C>().remove(self);
	}
	fn find_in_ancestors<Q: query::Query>(&self) -> Vec<Q::Fetch> {
		let mut entities = Some(Parent::ancestors(self.as_entity()).into_iter().collect());
		Q::filter(&WORLD, &mut entities);
		let res = entities.unwrap_or_default().into_iter().map(|entity| Q::fetch(&WORLD, entity)).collect::<Vec<_>>();
		res
	}
	fn try_find_first_in_ancestors<Q: query::Query>(&self) -> Option<Q::Fetch> {
		let mut entities = Some(Parent::ancestors(self.as_entity()).into_iter().collect());
		Q::filter(&WORLD, &mut entities);
		let res = entities.unwrap_or_default().into_iter().next().map(|e| Q::fetch(&WORLD, e));
		res
	}
	#[inline]
	#[track_caller]
	fn find_first_in_ancestors<Q: query::Query>(&self) -> Q::Fetch { self.try_find_first_in_ancestors::<Q>().expect("could not find query in ancestor") }
	fn find_in_descendants<Q: query::Query>(&self) -> Vec<Q::Fetch> {
		let mut entities = Some(Children::descendants(self.as_entity()).into_iter().collect());
		Q::filter(&WORLD, &mut entities);
		let res = entities.unwrap_or_default().into_iter().map(|entity| Q::fetch(&WORLD, entity)).collect::<Vec<_>>();
		res
	}
	fn find_in_children<Q: query::Query>(&self) -> Vec<Q::Fetch> {
		let mut entities = Some(self.as_entity().try_get_cmp::<Children>().map_or_else(default, |x| x.0.iter().copied().collect()));
		Q::filter(&WORLD, &mut entities);
		let res = entities.unwrap_or_default().into_iter().map(|entity| Q::fetch(&WORLD, entity)).collect::<Vec<_>>();
		res
	}
	fn try_find_first_in_descendants<Q: query::Query>(&self) -> Option<Q::Fetch> {
		let mut entities = Some(Children::descendants(self.as_entity()).into_iter().collect());
		Q::filter(&WORLD, &mut entities);
		let res = entities.unwrap_or_default().into_iter().next().map(|e| Q::fetch(&WORLD, e));
		res
	}
	fn try_find_first_in_children<Q: query::Query>(&self) -> Option<Q::Fetch> {
		let mut entities = Some(self.as_entity().try_get_cmp::<Children>().map_or_else(default, |x| x.0.iter().copied().collect()));
		Q::filter(&WORLD, &mut entities);
		let res = entities.unwrap_or_default().into_iter().next().map(|e| Q::fetch(&WORLD, e));
		res
	}
	#[inline]
	#[track_caller]
	fn find_first_in_descendants<Q: query::Query>(&self) -> Q::Fetch { self.try_find_first_in_descendants::<Q>().expect("could not find query in descendant") }
	#[inline]
	#[track_caller]
	fn find_first_in_children<Q: query::Query>(&self) -> Q::Fetch { self.try_find_first_in_children::<Q>().expect("could not find child") }
	#[inline] fn has_cmp<C: 'static>(&self) -> bool where Self: Sized { WORLD.storage::<C>().has(self.as_entity()) }
	#[inline] fn remove(&self) { WORLD.remove_entity(self.as_entity()) }
	#[inline] fn is_dead(&self)  -> bool { WORLD.is_dead(self.as_entity()) }
	#[inline] fn add_component<T: 'static>(&self, component: T) { WORLD.storage_mut::<T>().add(self.as_entity(), component) }
	#[inline] fn component<T: 'static>(self, component: T) -> Self where Self: Sized { self.add_component(component); self }
}

impl AsEntity for Entity {
	fn as_entity(&self) -> Entity { *self }
}

impl<T: AsEntity> AsEntity for &T {
	fn as_entity(&self) -> Entity { T::as_entity(*self) }
}
impl<T: AsEntity> AsEntity for &mut T {
	fn as_entity(&self) -> Entity { T::as_entity(*self) }
}
