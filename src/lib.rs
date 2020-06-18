#![feature(proc_macro_hygiene, trait_alias)]

mod basic_element;
mod element;
mod enclose;
pub mod prelude;
mod svg_element;
pub mod web_str;
pub mod create;
pub mod svg_create;
pub mod events;
pub mod state_slice;
mod style_storage;
mod container;
mod replaceable;
mod slot;

pub use basic_element::BasicElement;
pub use css;
pub use element::Element;
pub use hobo_derive::*;
pub use paste;
pub use web_sys;
pub use create::components as cmp;
pub use create::*;
pub use svg_create::*;
pub use events::*;
pub use container::*;
pub use replaceable::*;
pub use slot::*;

pub type Color = (u8, u8, u8, u8);

fn dom() -> web_sys::Document {
	web_sys::window().expect("no window").document().expect("no document")
}

#[derive(Default)]
pub struct Context {
	style_storage: style_storage::StyleStorage,
}

thread_local! {
	static CONTEXT: Context = Default::default();
}
