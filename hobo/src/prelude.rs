#[doc(hidden)] pub use std::convert::TryInto;
#[doc(hidden)] pub use wasm_bindgen::JsCast;

pub(crate) use crate::world::{World, WORLD};
pub use crate::{
	create::html::StringValue,
	css::{self, AppendProperty, F32},
	dom_events::impls::*,
	element::{Element, SomeElement},
	entity::{AsEntity, Entity},
	hierarchy::{Children, Parent},
	query::{Query, With},
	resource::{DefaultResource, Resource},
	signals_ext::{SignalExt2, SignalMapExt2, SignalVecExt2},
	state,
	storage::{DynStorage, Storage},
	web_str, MarkClassString,
};
#[doc(hidden)] pub use wasm_bindgen::prelude::*;
pub use web_sys;

#[must_use]
pub(crate) fn default<T: Default>() -> T { T::default() }
