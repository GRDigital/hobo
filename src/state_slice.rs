#![allow(dead_code)]

use std::{cell::{RefMut, RefCell}, rc::Rc, ops::{Deref, DerefMut}};
use slotmap::SlotMap;

static MAX_NESTED_UPDATES: usize = 100;

slotmap::new_key_type! {
	pub struct SubscriptionKey;
}

type SubscriptionFn = Rc<RefCell<dyn FnMut()>>;

#[derive(Default)]
pub struct StateSlice<T> {
	data: RefCell<T>,
	subscribers: RefCell<SlotMap<SubscriptionKey, SubscriptionFn>>,
	update_ongoing: RefCell<bool>,
	dirty: RefCell<bool>,
}

// TODO: better debug impl
impl<T: std::fmt::Debug> std::fmt::Debug for StateSlice<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.data.borrow().fmt(f)
	}
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
		*self.state.dirty.borrow_mut() = true;
		if *self.state.update_ongoing.borrow() { return; }
		*self.state.update_ongoing.borrow_mut() = true;

		for _ in 0..MAX_NESTED_UPDATES {
			*self.state.dirty.borrow_mut() = false;
			let snapshot = self.state.subscribers.borrow().values().cloned().collect::<Vec<_>>();
			for subscriber in snapshot {
				let mut subscriber = subscriber.borrow_mut();
				let f = subscriber.deref_mut();
				f();
			}

			if !*self.state.dirty.borrow() {
				*self.state.update_ongoing.borrow_mut() = false;
				return;
			}
		}

		panic!("too many nested updates");
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
			update_ongoing: Default::default(),
			dirty: Default::default(),
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

#[derive(shrinkwraprs::Shrinkwrap, Clone)]
pub struct State<T>(Rc<StateSlice<T>>);

impl<T> State<T> {
	pub fn new(initial: T) -> Self { State(Rc::new(StateSlice::new(initial))) }
}
