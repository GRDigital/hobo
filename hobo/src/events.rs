// use crate::prelude::*;
use slotmap::DenseSlotMap;
use std::{
	any::{Any, TypeId},
	cell::RefCell,
	collections::HashMap,
	rc::Weak,
};
use once_cell::sync::Lazy;

slotmap::new_key_type! {struct SubKey;}

type SubFn = Box<dyn FnMut(&dyn Any)>;

#[derive(Default)]
struct Events {
	subscribers: RefCell<HashMap<TypeId, DenseSlotMap<SubKey, SubFn>>>,
}

static EVENTS: Lazy<Events> = Lazy::new(Default::default);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SubInfo {
	key: SubKey,
	typeid: TypeId,
}

// This is only permissible because JS/WASM is single-threaded
// would have to be rethunked if/wheen threading arrives via std::thread
unsafe impl Send for Events {}
unsafe impl Sync for Events {}

// TODO: nested subscribtions etc?
impl Events {
	fn fire<E: Any>(&self, e: &E) {
		for subscriber in self.subscribers.borrow_mut().entry(TypeId::of::<E>()).or_default().values_mut() {
			subscriber(e);
		}
	}

	fn on_key<E: Any>(&self, mut f: impl FnMut(&E) + 'static) -> SubInfo {
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

	fn on<E: Any>(&'static self, f: impl FnMut(&E) + 'static) -> Subscription {
		Subscription(Box::new(self), self.on_key(f))
	}

	fn unsubscribe(&self, info: SubInfo) {
		self.subscribers.borrow_mut().entry(info.typeid).or_default().remove(info.key);
	}
}

trait Unsub {
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

pub fn fire<E: Any>(e: &E) { EVENTS.fire(e); }
pub fn on<E: Any>(f: impl FnMut(&E) + 'static) -> Subscription { EVENTS.on(f) }