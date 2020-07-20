mod basic_element;
mod container;
pub mod create;
mod element;
mod enclose;
pub mod events;
pub mod prelude;
mod replaceable;
mod slot;
pub mod state_slice;
mod style_storage;
mod svg_element;
pub mod web_str;

pub use basic_element::{BasicElement, RawElement};
pub use container::*;
pub use create::components as cmp;
pub use css;
pub use element::Element;
pub use events::*;
pub use hobo_derive::*;
pub use paste;
pub use replaceable::*;
pub use slot::*;
pub use web_sys;

pub type Color = css::color::Color;

fn dom() -> web_sys::Document { web_sys::window().expect("no window").document().expect("no document") }

#[derive(Default)]
pub struct Context {
	style_storage: style_storage::StyleStorage,
}

thread_local! {
	static CONTEXT: Context = Default::default();
}

pub trait SetText<T>: RawElement<RawElementType = T> where
	T: AsRef<web_sys::Element> + AsRef<web_sys::HtmlElement>,
{
	fn set_text<'a>(&self, x: impl Into<std::borrow::Cow<'a, str>>) {
		let html_element: &web_sys::HtmlElement = self.raw_element().as_ref();
		html_element.set_inner_text(&x.into());
	}

	fn text<'a>(self, x: impl Into<std::borrow::Cow<'a, str>>) -> Self where Self: Sized {
		self.set_text(x);
		self
	}
}

impl<T, E> SetText<E> for T where
	T: RawElement<RawElementType = E>,
	E: AsRef<web_sys::Element> + AsRef<web_sys::HtmlElement>,
{}
