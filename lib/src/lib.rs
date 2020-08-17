mod basic_element;
mod container;
pub mod create;
mod element;
mod enclose;
pub mod events;
pub mod prelude;
mod replaceable;
mod slot;
pub mod state;
mod style_storage;
mod svg_element;
pub mod web_str;

pub use basic_element::{BasicElement, RawElement};
pub use container::*;
pub use create::components as cmp;
pub use hobo_css as css;
pub use element::Element;
#[doc(inline)]
pub use events::*;
pub use hobo_derive::*;
#[doc(hidden)]
pub use paste;
pub use replaceable::*;
pub use slot::*;
pub use web_sys;

fn dom() -> web_sys::Document { web_sys::window().expect("no window").document().expect("no document") }

thread_local! {
	static STYLE_STORAGE: style_storage::StyleStorage = Default::default();
}

/// Trait for hobo components with textual contents
pub trait SetText<T>: RawElement<RawElementType = T>
where
	T: AsRef<web_sys::Element> + AsRef<web_sys::HtmlElement> + Clone,
{
	fn set_text<'a>(&self, x: impl Into<std::borrow::Cow<'a, str>>) {
		let raw = self.raw_element();
		let html_element: &web_sys::HtmlElement = raw.as_ref() as &web_sys::HtmlElement;
		html_element.set_inner_text(&x.into());
	}

	fn text<'a>(self, x: impl Into<std::borrow::Cow<'a, str>>) -> Self
	where
		Self: Sized,
	{
		self.set_text(x);
		self
	}
}

impl<T, E> SetText<E> for T
where
	T: RawElement<RawElementType = E>,
	E: AsRef<web_sys::Element> + AsRef<web_sys::HtmlElement> + Clone,
{}

pub trait ToClassStr {
	fn to_class_str(self) -> String;
}

impl ToClassStr for &str {
	fn to_class_str(self) -> String { self.into() }
}

impl ToClassStr for String {
	fn to_class_str(self) -> String { self }
}

impl ToClassStr for &css::Style {
	fn to_class_str(self) -> String {
		STYLE_STORAGE.with(move |style_storage| style_storage.fetch(self.clone()))
	}
}

impl<T: Element + 'static> ToClassStr for &T {
	fn to_class_str(self) -> String {
		T::type_class_string()
	}
}
