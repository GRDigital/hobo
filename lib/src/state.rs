#![allow(dead_code)]

//! pub/sub event-based state management

use slotmap::DenseSlotMap;
use std::{
	cell::{RefCell, RefMut, Cell, Ref},
	ops::{Deref, DerefMut},
	rc::{Rc, Weak},
};

// TODO: calling .update() on state children should only be permissible with a &mut i.e. only if .update() was called on parent
// to prevent children updating quietly without parent noticing
// subscribing, however, should be possible with .view()
//
// broken atm because `data` is `T` and so when update handler tries to access data - it is already borrowed by the update
// should probably separate subscribers and the bools into an Rc

static MAX_NESTED_UPDATES: usize = 100;

slotmap::new_key_type! {pub struct SubscriptionKey;}

type SubscriptionFn = Rc<RefCell<dyn FnMut()>>;

#[derive(Default)]
pub struct StateSlice<T> {
	data: T,
	subscribers: RefCell<DenseSlotMap<SubscriptionKey, SubscriptionFn>>,
	update_ongoing: bool,
	dirty: bool,
}

// TODO: better debug impl
impl<T: std::fmt::Debug> std::fmt::Debug for StateSlice<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { self.data.fmt(f) }
}

// This is only permissible because JS/WASM is single-threaded
// would have to be rethunked if/wheen threading arrives via std::thread
unsafe impl<T> Send for StateSlice<T> {}
unsafe impl<T> Sync for StateSlice<T> {}

// impl<T> Deref for StateSlice<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target { &self.data }
// }

// This is issued for .update() calls and will run all subscribers upon drop
pub struct StateSliceGuard<'a, T> {
	// data_ref: Option<&'a mut T>,
	state: &'a mut StateSlice<T>,
}

impl<'a, T> Deref for StateSliceGuard<'a, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target { &self.state.data }
}

impl<'a, T> DerefMut for StateSliceGuard<'a, T> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state.data }
}

// If there's already an ongoing update, this just releases state lock and does nothing
// then the ongoing update will re-run all the subs (as the state has changed once again)
impl<'a, T> Drop for StateSliceGuard<'a, T> {
	fn drop(&mut self) {
		// drop(self.data_ref.take());
		self.state.dirty = true;
		if self.state.update_ongoing { return; }
		self.state.update_ongoing = true;

		println!("starting to run updates");
		for _ in 0..MAX_NESTED_UPDATES {
			self.state.dirty = false;
			let snapshot = self.state.subscribers.borrow().values().cloned().collect::<Vec<_>>();
			for subscriber in snapshot {
				println!("about to borrow mut");
				let mut subscriber = subscriber.borrow_mut();
				println!("borrowed yes");
				// let f = subscriber.deref_mut();
				subscriber();
			}

			if !self.state.dirty {
				self.state.update_ongoing = false;
				println!("finished updates");
				return;
			}
		}

		panic!("too many nested updates");
	}
}

pub trait Unsub {
	fn unsubscribe(&self, key: SubscriptionKey);
}

impl<T> Unsub for Weak<RefCell<StateSlice<T>>> {
	fn unsubscribe(&self, key: SubscriptionKey) {
		if let Some(state) = self.upgrade() { state.borrow_mut().unsubscribe(key); }
	}
}

impl<T> Unsub for &'static State<T> {
	fn unsubscribe(&self, key: SubscriptionKey) { self.0.borrow_mut().unsubscribe(key); }
}

pub struct Subscription(Box<dyn Unsub>, SubscriptionKey);
impl Drop for Subscription { fn drop(&mut self) { self.0.unsubscribe(self.1); } }

impl<T> StateSlice<T> {
	pub fn new(initial: T) -> Self {
		Self { data: initial, subscribers: Default::default(), update_ongoing: Default::default(), dirty: Default::default() }
	}

	pub fn update<'a>(&'a mut self) -> impl DerefMut<Target = T> + 'a {
		StateSliceGuard { state: self }
	}

	pub fn view<'a>(&'a self) -> &'a T { &self.data }

	pub fn subscribe_key(&self, f: impl FnMut() + 'static) -> SubscriptionKey {
		self.subscribers.borrow_mut().insert(Rc::new(RefCell::new(f)))
	}

	// pub fn subscribe(&'static mut self, f: impl FnMut() + 'static) -> Subscription {
	//     Subscription(Box::new(self), self.subscribe_key(f))
	// }

	fn unsubscribe(&mut self, key: SubscriptionKey) { self.subscribers.borrow_mut().remove(key); }
}

#[derive(Default)]
pub struct State<T>(pub Rc<RefCell<StateSlice<T>>>);

// This is only permissible because JS/WASM is single-threaded
// would have to be rethunked if/wheen threading arrives via std::thread
unsafe impl<T> Send for State<T> {}
unsafe impl<T> Sync for State<T> {}

impl<T> Clone for State<T> {
	fn clone(&self) -> Self { Self(Rc::clone(&self.0)) }
}

struct StateGuard<'a, T> {
	state: RefMut<'a, StateSlice<T>>,
}

impl<'a, T> Deref for StateGuard<'a, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target { &self.state.data }
}

impl<'a, T> DerefMut for StateGuard<'a, T> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state.data }
}

impl<'a, T> Drop for StateGuard<'a, T> {
	fn drop(&mut self) {
		println!("starting drop");
		self.state.update();
		println!("dropped");
	}
}

impl<T: 'static> State<T> {
	pub fn new(initial: T) -> Self { State(Rc::new(RefCell::new(StateSlice::new(initial)))) }

	pub fn update<'a>(&'a self) -> impl DerefMut<Target = T> + 'a {
		StateGuard { state: self.0.borrow_mut() }
	}

	pub fn view<'a>(&'a self) -> impl Deref<Target = T> + 'a { Ref::map(self.0.borrow(), StateSlice::view) }

	pub fn subscribe_key(&self, f: impl FnMut() + 'static) -> SubscriptionKey {
		self.0.borrow_mut().subscribe_key(f)
	}

	#[must_use]
	pub fn subscribe(&self, f: impl FnMut() + 'static) -> Subscription {
		Subscription(Box::new(Rc::downgrade(&self.0)), self.0.borrow_mut().subscribe_key(f))
	}
}

#[test]
fn state_update() {
	use crate::enclose as e;

	let state = State::new(5);
	let sub = state.subscribe(e!((%state state) move || {
		assert_eq!(*state.view(), 10);
	}));
	*state.update() = 10;
}

#[test]
fn sub_drop() {
	use crate::enclose as e;

	let state = State::new(5);
	let sub = state.subscribe(move || panic!("sub ran after dropped"));
	drop(sub);
	*state.update() = 10;
	assert_eq!(*state.view(), 10);
}
