/// methods to create HTML Elements as well as their types
pub mod components;
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
#[doc(hidden)] pub mod state;
mod storage;
mod style_storage;
pub mod web_str;
mod world;

pub use web_sys;
use crate::prelude::*;
#[doc(hidden)] pub use discard;
pub use element::{Element, SomeElement};
pub use entity::AsEntity;
pub use futures_signals as signals;
pub use hobo_css as css;
use owning_ref::{OwningHandle, OwningRef, OwningRefMut};
pub use paste;
pub use prelude::{Children, Parent};
use racy_cell::RacyCell;
use std::{
	any::TypeId,
	cell::{Ref, RefCell, RefMut},
	rc::Rc,
};
use storage::*;
use style_storage::{StyleStorage, STYLE_STORAGE};
use sugars::hash;

// NOTES:
// queries to be able to find entities with/by components in children/parent/ancestor/family - done
// * optionaly specify depth?
// resources stay, resources could be useful for caching/memoization/etc
// add a name component that sets data-name or smth
// * should be possible to find by name for those cases where you cba to set up a proper relationship
// could use an attribute macro over intostyle expressions to give them names and use names rather than hashes
// organise uses, itnernal prelude uses could be pub(crate)
// test shit ffs
// move World and AsEntity into separate files
// could remove all *_mut elements and specify whether you want mutable or immutable component with the same trick as in Query

// this is not necessary, but it makes it convenient to further remap to some OwningRef or whatever
type StorageRef<'a, Component> = OwningRef<OwningHandle<Rc<RefCell<Box<(dyn storage::DynStorage + 'static)>>>, Ref<'a, Box<dyn storage::DynStorage>>>, SimpleStorage<Component>>;
type StorageRefMut<'a, Component> = OwningRefMut<OwningHandle<Rc<RefCell<Box<(dyn storage::DynStorage + 'static)>>>, RefMut<'a, Box<dyn storage::DynStorage>>>, SimpleStorage<Component>>;

/// Register a browser window to also receive styles, automatically called for the global `window` object
pub fn register_window(window: &web_sys::Window) {
	let style_storage = unsafe { &mut *STYLE_STORAGE.get() as &mut StyleStorage };
	style_storage.register_window(window);
}

#[extend::ext(pub, name = MarkClassString)]
impl<T: 'static> T {
	fn mark_class_string() -> String {
		format!("t-{:x}", hash!(TypeId::of::<Self>()))
	}
}

/// Find all entities matching a query
pub fn find<Q: query::Query>() -> Vec<Q::Fetch> {
	World::mark_borrow_mut();
	let world = unsafe { &mut *WORLD.get() as &mut World };
	let mut entities = None;
	Q::filter(world, &mut entities);
	let res = entities.unwrap_or_default().into_iter().map(|entity| Q::fetch(world, entity)).collect::<Vec<_>>();
	World::unmark_borrow_mut();
	res
}

/// Find one entity matching a query if there is one
pub fn try_find_one<Q: query::Query>() -> Option<Q::Fetch> {
	World::mark_borrow_mut();
	let world = unsafe { &mut *WORLD.get() as &mut World };
	let mut entities = None;
	Q::filter(world, &mut entities);
	let res = entities.unwrap_or_default().into_iter().next().map(|entity| Q::fetch(world, entity));
	World::unmark_borrow_mut();
	res
}

/// Find one entity matching a query, panic otherwise
pub fn find_one<Q: query::Query>() -> Q::Fetch { try_find_one::<Q>().unwrap() }

pub fn world() -> world::WorldMut {
	World::mark_borrow_mut();
	let world = unsafe { &mut *WORLD.get() as &mut World };
	world::WorldMut(world)
}
