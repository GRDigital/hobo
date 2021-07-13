#[doc(hidden)] pub use std::convert::TryInto;
#[doc(hidden)] pub use wasm_bindgen::JsCast;

pub use crate::{
	state,
	web_str,
	WORLD,
	World,
	Entity,
	element::{
		Element,
		SomeElement,
		Parent,
		Children,
	},
	AsEntity,
	storage::{
		Storage,
		DynStorage,
	},
	query::*,
	TypeClassString,
	dom_events::impls::*,
	create::StringValue,
};
pub use crate::css::{self, AppendProperty, F32, F32Ext as _};
pub use wasm_bindgen::prelude::*;
pub use web_sys;

#[must_use]
pub fn default<T: Default>() -> T { T::default() }
