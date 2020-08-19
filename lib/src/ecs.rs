mod query;
mod storage;

use crate::prelude::*;
use std::collections::{HashMap, BTreeMap, BTreeSet, HashSet};
use std::any::{Any, TypeId};
use std::rc::{Weak, Rc};
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::cell::{Ref, RefMut, RefCell, Cell};
use std::marker::PhantomData;
use query::*;
use storage::*;
use owning_ref::{OwningRef, OwningRefMut, OwningHandle};

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
	static TEST: Lazy<std::sync::Mutex<u32>> = Lazy::new(|| std::sync::Mutex::new(0));

	let entity = WORLD.new_entity();

	let mut sys = System::new::<(Added<(String,)>, (String,))>(|world, entity| {
		let other_entity = WORLD.new_entity();
		dbg!(world.storage::<String>().get(entity));
		world.storage_mut::<String>().add(other_entity, String::from("big poop"));
		*TEST.lock().unwrap() += 1;
	});

	let mut archetype_enter_sys = System::new::<(Added<(String, u64)>, (String, u64))>(|world, entity| {
		dbg!("archetype entered");
		*TEST.lock().unwrap() += 1;
	});

	let mut archetype_leave_sys = System::new::<(Removed<(String, u64)>,)>(|world, entity| {
		dbg!("archetype left");
		*TEST.lock().unwrap() += 1;
	});

	let mut simple_remove_sys = System::new::<(Removed<(String,)>,)>(|_, _| {
		dbg!("AAAAAAAAAA");
		*TEST.lock().unwrap() += 1;
	});

	sys.entities.insert(entity);
	archetype_enter_sys.entities.insert(entity);
	archetype_leave_sys.entities.insert(entity);
	simple_remove_sys.entities.insert(entity);

	WORLD.new_system(sys);
	WORLD.new_system(archetype_enter_sys);
	WORLD.new_system(archetype_leave_sys);
	WORLD.new_system(simple_remove_sys);

	WORLD.storage_mut::<String>().add(entity, String::from("poop"));
	WORLD.storage_mut::<u64>().add(entity, 10u64);
	WORLD.storage_mut::<u64>().remove(entity);
	WORLD.storage_mut::<u64>().add(entity, 10u64);

	WORLD.remove_entity(entity);

	assert_eq!(*TEST.lock().unwrap(), 6);
}

/*
fn mock_world() -> &'static World { todo!() }

struct EntityWrapper {
	entity: Entity,
}

impl EntityWrapper {
	fn child(self, child: EntityWrapper) -> Self {
		// let storage = world.storage_mut::<Node>();
		// storage.get_mut(child).unwrap().parent = Some(self.entity);
		// storage.get_mut(self.entity).unwrap().children.push(child);
		self
	}

	fn on_click(self, handler: impl FnMut(&World, Entity, web_sys::MouseEvent)) -> Self {
		// world.storage_mut::<
		self
	}

	fn class(self, style: String) -> Self {
		let storage = mock_world().storage::<web_sys::Element>();
		let element = storage.get(self.entity).unwrap();
		let _ = element.set_attribute("class", &style).unwrap();
		self
	}
}

fn div() -> EntityWrapper {
	let world = mock_world();
	let entity = world.new_entity();
	let div = crate::create::div();
	world.storage_mut::<web_sys::Element>().add(entity, (div.as_ref() as &web_sys::Element).clone());
	world.storage_mut::<web_sys::HtmlElement>().add(entity, (div.as_ref() as &web_sys::HtmlElement).clone());
	world.storage_mut::<web_sys::HtmlDivElement>().add(entity, div);
	// world.storage_mut::<Node>().add(entity, Node::default());

	EntityWrapper { entity }
}
*/
