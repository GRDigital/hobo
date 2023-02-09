/// methods to create HTML Elements as well as their types
pub mod create;
pub mod dom_events;
pub mod element;
mod enclose;
pub mod entity;
/// simple way to fire and respond to arbitrarily-typed events
pub mod events;
/// Parent and Child relations
pub mod hierarchy;
pub mod prelude;
pub mod query;
mod racy_cell;
/// resources are globally-accessible components
pub mod resource;
pub mod signals_ext;
mod storage;
mod style_storage;
pub mod web_str;
mod world;

pub use web_sys;
use crate::prelude::*;
#[doc(hidden)] pub use discard;
pub use element::{AsElement, Element};
pub use entity::AsEntity;
pub use futures_signals as signals;
pub use hobo_css as css;
use owning_ref::{OwningRef, OwningRefMut};
#[doc(hidden)] pub use paste;
pub use prelude::{Children, Parent};
use std::{
	any::TypeId,
	cell::{Ref, RefMut},
};
use storage::*;
use style_storage::{StyleStorage, STYLE_STORAGE};
use sugars::hash;
#[doc(hidden)] pub use world::World;
pub use owning_ref;

// NOTES:
// queries to be able to find entities with/by components in children/parent/ancestor/family - done
// * optionaly specify depth?
// resources stay, resources could be useful for caching/memoization/etc
// add a name component that sets data-name or smth
// * should be possible to find by name for those cases where you cba to set up a proper relationship
// could use an attribute macro over intostyle expressions to give them names and use names rather than hashes
// organise uses, itnernal prelude uses could be pub(crate)
// test shit ffs
// could? remove all *_mut elements and specify whether you want mutable or immutable component with the same trick as in Query

#[cfg(debug_assertions)]
pub mod backtrace {
	use super::*;
	use std::{
		collections::{HashMap, BTreeMap},
		cell::RefCell,
		panic::Location,
	};
	use once_cell::sync::Lazy;
	use shrinkwraprs::Shrinkwrap;

	pub static STORAGE_MAP: Lazy<BacktraceStorage> = Lazy::new(Default::default);

	#[repr(transparent)]
	#[derive(Debug, Default)]
	pub struct BacktraceStorage<'a>(pub RefCell<HashMap<TypeId, LocationMap<'a>>>);

	unsafe impl Send for BacktraceStorage<'_> {}
	unsafe impl Sync for BacktraceStorage<'_> {}

	#[repr(transparent)]
	#[derive(Debug, Default, Shrinkwrap)]
	#[shrinkwrap(mutable)]
	pub struct LocationMap<'a>(pub BTreeMap<Location<'a>, bool>);

	impl<'a> std::fmt::Display for LocationMap<'a> {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			for (location, mutable) in self.iter().rev() {
				// pretty offset
				writeln!(f, "{:>5} {}", if *mutable { "(mut)" } else { "" }, location)?;
			};
			Ok(())
		}
	}
}

// this is not necessary, but it makes it convenient to further remap to some OwningRef or whatever
pub type StorageRef<'a, Component> = OwningRef<Ref<'a, Box<dyn storage::DynStorage>>, SimpleStorage<Component>>;
pub type StorageRefMut<'a, Component> = OwningRefMut<RefMut<'a, Box<dyn storage::DynStorage>>, SimpleStorage<Component>>;

/// Register a browser window to also receive styles, automatically called for the global `window` object with the name "default"
pub fn register_window(window: &web_sys::Window, window_name: String) {
	let style_storage = unsafe { &mut *STYLE_STORAGE.get() as &mut StyleStorage };
	style_storage.register_window(window, window_name);
}

/// Removes a window from the style storage
pub fn unregister_window(window_name: &str) {
	let style_storage = unsafe { &mut *STYLE_STORAGE.get() as &mut StyleStorage };
	style_storage.unregister_window(window_name);
}

#[doc(hidden)]
#[extend::ext(pub, name = MarkClassString)]
impl<T: 'static> T {
	fn mark_class_string() -> String {
		format!("t-{:x}", hash!(TypeId::of::<Self>()))
	}
}

/// Find all entities matching a query
pub fn find<Q: query::Query>() -> Vec<Q::Fetch> {
	let mut entities = None;
	Q::filter(&WORLD, &mut entities);
	entities.unwrap_or_default().into_iter().map(|entity| Q::fetch(&WORLD, entity)).collect::<Vec<_>>()
}

/// Find one entity matching a query if there is one
pub fn try_find_one<Q: query::Query>() -> Option<Q::Fetch> {
	let mut entities = None;
	Q::filter(&WORLD, &mut entities);
	entities.unwrap_or_default().into_iter().next().map(|entity| Q::fetch(&WORLD, entity))
}

/// Find one entity matching a query, panic otherwise
pub fn find_one<Q: query::Query>() -> Q::Fetch { try_find_one::<Q>().unwrap() }

// #[doc(hidden)]
// pub fn world() -> world::WorldMut {
//     World::mark_borrow_mut();
//     let world = unsafe { &mut *WORLD.get() as &mut World };
//     world::WorldMut(world)
// }
