use super::*;

static MAX_NESTED_UPDATES: usize = 100;
type SubscriptionFn = Rc<RefCell<dyn FnMut()>>;

#[derive(Default)]
pub(super) struct StateSliceMeta {
	pub(super) subscribers: DenseSlotMap<SubscriptionKey, SubscriptionFn>,
	pub(super) update_ongoing: bool,
	pub(super) dirty: bool,
}

impl StateSliceMeta {
	pub(super) fn trigger_update(this: &Rc<RefCell<Self>>) {
		this.borrow_mut().dirty = true;
		if this.borrow().update_ongoing { return; }
		this.borrow_mut().update_ongoing = true;

		for _ in 0..MAX_NESTED_UPDATES {
			this.borrow_mut().dirty = false;
			let snapshot = this.borrow().subscribers.values().cloned().collect::<Vec<_>>();
			for subscriber in snapshot {
				let mut subscriber = subscriber.borrow_mut();
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

#[derive(Default)]
pub struct StateSlice<T> {
	pub(super) data: T,
	pub(super) meta: Rc<RefCell<StateSliceMeta>>,
}

impl<T> StateSlice<T> {
	pub fn new(initial: T) -> Self {
		Self { data: initial, meta: Default::default() }
	}

	pub fn update<'a>(&'a mut self) -> impl DerefMut<Target = T> + 'a {
		StateSliceGuard { data: &mut self.data, meta: Rc::clone(&self.meta) }
	}

	pub fn view(&self) -> &T { &self.data }

	pub fn subscribe_key(&self, f: impl FnMut() + 'static) -> SubscriptionKey {
		self.meta.borrow_mut().subscribers.insert(Rc::new(RefCell::new(f)))
	}

	pub fn unsubscribe(&self, key: SubscriptionKey) { self.meta.borrow_mut().subscribers.remove(key); }

}

// TODO: better debug impl
impl<T: std::fmt::Debug> std::fmt::Debug for StateSlice<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { self.data.fmt(f) }
}

// This is only permissible because JS/WASM is single-threaded
// would have to be rethunked if/wheen threading arrives via std::thread
unsafe impl<T> Send for StateSlice<T> {}
unsafe impl<T> Sync for StateSlice<T> {}

// This is issued for .update() calls and will run all subscribers upon drop
pub(super) struct StateSliceGuard<'a, T> {
	pub(super) data: &'a mut T,
	pub(super) meta: Rc<RefCell<StateSliceMeta>>,
}

impl<'a, T> Deref for StateSliceGuard<'a, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target { self.data }
}

impl<'a, T> DerefMut for StateSliceGuard<'a, T> {
	fn deref_mut(&mut self) -> &mut Self::Target { self.data }
}

// If there's already an ongoing update, this just releases state lock and does nothing
// then the ongoing update will re-run all the subs (as the state has changed once again)
impl<'a, T> Drop for StateSliceGuard<'a, T> {
	fn drop(&mut self) {
		StateSliceMeta::trigger_update(&self.meta);
	}
}
