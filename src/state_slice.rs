#![allow(dead_code)]

use std::{cell::{RefMut, RefCell}, rc::Rc, ops::{Deref, DerefMut}};
use slotmap::SlotMap;

slotmap::new_key_type! {
	pub struct SubscriptionKey;
}

type SubscriptionFn = Rc<RefCell<dyn FnMut()>>;

#[derive(Default)]
pub struct StateSlice<T> {
	data: RefCell<T>,
	subscribers: RefCell<SlotMap<SubscriptionKey, SubscriptionFn>>,
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

	fn deref(&self) -> &Self::Target { self.data_ref.as_ref().unwrap() }
}

impl<'a, T> DerefMut for StateSliceGuard<'a, T> {
	fn deref_mut(&mut self) -> &mut Self::Target { self.data_ref.as_mut().unwrap() }
}

impl<'a, T> Drop for StateSliceGuard<'a, T> {
	fn drop(&mut self) {
		drop(self.data_ref.take());
		let snapshot = self.state.subscribers.borrow().values().cloned().collect::<Vec<_>>();
		for subscriber in snapshot {
			let mut subscriber = subscriber.borrow_mut();
			let f = subscriber.deref_mut();
			f();
		}
	}
}

pub struct Subscription<'a>(&'a dyn Unsub<'a>, SubscriptionKey);

impl<'a> Drop for Subscription<'a> {
	fn drop(&mut self) { self.0.unsubscribe(self.1); }
}

pub trait Unsub<'a> {
	fn unsubscribe(&'a self, key: SubscriptionKey);
}

impl<T> StateSlice<T> {
	pub fn new(initial: T) -> Self {
		Self {
			data: RefCell::new(initial),
			subscribers: Default::default(),
		}
	}

	pub fn update<'a>(&'a self) -> impl DerefMut<Target = T> + 'a {
		StateSliceGuard {
			data_ref: Some(self.data.borrow_mut()),
			state: self,
		}
	}

	pub fn view<'a>(&'a self) -> impl Deref<Target = T> + 'a { self.data.borrow() }

	#[must_use]
	pub fn subscribe(&self, f: impl FnMut() + 'static) -> Subscription {
		Subscription(self, self.subscribers.borrow_mut().insert(Rc::new(RefCell::new(f))))
	}
}

impl<'a, T> Unsub<'a> for StateSlice<T> {
	fn unsubscribe(&'a self, key: SubscriptionKey) { self.subscribers.borrow_mut().remove(key); }
}

#[derive(shrinkwraprs::Shrinkwrap)]
pub struct State<T>(Rc<StateSlice<T>>);

impl<T> State<T> {
	pub fn new(initial: T) -> Self { State(Rc::new(StateSlice::new(initial))) }
}
