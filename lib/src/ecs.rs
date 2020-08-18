mod query;
mod storage;

use crate::prelude::*;
use std::collections::{HashMap, BTreeMap, BTreeSet, HashSet};
use std::any::{Any, TypeId};
use std::rc::{Weak, Rc};
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::cell::{Ref, RefMut, RefCell, Cell};
use chashmap::{ReadGuard, WriteGuard, CHashMap};
use std::marker::PhantomData;
use query::*;
use storage::*;
use owning_ref::{OwningRef, OwningRefMut, OwningHandle, RefMutRef, RefRef, Erased, RefMutRefMut, RcRef};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Entity(u64);

#[derive(Clone)]
pub struct System {
	f: fn(&World, Entity),
	entities: HashSet<Entity>,
	query: fn(&World, Entity) -> bool,
}

impl std::fmt::Debug for System {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		self.fn_ptr().fmt(f)?;
		self.query_ptr().fmt(f)?;
		self.entities.fmt(f)
	}
}

impl System {
	fn new<Q: Query>(f: fn(&World, Entity)) -> Self {
		Self { f, query: Q::query, entities: Default::default() }
	}

	fn fn_ptr(&self) -> *const fn(&World, Entity) {
		self.f as _
	}

	fn query_ptr(&self) -> *const fn(&World, Entity) {
		self.query as _
	}
}

// systems register entities they care about upon creation
// identical systems have their entities merged
#[derive(Default)]
pub struct World {
	pub next_entity_id: Cell<u64>,
	pub storages: RefCell<HashMap<TypeId, Rc<RefCell<Box<dyn Any>>>>>,
	pub systems_interests: RefCell<HashMap<Entity, RefCell<Vec<Rc<RefCell<System>>>>>>,
	pub systems: RefCell<HashMap<*const fn(&World, Entity), Weak<RefCell<System>>>>,
}

unsafe impl Send for World {}
unsafe impl Sync for World {}

impl World {
	pub fn storage_mut<'a, 'b: 'a, Component: 'static>(&'b self) -> StorageGuardMut<'b, Component, impl std::ops::DerefMut<Target = SimpleStorage<Component>> + 'a> {
	// pub fn storage_mut<Component: 'static>(&self) -> StorageGuardMut<'_, Component, OwningRefMut<Box<dyn Erased>, SimpleStorage<Component>>> {
		self.storages.borrow_mut().entry(TypeId::of::<SimpleStorage<Component>>()).or_insert_with(|| Rc::new(RefCell::new(Box::new(SimpleStorage::<Component>::default()))));
		let storage_refcell = OwningRef::new(self.storages.borrow())
			.map(|x| x.get(&TypeId::of::<SimpleStorage<Component>>()).unwrap());
		let ref_res = OwningRefMut::new(OwningHandle::new_mut(RcRef::new(Rc::clone(&*storage_refcell))))
			.map_mut(|x| x.downcast_mut::<SimpleStorage<Component>>().unwrap());
		StorageGuardMut(self, ref_res)
	}

	pub fn storage<Component: 'static>(&self) -> StorageGuard<'_, Component, impl std::ops::Deref<Target = SimpleStorage<Component>> + '_> {
		// self.storages.borrow_mut().entry(TypeId::of::<SimpleStorage<Component>>()).or_insert_with(|| RefCell::new(Box::new(SimpleStorage::<Component>::default())));
		let storages = OwningHandle::new(&self.storages);
		let storage_or = OwningRef::new(storages);
		let x = storage_or.map(|x| x.get(&TypeId::of::<SimpleStorage<Component>>()).unwrap()).clone();
		let y = OwningHandle::new(x);
		let z = OwningRef::new(y);
		StorageGuard::new(self, z.map(|x| x.downcast_ref::<SimpleStorage<Component>>().unwrap()))
	}

	pub fn new_entity(&self) -> Entity {
		let entity = Entity(self.next_entity_id.get());
		self.next_entity_id.set(self.next_entity_id.get() + 1);
		entity
	}

	pub fn new_system(&self, sys: System) {
		let key = sys.fn_ptr();
		let sys_rc = if let Some(weak) = self.systems.borrow_mut().get(&key) {
			if let Some(sys_rc) = weak.upgrade() { Rc::clone(&sys_rc) }
			else { Rc::new(RefCell::new(sys)) }
		} else {
			Rc::new(RefCell::new(sys))
		};
		self.systems.borrow_mut().insert(key, Rc::downgrade(&sys_rc));

		let sys = sys_rc.borrow_mut();
		for &entity in &sys.entities {
			let mut systems_interests = self.systems_interests.borrow_mut();
			systems_interests.entry(entity).or_insert_with(|| RefCell::new(Vec::new())).borrow_mut().push(Rc::clone(&sys_rc));
		}

		// pub fn remove_entity(&self) -> Entity {
		// }
	}
}

#[test]
fn fuck() {
	static WORLD: Lazy<World> = Lazy::new(World::default);

	let entity = WORLD.new_entity();
	{
		let mut sys = System::new::<Added<(String,)>>(|world, entity| {
			dbg!(world.storage::<String>().get(entity));
		});
		sys.entities.insert(entity);
		WORLD.new_system(sys);
		dbg!("about to insert cmp");
		WORLD.storage_mut::<String>().add(entity, String::from("poop"));
		dbg!("inserted cmp");
	}
	panic!("AAAAAAAAAAA");

	// let mut world = World::default();
	// let entity = world.new_entity();

	// let sys = System::new::<(Modified<(String,)>, String)>(|world, entity| {});
	// let sys2 = System::new::<Added<(String,)>>(|world, entity| {});

	// assert_ne!(sys.fn_ptr(), sys2.fn_ptr());
	// assert_eq!(sys.fn_ptr(), sys.clone().fn_ptr());

	// world.new_system(sys);
}
