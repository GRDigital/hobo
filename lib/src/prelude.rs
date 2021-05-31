#[doc(hidden)] pub use std::convert::TryInto;
#[doc(hidden)] pub use wasm_bindgen::JsCast;

pub use crate::{
	state,
	web_str,
	WORLD,
	World,
	Entity,
	Element,
	SomeElement,
	AsEntity,
	storage::{
		Storage,
		DynStorage,
	},
	query::*,
	TypeClassString,
	Component,
	dom_events::impls::*,
	create::StringValue,
};
pub use crate::css::{self, AppendProperty, F32, F32Ext as _};
pub use wasm_bindgen::prelude::*;
pub use web_sys;

#[must_use]
pub fn default<T: Default>() -> T { T::default() }
