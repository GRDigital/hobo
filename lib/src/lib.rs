pub mod prelude;
pub mod web_str;
mod style_storage;
mod enclose;
mod storage;
pub mod state;
pub mod create;
mod dom_events;
pub mod events;
mod element;
mod racy_cell;
mod query;
mod hierarchy;
mod signals_ext;
mod entity;
mod resource;
mod world;

pub use hobo_css as css;
pub use web_sys;
pub use paste;
pub use futures_signals as signals;
pub use discard;
use crate::prelude::*;
use std::any::TypeId;
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use storage::*;
use owning_ref::{OwningRef, OwningRefMut, OwningHandle};
use style_storage::{STYLE_STORAGE, StyleStorage};
pub use element::{Element, Classes, SomeElement};
use racy_cell::RacyCell;
use sugars::hash;
pub use prelude::{Parent, Children};

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

pub fn fetch_classname(style: impl Into<css::Style>) -> String {
	let style_storage = unsafe { &mut *STYLE_STORAGE.get() as &mut StyleStorage };
	style_storage.fetch(style.into())
}

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

pub fn find<Q: query::Query>() -> Vec<Q::Fetch> {
	World::mark_borrow_mut();
	let world = unsafe { &mut *WORLD.get() as &mut World };
	let mut entities = None;
	Q::filter(world, &mut entities);
	let res = entities.unwrap_or_default().into_iter().map(|entity| Q::fetch(world, entity)).collect::<Vec<_>>();
	World::unmark_borrow_mut();
	res
}

pub fn try_find_one<Q: query::Query>() -> Option<Q::Fetch> {
	World::mark_borrow_mut();
	let world = unsafe { &mut *WORLD.get() as &mut World };
	let mut entities = None;
	Q::filter(world, &mut entities);
	let res = entities.unwrap_or_default().into_iter().next().map(|entity| Q::fetch(world, entity));
	World::unmark_borrow_mut();
	res
}

pub fn find_one<Q: query::Query>() -> Q::Fetch { try_find_one::<Q>().unwrap() }

pub fn world() -> world::WorldMut {
	World::mark_borrow_mut();
	let world = unsafe { &mut *WORLD.get() as &mut World };
	world::WorldMut(world)
}
