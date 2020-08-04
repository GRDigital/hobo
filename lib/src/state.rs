#![allow(dead_code)]

//! pub/sub event-based state management

use slotmap::DenseSlotMap;
use std::{
	cell::{RefCell, RefMut, Cell},
	ops::{Deref, DerefMut},
	rc::{Rc, Weak},
};

static MAX_NESTED_UPDATES: usize = 100;

slotmap::new_key_type! {pub struct SubscriptionKey;}

type SubscriptionFn = Rc<RefCell<dyn FnMut()>>;

#[derive(Default)]
pub struct StateSlice<T> {
	data: RefCell<T>,
	subscribers: RefCell<DenseSlotMap<SubscriptionKey, SubscriptionFn>>,
	update_ongoing: Cell<bool>,
	dirty: Cell<bool>,
}

// TODO: better debug impl
impl<T: std::fmt::Debug> std::fmt::Debug for StateSlice<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { self.data.borrow().fmt(f) }
}

// This is only permissible because JS/WASM is single-threaded
// would have to be rethunked if/wheen threading arrives via std::thread
unsafe impl<T> Send for StateSlice<T> {}
unsafe impl<T> Sync for StateSlice<T> {}

// This is issued for .update() calls and will run all subscribers upon drop
struct StateSliceGuard<'a, T> {
	data_ref: Option<RefMut<'a, T>>,
	state: &'a StateSlice<T>,
}

impl<'a, T> Deref for StateSliceGuard<'a, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target { self.data_ref.as_ref().expect("deref for guard failed") }
}

impl<'a, T> DerefMut for StateSliceGuard<'a, T> {
	fn deref_mut(&mut self) -> &mut Self::Target { self.data_ref.as_mut().expect("deref mut for guard failed") }
}

// If there's already an ongoing update, this just releases state lock and does nothing
// then the ongoing update will re-run all the subs (as the state has changed once again)
impl<'a, T> Drop for StateSliceGuard<'a, T> {
	fn drop(&mut self) {
		drop(self.data_ref.take());
		self.state.dirty.set(true);
		if self.state.update_ongoing.get() {
			return;
		}
		self.state.update_ongoing.set(true);

		for _ in 0..MAX_NESTED_UPDATES {
			self.state.dirty.set(false);
			let snapshot = self.state.subscribers.borrow().values().cloned().collect::<Vec<_>>();
			for subscriber in snapshot {
				let mut subscriber = subscriber.borrow_mut();
				let f = subscriber.deref_mut();
				f();
			}

			if !self.state.dirty.get() {
				self.state.update_ongoing.set(false);
				return;
			}
		}

		panic!("too many nested updates");
	}
}

pub trait Unsub {
	fn unsubscribe(&self, key: SubscriptionKey);
}

impl<T> Unsub for Weak<StateSlice<T>> {
	fn unsubscribe(&self, key: SubscriptionKey) {
		if let Some(state) = self.upgrade() { state.unsubscribe(key); }
	}
}

impl<T> Unsub for &'static StateSlice<T> {
	fn unsubscribe(&self, key: SubscriptionKey) { StateSlice::unsubscribe(self, key); }
}

pub struct Subscription(Box<dyn Unsub>, SubscriptionKey);
impl Drop for Subscription { fn drop(&mut self) { self.0.unsubscribe(self.1); } }

impl<T> StateSlice<T> {
	pub fn new(initial: T) -> Self {
		Self { data: RefCell::new(initial), subscribers: Default::default(), update_ongoing: Default::default(), dirty: Default::default() }
	}

	pub fn update<'a>(&'a self) -> impl DerefMut<Target = T> + 'a {
		StateSliceGuard { data_ref: Some(self.data.borrow_mut()), state: self }
	}

	pub fn view<'a>(&'a self) -> impl Deref<Target = T> + 'a { self.data.borrow() }

	pub fn subscribe_key(&self, f: impl FnMut() + 'static) -> SubscriptionKey {
		self.subscribers.borrow_mut().insert(Rc::new(RefCell::new(f)))
	}

	pub fn subscribe(&'static self, f: impl FnMut() + 'static) -> Subscription {
		Subscription(Box::new(self), self.subscribe_key(f))
	}

	fn unsubscribe(&self, key: SubscriptionKey) { self.subscribers.borrow_mut().remove(key); }
}

#[derive(shrinkwraprs::Shrinkwrap, Default)]
pub struct State<T>(pub Rc<StateSlice<T>>);

impl<T> Clone for State<T> {
	fn clone(&self) -> Self { Self(Rc::clone(&self.0)) }
}

impl<T: 'static> State<T> {
	pub fn new(initial: T) -> Self { State(Rc::new(StateSlice::new(initial))) }

	#[must_use]
	pub fn subscribe(&self, f: impl FnMut() + 'static) -> Subscription {
		Subscription(Box::new(Rc::downgrade(&self.0)), self.0.subscribe_key(f))
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
