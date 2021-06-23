// use crate::prelude::*;
use std::any::{Any, TypeId};
use slotmap::DenseSlotMap;
use std::rc::Weak;
use std::cell::RefCell;
use std::collections::HashMap;

slotmap::new_key_type! {pub struct SubKey;}

type SubFn = Box<dyn FnMut(&dyn Any)>;

#[derive(Default)]
pub struct Events {
	subscribers: RefCell<HashMap<TypeId, DenseSlotMap<SubKey, SubFn>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubInfo {
	key: SubKey,
	typeid: TypeId,
}

// This is only permissible because JS/WASM is single-threaded
// would have to be rethunked if/wheen threading arrives via std::thread
unsafe impl Send for Events {}
unsafe impl Sync for Events {}

// TODO: nested subscribtions etc?
impl Events {
	pub fn fire<E: Any>(&self, e: &E) {
		for subscriber in self.subscribers.borrow_mut().entry(TypeId::of::<E>()).or_default().values_mut() {
			subscriber(e);
		}
	}

	pub fn on_key<E: Any>(&self, mut f: impl FnMut(&E) + 'static) -> SubInfo {
		let wrapper = move |e: &dyn Any| {
			if let Some(e) = e.downcast_ref::<E>() {
				f(e);
			}
		};
		let typeid = TypeId::of::<E>();
		SubInfo {
			key: self.subscribers.borrow_mut().entry(typeid).or_default().insert(Box::new(wrapper)),
			typeid,
		}
	}

	pub fn on<E: Any>(&'static self, f: impl FnMut(&E) + 'static) -> Subscription {
		Subscription(Box::new(self), self.on_key(f))
	}

	pub fn unsubscribe(&self, info: SubInfo) {
		self.subscribers.borrow_mut().entry(info.typeid).or_default().remove(info.key);
	}
}

pub trait Unsub {
	fn unsubscribe(&self, info: SubInfo);
}

impl Unsub for Weak<Events> {
	fn unsubscribe(&self, info: SubInfo) {
		if let Some(events) = self.upgrade() { events.unsubscribe(info); }
	}
}

impl Unsub for &'static Events {
	fn unsubscribe(&self, info: SubInfo) { Events::unsubscribe(self, info); }
}

pub struct Subscription(Box<dyn Unsub>, SubInfo);
impl Drop for Subscription { fn drop(&mut self) { self.0.unsubscribe(self.1); } }
