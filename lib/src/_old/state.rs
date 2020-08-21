#![allow(dead_code)]

//! pub/sub event-based state management

mod state_slice;
mod tests;

use slotmap::DenseSlotMap;
use std::{
	cell::{RefCell, RefMut, Ref},
	ops::{Deref, DerefMut},
	rc::{Rc, Weak},
};
use state_slice::StateSliceMeta;
pub use state_slice::StateSlice;
pub use crate::{__view as view, __update as update};

slotmap::new_key_type! {pub struct SubscriptionKey;}

pub struct Subscription(Weak<RefCell<StateSliceMeta>>, SubscriptionKey);
impl Drop for Subscription {
	fn drop(&mut self) {
		let meta = if let Some(x) = self.0.upgrade() { x } else { return; };
		meta.borrow_mut().subscribers.remove(self.1);
	}
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
	state: Option<RefMut<'a, StateSlice<T>>>,
}

impl<'a, T> Deref for StateGuard<'a, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target { &self.state.as_ref().unwrap().data }
}

impl<'a, T> DerefMut for StateGuard<'a, T> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.state.as_mut().unwrap().data }
}

impl<'a, T> Drop for StateGuard<'a, T> {
	fn drop(&mut self) {
		let meta = self.state.take().unwrap().meta.clone();
		StateSliceMeta::trigger_update(&meta);
	}
}

impl<T: 'static> State<T> {
	pub fn new(initial: T) -> Self { State(Rc::new(RefCell::new(StateSlice::new(initial)))) }

	pub fn update<'a>(&'a self) -> impl DerefMut<Target = T> + 'a {
		StateGuard { state: Some(self.0.borrow_mut()) }
	}

	pub fn view<'a>(&'a self) -> impl Deref<Target = T> + 'a { Ref::map(self.0.borrow(), StateSlice::view) }

	pub fn subscribe_key(&self, f: impl FnMut() + 'static) -> SubscriptionKey {
		self.0.borrow().subscribe_key(f)
	}

	#[must_use]
	pub fn subscribe(&self, f: impl FnMut() + 'static) -> Subscription {
		let state_slice = self.0.borrow();
		Subscription(Rc::downgrade(&state_slice.meta), state_slice.subscribe_key(f))
	}

	fn unsubscribe(&self, key: SubscriptionKey) { self.0.borrow().unsubscribe(key) }
}

#[macro_export]
macro_rules! __view {
	($($element:ident).*) => {
		$($element.view()).*
	};
}

#[macro_export]
macro_rules! __update {
	($($element:ident).*) => {
		$($element.update()).*
	};
}
