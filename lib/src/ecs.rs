mod query;
mod storage;

use crate::prelude::*;
use std::collections::{HashMap, HashSet};
use std::any::{Any, TypeId};
use std::rc::Rc;
use once_cell::sync::Lazy;
use std::cell::{RefCell, Cell};
use std::marker::PhantomData;
use query::*;
use storage::*;
use owning_ref::{OwningRef, OwningRefMut, OwningHandle};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Entity(u64);

pub struct System {
	f: Box<dyn Fn(Entity) + 'static>,
	query: fn(&World, Entity) -> bool,
}

impl System {
	fn new<Q: Query, F: Fn(Entity) + 'static>(f: F) -> Self {
		Self { f: Box::new(f), query: Q::query }
	}
}

// systems register entities they care about upon creation
// identical systems have their entities merged
#[derive(Default)]
pub struct World {
	pub next_entity_id: Cell<u64>,
	pub storages: RefCell<HashMap<TypeId, RefCell<Box<dyn DynStorage>>>>,
	pub resources: RefCell<HashMap<TypeId, RefCell<Box<dyn Any>>>>,
	pub systems_interests: RefCell<HashMap<Entity, Vec<Rc<System>>>>,
	pub dead_entities: RefCell<HashSet<Entity>>,
}

unsafe impl Send for World {}
unsafe impl Sync for World {}

pub static WORLD: Lazy<World> = Lazy::new(|| {
	let world = World::default();
	world.register_resource(crate::style_storage::StyleStorage::default());
	world
});

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

	pub fn register_resource<T: 'static>(&self, resource: T) {
		self.resources.borrow_mut().entry(TypeId::of::<T>()).or_insert_with(|| RefCell::new(Box::new(resource)));
	}

	// should probably return Option<T>
	pub fn resource<T: 'static>(&self) -> impl std::ops::Deref<Target = T> + '_ {
		let resources = OwningRef::new(self.resources.borrow());
		let resource_cell = OwningRef::new(OwningHandle::new(resources.map(|x| x.get(&TypeId::of::<T>()).unwrap())));
		resource_cell.map(|x| x.downcast_ref::<T>().unwrap())
	}

	// should probably return Option<T>
	pub fn resource_mut<T: 'static>(&self) -> impl std::ops::DerefMut<Target = T> + '_ {
		let resources = OwningRef::new(self.resources.borrow());
		let resource_cell = OwningRefMut::new(OwningHandle::new_mut(resources.map(|x| x.get(&TypeId::of::<T>()).unwrap())));
		resource_cell.map_mut(|x| x.downcast_mut::<T>().unwrap())
	}

	pub fn new_entity(&self) -> Entity {
		let entity = Entity(self.next_entity_id.get());
		self.next_entity_id.set(self.next_entity_id.get() + 1);
		entity
	}

	pub fn new_system(&self, sys: System, entities: impl IntoIterator<Item = Entity>) {
		let sys = Rc::new(sys);
		for entity in entities.into_iter() {
			let mut systems_interests = self.systems_interests.borrow_mut();
			systems_interests.entry(entity).or_insert_with(Vec::new).push(Rc::clone(&sys));
		}
	}

	pub fn remove_entity(&self, entity: Entity) {
		self.dead_entities.borrow_mut().insert(entity);
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

	pub fn is_dead(&self, entity: Entity) -> bool {
		self.dead_entities.borrow().contains(&entity)
	}

	fn schedule_systems(&self, entities: impl IntoIterator<Item = Entity>) -> Vec<(Entity, Rc<System>)> {
		let interests = self.systems_interests.borrow();

		let mut v = vec![];
		for entity in entities {
			if let Some(systems) = interests.get(&entity) {
				for system in systems.iter() {
					if (system.query)(self, entity) {
						v.push((entity, Rc::clone(&system)));
					}
				}
			}
		}
		v
	}

	fn run_systems(&self, v: Vec<(Entity, Rc<System>)>) {
		for (entity, system) in v {
			(system.f)(entity);
		}
	}
}

#[test]
fn fuck() {
	static TEST: Lazy<std::sync::Mutex<u32>> = Lazy::new(|| std::sync::Mutex::new(0));

	let entity = WORLD.new_entity();

	let sys = System::new::<(Added<(String,)>, (String,)), _>(|entity| {
		let other_entity = WORLD.new_entity();
		dbg!(WORLD.storage::<String>().get(entity));
		WORLD.storage_mut::<String>().add(other_entity, String::from("big poop"));
		*TEST.lock().unwrap() += 1;
	});

	let archetype_enter_sys = System::new::<(Added<(String, u64)>, (String, u64)), _>(|entity| {
		dbg!("archetype entered");
		*TEST.lock().unwrap() += 1;
	});

	let archetype_leave_sys = System::new::<(Removed<(String, u64)>,), _>(|entity| {
		dbg!("archetype left");
		*TEST.lock().unwrap() += 1;
	});

	let simple_remove_sys = System::new::<(Removed<(String,)>,), _>(|_| {
		dbg!("AAAAAAAAAA");
		*TEST.lock().unwrap() += 1;
	});

	WORLD.new_system(sys, vec![entity]);
	WORLD.new_system(archetype_enter_sys, vec![entity]);
	WORLD.new_system(archetype_leave_sys, vec![entity]);
	WORLD.new_system(simple_remove_sys, vec![entity]);

	WORLD.storage_mut::<String>().add(entity, String::from("poop"));
	WORLD.storage_mut::<u64>().add(entity, 10u64);
	WORLD.storage_mut::<u64>().remove(entity);
	WORLD.storage_mut::<u64>().add(entity, 10u64);

	WORLD.remove_entity(entity);

	assert_eq!(*TEST.lock().unwrap(), 6);
}

/*

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Element {
	entity: Entity,
}

#[derive(Default)]
struct Parent(Entity);

#[derive(Default)]
struct Children(Vec<Entity>);

impl Element {
	fn child(self, child: Element) -> Self {
		if WORLD.is_dead(self.entity) || WORLD.is_dead(child.entity) { return self; }
		WORLD.storage_mut::<Children>().get_mut_or(self.entity, Children::default).0.push(child.entity);
		WORLD.storage_mut::<Parent>().get_mut_or(child.entity, Parent::default).0 = self.entity;

		let storage = WORLD.storage::<web_sys::Node>();
		if let (Some(parent_node), Some(child_node)) = (storage.get(self.entity), storage.get(child.entity)) {
			parent_node.append_child(child_node).unwrap();
		}

		self
	}

	fn on_click(self, mut f: impl FnMut(Entity, web_sys::MouseEvent) + 'static) -> Self {
		if WORLD.is_dead(self.entity) { return self; }
		if let Some(target) = WORLD.storage::<web_sys::EventTarget>().get(self.entity) {
			let handler = Closure::wrap(Box::new(move |e| f(self.entity, e)) as Box<dyn FnMut(web_sys::MouseEvent) + 'static>);
			target.add_event_listener_with_callback(web_str::click(), handler.as_ref().unchecked_ref()).expect("can't add event listener");
			WORLD.storage_mut::<Vec<crate::events::EventHandler>>().get_mut_or(self.entity, Vec::new).push(crate::events::EventHandler::MouseEvent(handler));
		}

		self
	}

	fn class(self, style: crate::css::Style) -> Self {
		if WORLD.is_dead(self.entity) { return self; }
		if let Some(element) = WORLD.storage::<web_sys::Element>().get(self.entity) {
			let style_storage = WORLD.resource::<crate::style_storage::StyleStorage>();
			element.set_attribute(web_str::class(), &style_storage.fetch(style)).unwrap();
		}

		self
	}

	fn remove(self) {
		WORLD.remove_entity(self.entity)
	}

	fn state<T: 'static>(self, state: T) -> Self {
		WORLD.storage_mut::<T>().add(self.entity, state);
		self
	}

	fn system<Q: Query, F: Fn(Entity) + 'static>(self, f: F) -> Self {
		WORLD.new_system(System::new::<Q, _>(f), vec![self.entity]);
		self
	}
}

fn html_element(world: &'static World, entity: Entity, element: &impl AsRef<web_sys::HtmlElement>) {
	let element = element.as_ref().clone();
	world.storage_mut::<web_sys::Node>().add(entity, (element.as_ref() as &web_sys::Node).clone());
	world.storage_mut::<web_sys::Element>().add(entity, (element.as_ref() as &web_sys::Element).clone());
	world.storage_mut::<web_sys::EventTarget>().add(entity, (element.as_ref() as &web_sys::EventTarget).clone());
	world.storage_mut::<web_sys::HtmlElement>().add(entity, element);

	let sys = System::new::<(Removed<(web_sys::Element,)>,), _>(move |entity| {
		world.storage_mut::<web_sys::Element>().take_removed(entity).unwrap().remove();
		world.storage_mut::<web_sys::Node>().remove(entity);
		world.storage_mut::<web_sys::Element>().remove(entity);
		world.storage_mut::<web_sys::EventTarget>().remove(entity);
		world.storage_mut::<web_sys::HtmlElement>().remove(entity);
		world.storage_mut::<Vec<crate::events::EventHandler>>().remove(entity);
		if let Some(children) = world.storage::<Children>().get(entity) {
			for &child in &children.0 {
				world.storage_mut::<web_sys::Element>().remove(child);
			}
		}
	});
	world.new_system(sys, vec![entity]);
}

fn div() -> Element {
	let entity = WORLD.new_entity();
	let element = crate::create::div();
	html_element(&WORLD, entity, &element);
	WORLD.storage_mut::<web_sys::HtmlDivElement>().add(entity, element);
	let sys = System::new::<(Removed<(web_sys::HtmlElement,)>,), _>(move |entity| {
		WORLD.storage_mut::<web_sys::HtmlDivElement>().remove(entity);
	});
	WORLD.new_system(sys, vec![entity]);

	Element { entity }
}

fn input() -> Element {
	let entity = WORLD.new_entity();
	let element = crate::create::input();
	html_element(&WORLD, entity, &element);
	WORLD.storage_mut::<web_sys::HtmlInputElement>().add(entity, element);
	let sys = System::new::<(Removed<(web_sys::HtmlElement,)>,), _>(move |entity| {
		WORLD.storage_mut::<web_sys::HtmlInputElement>().remove(entity);
	});
	WORLD.new_system(sys, vec![entity]);

	Element { entity }
}

struct SomeElementState {
	foo: u32,
	bar: String,
}

fn some_element() -> Element {
	let input = input();
	let state = SomeElementState { foo: 15, bar: "woo".to_owned() };
	div()
		.child(div()
			.class(css::class!(css::width!(100 px)))
		)
		.on_click(move |_entity, _event| {
			// this is safe because class checks if the entity exists internally
			input.class(css::class!(css::width!(200 px)));
			dbg!("woo");
		})
		.state(state)
		.system::<(Modified<(SomeElementState,)>,), _>(move |entity| {
			let state_storage = WORLD.storage::<SomeElementState>();
			let state = state_storage.get(entity).unwrap();
			if WORLD.is_dead(input.entity) { return; }
			let value = WORLD.storage::<web_sys::HtmlInputElement>().get(input.entity).unwrap().value();
			dbg!(value, &state.bar);
		})
}
*/
