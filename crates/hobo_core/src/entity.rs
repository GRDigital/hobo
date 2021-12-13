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
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let entity = self.as_entity();
		let storage = world.storage::<C>();
		if !storage.has(entity) {
			World::unmark_borrow_mut();
			return None;
		}
		let res = Some(OwningRef::new(storage).map(|x| x.get(entity).unwrap()));
		World::unmark_borrow_mut();
		res
	}
	#[inline] fn try_get_cmp_mut<'a, C: 'static>(&self) -> Option<OwningRefMut<StorageGuard<'a, C, StorageRefMut<'a, C>>, C>> where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let entity = self.as_entity();
		if !world.storage::<C>().has(entity) {
			World::unmark_borrow_mut();
			return None;
		}
		let res = Some(OwningRefMut::new(world.storage_mut::<C>()).map_mut(|x| x.get_mut(entity).unwrap()));
		World::unmark_borrow_mut();
		res
	}
	#[inline]
	#[track_caller]
	fn get_cmp<'a, C: 'static>(&self) -> OwningRef<StorageRef<'a, C>, C> where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = OwningRef::new(world.storage::<C>()).try_map(|x| x.get(self).ok_or_else(|| type_name::<C>())).expect("entity does not have component");
		World::unmark_borrow_mut();
		res
	}
	#[inline]
	#[track_caller]
	fn get_cmp_mut<'a, C: 'static>(&self) -> OwningRefMut<StorageGuard<'a, C, StorageRefMut<'a, C>>, C> where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = OwningRefMut::new(world.storage_mut::<C>()).try_map_mut(|x| x.get_mut(self).ok_or_else(|| type_name::<C>())).expect("entity does not have component");
		World::unmark_borrow_mut();
		res
	}
	#[inline] fn get_cmp_mut_or<'a, C: 'static>(&self, f: impl FnOnce() -> C) -> OwningRefMut<StorageGuard<'a, C, StorageRefMut<'a, C>>, C> where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = OwningRefMut::new(world.storage_mut::<C>()).map_mut(move |x| x.get_mut_or(self, f));
		World::unmark_borrow_mut();
		res
	}
	#[inline] fn get_cmp_mut_or_default<'a, C: 'static + Default>(&self) -> OwningRefMut<StorageGuard<'a, C, StorageRefMut<'a, C>>, C> where Self: Sized {
		self.get_cmp_mut_or(Default::default)
	}
	#[inline] fn remove_cmp<C: 'static>(&self) where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		world.storage_mut::<C>().remove(self);
		World::unmark_borrow_mut();
	}
	fn find_in_ancestors<Q: query::Query>(&self) -> Vec<Q::Fetch> {
		let mut entities = Some(Parent::ancestors(self.as_entity()).into_iter().collect());
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		Q::filter(world, &mut entities);
		let res = entities.unwrap_or_default().into_iter().map(|entity| Q::fetch(world, entity)).collect::<Vec<_>>();
		World::unmark_borrow_mut();
		res
	}
	fn try_find_first_in_ancestors<Q: query::Query>(&self) -> Option<Q::Fetch> {
		let mut entities = Some(Parent::ancestors(self.as_entity()).into_iter().collect());
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		Q::filter(world, &mut entities);
		let res = entities.unwrap_or_default().into_iter().next().map(|e| Q::fetch(world, e));
		World::unmark_borrow_mut();
		res
	}
	#[inline]
	#[track_caller]
	fn find_first_in_ancestors<Q: query::Query>(&self) -> Q::Fetch { self.try_find_first_in_ancestors::<Q>().expect("could not find query in ancestor") }
	fn find_in_descendants<Q: query::Query>(&self) -> Vec<Q::Fetch> {
		let mut entities = Some(Children::descendants(self.as_entity()).into_iter().collect());
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		Q::filter(world, &mut entities);
		let res = entities.unwrap_or_default().into_iter().map(|entity| Q::fetch(world, entity)).collect::<Vec<_>>();
		World::unmark_borrow_mut();
		res
	}
	fn find_in_children<Q: query::Query>(&self) -> Vec<Q::Fetch> {
		let mut entities = Some(self.as_entity().try_get_cmp::<Children>().map_or_else(default, |x| x.0.iter().copied().collect()));
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		Q::filter(world, &mut entities);
		let res = entities.unwrap_or_default().into_iter().map(|entity| Q::fetch(world, entity)).collect::<Vec<_>>();
		World::unmark_borrow_mut();
		res
	}
	fn try_find_first_in_descendants<Q: query::Query>(&self) -> Option<Q::Fetch> {
		let mut entities = Some(Children::descendants(self.as_entity()).into_iter().collect());
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		Q::filter(world, &mut entities);
		let res = entities.unwrap_or_default().into_iter().next().map(|e| Q::fetch(world, e));
		World::unmark_borrow_mut();
		res
	}
	fn try_find_first_in_children<Q: query::Query>(&self) -> Option<Q::Fetch> {
		let mut entities = Some(self.as_entity().try_get_cmp::<Children>().map_or_else(default, |x| x.0.iter().copied().collect()));
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		Q::filter(world, &mut entities);
		let res = entities.unwrap_or_default().into_iter().next().map(|e| Q::fetch(world, e));
		World::unmark_borrow_mut();
		res
	}
	#[inline]
	#[track_caller]
	fn find_first_in_descendants<Q: query::Query>(&self) -> Q::Fetch { self.try_find_first_in_descendants::<Q>().expect("could not find query in descendant") }
	#[inline]
	#[track_caller]
	fn find_first_in_children<Q: query::Query>(&self) -> Q::Fetch { self.try_find_first_in_children::<Q>().expect("could not find child") }
	#[inline] fn has_cmp<C: 'static>(&self) -> bool where Self: Sized {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = world.storage::<C>().has(self.as_entity());
		World::unmark_borrow_mut();
		res
	}

	#[inline] fn remove(&self) {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = world.remove_entity(self.as_entity());
		World::unmark_borrow_mut();
		res
	}
	#[inline] fn is_dead(&self)  -> bool {
		World::mark_borrow();
		let world = unsafe { &*WORLD.get() as &World };
		let res = world.is_dead(self.as_entity());
		World::unmark_borrow();
		res
	}
	#[inline] fn add_component<T: 'static>(&self, component: T) {
		World::mark_borrow_mut();
		let world = unsafe { &mut *WORLD.get() as &mut World };
		let res = world.storage_mut::<T>().add(self.as_entity(), component);
		World::unmark_borrow_mut();
		res
	}
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
