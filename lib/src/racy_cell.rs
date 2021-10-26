use std::cell::UnsafeCell;

#[repr(transparent)]
pub(crate) struct RacyCell<T>(UnsafeCell<T>);

impl<T> RacyCell<T> {
	pub(crate) const fn new(value: T) -> Self { RacyCell(UnsafeCell::new(value)) }

	pub(crate) fn get(&self) -> *mut T { self.0.get() }
}

unsafe impl<T> Sync for RacyCell<T> {}
unsafe impl<T> Send for RacyCell<T> {}
