// use crate::prelude::*;
use std::any::Any;
use slotmap::DenseSlotMap;
use std::rc::Weak;
use std::cell::RefCell;

slotmap::new_key_type! {pub struct SubKey;}

#[derive(Default)]
pub struct Events {
	subscribers: RefCell<DenseSlotMap<SubKey, Box<dyn FnMut(&dyn Any)>>>,
}

// This is only permissible because JS/WASM is single-threaded
// would have to be rethunked if/wheen threading arrives via std::thread
unsafe impl Send for Events {}
unsafe impl Sync for Events {}

// TODO: nested subscribtions etc?
impl Events {
	pub fn fire<E: Any>(&self, e: &E) {
		for subscriber in self.subscribers.borrow_mut().values_mut() {
			subscriber(e);
		}
	}

	pub fn on_key<E: Any>(&self, mut f: impl FnMut(&E) + 'static) -> SubKey {
		let wrapper = move |e: &dyn Any| {
			if let Some(e) = e.downcast_ref::<E>() {
				f(e);
			}
		};
		self.subscribers.borrow_mut().insert(Box::new(wrapper))
	}

	pub fn on<E: Any>(&'static self, f: impl FnMut(&E) + 'static) -> Subscription {
		Subscription(Box::new(self), self.on_key(f))
	}

	pub fn unsubscribe(&self, key: SubKey) {
		self.subscribers.borrow_mut().remove(key);
	}
}

pub trait Unsub {
	fn unsubscribe(&self, key: SubKey);
}

impl Unsub for Weak<Events> {
	fn unsubscribe(&self, key: SubKey) {
		if let Some(events) = self.upgrade() { events.unsubscribe(key); }
	}
}

impl Unsub for &'static Events {
	fn unsubscribe(&self, key: SubKey) { Events::unsubscribe(self, key); }
}

pub struct Subscription(Box<dyn Unsub>, SubKey);
impl Drop for Subscription { fn drop(&mut self) { self.0.unsubscribe(self.1); } }
