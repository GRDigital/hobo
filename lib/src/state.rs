#![allow(dead_code)]

//! pub/sub event-based state management

// mod tests;

use slotmap::DenseSlotMap;
use std::{
	cell::{RefCell, RefMut, Ref},
	ops::{Deref, DerefMut},
	rc::{Rc, Weak},
};
use owning_ref::OwningRefMut;

slotmap::new_key_type! {pub struct SubscriptionKey;}
static MAX_NESTED_UPDATES: usize = 100;
type SubscriptionFn = Rc<RefCell<dyn FnMut()>>;

#[derive(Default)]
pub struct StateMeta {
	subscribers: DenseSlotMap<SubscriptionKey, SubscriptionFn>,
	update_ongoing: bool,
	dirty: bool,
}

impl StateMeta {
	// if there is an update going on, dirty flag is reset to true, which forces the topmost trigger_update to run again
	// it will run in a loop until subscribers no longer set the dirty flag
	// useful if e.g. there's a scattered chain of state transformations predicated on state already being in a particular configuration
	fn trigger_update(this: &Rc<RefCell<Self>>) {
		this.borrow_mut().dirty = true;
		if this.borrow().update_ongoing { return; }
		this.borrow_mut().update_ongoing = true;

		for _ in 0..MAX_NESTED_UPDATES {
			this.borrow_mut().dirty = false;
			let snapshot = this.borrow().subscribers.values().cloned().collect::<Vec<_>>();
			for subscriber in snapshot {
				let subscriber = &mut *subscriber.borrow_mut();
				subscriber();
			}

			if !this.borrow().dirty {
				this.borrow_mut().update_ongoing = false;
				return;
			}
		}

		panic!("too many nested updates");
	}
}

pub struct Subscription(Weak<RefCell<StateMeta>>, SubscriptionKey);
impl Drop for Subscription {
	fn drop(&mut self) {
		// log::info!("DROPPING STATE SUBSCRIPTION");
		let meta = if let Some(x) = self.0.upgrade() { x } else { return; };
		meta.borrow_mut().subscribers.remove(self.1);
	}
}

#[derive(Default)]
pub struct State<T> {
	pub data: Rc<RefCell<T>>,
	pub meta: Rc<RefCell<StateMeta>>,
}

// This is only permissible because JS/WASM is single-threaded
// would have to be rethunked if/wheen threading arrives via std::thread
unsafe impl<T> Send for State<T> {}
unsafe impl<T> Sync for State<T> {}

impl<T> Clone for State<T> {
	fn clone(&self) -> Self { Self { data: Rc::clone(&self.data), meta: Rc::clone(&self.meta) } }
}

pub struct StateGuard<'a, T, Inner: DerefMut<Target = T> + 'a> {
	state: &'a State<T>,
	data: Option<Inner>,
}

impl<'a, T, Inner: DerefMut<Target = T> + 'a> Deref for StateGuard<'a, T, Inner> {
	type Target = T;

	fn deref(&self) -> &Self::Target { self.data.as_ref().unwrap() }
}

impl<'a, T, Inner: DerefMut<Target = T> + 'a> DerefMut for StateGuard<'a, T, Inner> {
	fn deref_mut(&mut self) -> &mut Self::Target { self.data.as_mut().unwrap() }
}

impl<'a, T, Inner: DerefMut<Target = T> + 'a> Drop for StateGuard<'a, T, Inner> {
	fn drop(&mut self) {
		drop(self.data.take());
		let meta = self.state.meta.clone();
		StateMeta::trigger_update(&meta);
	}
}

impl<T: 'static> State<T> {
	pub fn new(initial: T) -> Self {
		Self {
			data: Rc::new(RefCell::new(initial)),
			meta: Rc::new(RefCell::new(StateMeta::default())),
		}
	}

	pub fn update(&self) -> StateGuard<T, OwningRefMut<RefMut<T>, T>> {
		StateGuard {
			state: self,
			data: Some(OwningRefMut::new(self.data.borrow_mut())),
		}
	}

	pub fn view(&self) -> Ref<T> { self.data.borrow() }

	// the difference between this and subscribe is that subscribe will automatically unsubscribe on drop via Weak to StateMeta
	// thus subscribe_key should almost never be called
	pub fn subscribe_key(&self, f: impl FnMut() + 'static) -> SubscriptionKey {
		self.meta.borrow_mut().subscribers.insert(Rc::new(RefCell::new(f)))
	}

	#[must_use]
	pub fn subscribe(&self, f: impl FnMut() + 'static) -> Subscription {
		Subscription(Rc::downgrade(&self.meta), self.subscribe_key(f))
	}

	fn unsubscribe(&self, key: SubscriptionKey) { self.meta.borrow_mut().subscribers.remove(key); }
}
