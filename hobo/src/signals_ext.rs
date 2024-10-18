use discard::DiscardOnDrop;
use futures_signals::{
	signal::{Signal, SignalExt},
	signal_map::{MapDiff, SignalMap, SignalMapExt},
	signal_vec::{SignalVec, SignalVecExt, VecDiff},
	CancelableFutureHandle,
};
use wasm_bindgen_futures::spawn_local as spawn;

pub type Subscription = DiscardOnDrop<CancelableFutureHandle>;

pub trait SignalExt2: Signal {
	fn subscribe<F>(self, callback: F) -> Subscription where
		F: FnMut(Self::Item) + 'static,
		Self: Sized;

	// fn spawn(self) -> Subscription where
	//     Self: Sized;
}

impl<T: Signal + 'static> SignalExt2 for T {
	fn subscribe<F>(self, mut callback: F) -> Subscription where
		F: FnMut(Self::Item) + 'static,
		Self: Sized,
	{
		let (handle, fut) = futures_signals::cancelable_future(self.for_each(move |x| { callback(x); std::future::ready(()) }), Default::default);
		spawn(fut);
		handle
	}

	// fn spawn(self) -> Subscription where
	//     Self: Sized,
	// {
	//     let (handle, fut) = futures_signals::cancelable_future(self.for_each(move |_| std::future::ready(())), Default::default);
	//     spawn(fut);
	//     handle
	// }
}

pub trait SignalMapExt2: SignalMap {
	fn subscribe<F>(self, callback: F) -> Subscription where
		F: FnMut(MapDiff<Self::Key, Self::Value>) + 'static,
		Self: Sized;
}

impl<T: SignalMap + 'static> SignalMapExt2 for T {
	fn subscribe<F>(self, mut callback: F) -> Subscription where
		F: FnMut(MapDiff<Self::Key, Self::Value>) + 'static,
		Self: Sized,
	{
		let (handle, fut) = futures_signals::cancelable_future(self.for_each(move |x| { callback(x); std::future::ready(()) }), Default::default);
		spawn(fut);
		handle
	}
}

pub trait SignalVecExt2: SignalVec {
	fn subscribe<F>(self, callback: F) -> Subscription where
		F: FnMut(VecDiff<Self::Item>) + 'static,
		Self: Sized;
}

impl<T: SignalVec + 'static> SignalVecExt2 for T {
	fn subscribe<F>(self, mut callback: F) -> Subscription where
		F: FnMut(VecDiff<Self::Item>) + 'static,
		Self: Sized,
	{
		let (handle, fut) = futures_signals::cancelable_future(self.for_each(move |x| { callback(x); std::future::ready(()) }), Default::default);
		spawn(fut);
		handle
	}
}
