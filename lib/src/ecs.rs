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
	fn new<Q: Query + ?Sized>(f: fn(&World, Entity)) -> Self {
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
	pub storages: RefCell<HashMap<TypeId, RefCell<Box<dyn DynStorage>>>>,
	pub systems_interests: RefCell<HashMap<Entity, RefCell<Vec<Rc<RefCell<System>>>>>>,
	pub systems: RefCell<HashMap<*const fn(&World, Entity), Weak<RefCell<System>>>>,
}

unsafe impl Send for World {}
unsafe impl Sync for World {}

impl World {
	fn assure_storage<Component: 'static>(&self) {
		self.storages.borrow_mut().entry(TypeId::of::<SimpleStorage<Component>>()).or_insert_with(|| RefCell::new(Box::new(SimpleStorage::<Component>::default())));
	}

	pub fn storage_mut<Component: 'static>(&self) -> StorageGuard<'_, Component, impl std::ops::DerefMut<Target = SimpleStorage<Component>> + '_> {
		self.assure_storage::<Component>();

		let storages = OwningRef::new(self.storages.borrow());
		let storage_cell = OwningRefMut::new(OwningHandle::new_mut(storages.map(|x| x.get(&TypeId::of::<SimpleStorage<Component>>()).unwrap())));
		let storage = storage_cell.map_mut(|x| x.as_any_mut().downcast_mut::<SimpleStorage<Component>>().unwrap());
		StorageGuard(self, Some(storage))
	}

	pub fn storage<Component: 'static>(&self) -> impl std::ops::Deref<Target = SimpleStorage<Component>> + '_ {
		self.assure_storage::<Component>();

		let storages = OwningRef::new(self.storages.borrow());
		let storage_cell = OwningRef::new(OwningHandle::new(storages.map(|x| x.get(&TypeId::of::<SimpleStorage<Component>>()).unwrap())));
		storage_cell.map(|x| x.as_any().downcast_ref::<SimpleStorage<Component>>().unwrap())
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
	}

	pub fn remove_entity(&self, entity: Entity) {
		for storage in self.storages.borrow().values() {
			storage.borrow_mut().remove(entity);
		}

		let systems = self.schedule_systems(vec![entity]);

		for storage in self.storages.borrow().values() {
			storage.borrow_mut().flush();
		}

		self.run_systems(systems);

		self.systems_interests.borrow_mut().remove(&entity);
	}

	fn schedule_systems(&self, entities: impl IntoIterator<Item = Entity>) -> Vec<(Entity, Rc<RefCell<System>>)> {
		let interests = self.systems_interests.borrow();

		let mut v = vec![];
		for entity in entities {
			if let Some(systems) = interests.get(&entity) {
				for system_rc in systems.borrow().iter() {
					let system = system_rc.borrow();
					if (system.query)(self, entity) {
						v.push((entity, Rc::clone(&system_rc)));
					}
				}
			}
		}
		v
	}

	fn run_systems(&self, v: Vec<(Entity, Rc<RefCell<System>>)>) {
		for (entity, system) in v {
			(system.borrow().f)(self, entity);
		}
	}
}

#[test]
fn fuck() {
	static WORLD: Lazy<World> = Lazy::new(World::default);

	let entity = WORLD.new_entity();

	let mut sys = System::new::<Added<(String,)>>(|world, entity| {
		let other_entity = WORLD.new_entity();
		dbg!(world.storage::<String>().get(entity));
		world.storage_mut::<String>().add(other_entity, String::from("big poop"));
	});

	let mut panic_sys = System::new::<Removed<(String,)>>(|_, _| {
		dbg!("AAAAAAAAAA");
	});

	sys.entities.insert(entity);
	panic_sys.entities.insert(entity);

	WORLD.new_system(sys);
	WORLD.new_system(panic_sys);

	let mut storage = WORLD.storage_mut::<String>();
	storage.add(entity, String::from("poop"));
	drop(storage);
	WORLD.remove_entity(entity);
	// storage.remove(entity);
}
