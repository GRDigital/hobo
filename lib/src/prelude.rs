#[doc(hidden)] pub use std::convert::TryInto;
#[doc(hidden)] pub use wasm_bindgen::JsCast;

pub use crate::{
	hierarchy::{Parent, Children},
	state,
	web_str,
	entity::{Entity, AsEntity},
	element::{Element, SomeElement},
	storage::{Storage, DynStorage},
	query::*,
	MarkClassString,
	dom_events::impls::*,
	create::StringValue,
	signals_ext::{SignalExt2, SignalMapExt2, SignalVecExt2},
	resource::{Resource, DefaultResource},
};
pub(crate) use crate::world::{World, WORLD};
pub use crate::css::{self, AppendProperty, F32, F32Ext as _};
pub use wasm_bindgen::prelude::*;
pub use web_sys;

#[must_use]
pub fn default<T: Default>() -> T { T::default() }
