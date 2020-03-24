#![feature(proc_macro_hygiene)]

mod basic_element;
mod element;
mod enclose;
pub mod prelude;
mod svg_element;
pub mod web_str;
pub mod components;
pub mod create;
pub mod svg_create;
pub mod events;
pub mod state_slice;

pub use basic_element::BasicElement;
pub use css;
pub use element::Element;
pub use hobo_derive::*;
pub use paste;
use std::{
	cell::RefCell,
	collections::HashMap,
	hash::{Hash, Hasher},
	borrow::Cow,
};
pub use web_sys;
pub use components as cmp;
pub use create::*;
pub use svg_create::*;
pub use events::*;

pub type Color = (u8, u8, u8, u8);

fn dom() -> web_sys::Document {
	web_sys::window().expect("no window").document().expect("no document")
}

#[derive(Default)]
pub struct Context {
	style_storage: StyleStorage,
}

thread_local! {
	static CONTEXT: Context = Default::default();
}

#[derive(Default)]
struct StyleStorage {
	map: RefCell<HashMap<css::Style, u64>>,
}

// TODO: right now if the same style is reused in multiple windows - won't work, need to track style insertion per window
// it checks if the style is already inserted as css into <style>
// if yes, just returns the class name
// if no, inserts it into <style> and then returns the class name
impl StyleStorage {
	fn fetch<'a>(&self, element: &web_sys::Element, style: impl Into<Cow<'a, css::Style>>) -> String {
		let style = style.into();
		if let Some(id) = self.map.borrow().get(&style) {
			return format!("s{}", id);
		}
		let mut hasher = std::collections::hash_map::DefaultHasher::new();
		style.hash(&mut hasher);
		let id = hasher.finish();
		self.map.borrow_mut().insert(style.clone().into_owned(), id);
		let class = format!("s{}", id);
		let mut style = style.into_owned();
		for rule in style.0.iter_mut() {
			for selector_component in (rule.0).0.iter_mut() {
				if *selector_component == css::selector::SelectorComponent::ClassPlaceholder {
					*selector_component = css::selector::SelectorComponent::Class(class.clone());
				}
			}
		}
		let dom = element.owner_document().expect("element not attached to a dom");
		let head = dom.head().expect("dom has no head");
		let style_element = if let Some(x) = head.get_elements_by_tag_name("style").get_with_index(0) {
			x
		} else {
			let element = dom.create_element(web_str::style()).unwrap();
			head.append_child(&element).unwrap();
			element
		};
		style_element.append_with_str_1(&style.to_string()).unwrap();
		class
	}
}

#[doc(hidden)]
#[extend::ext(pub, name = RawSetClass)]
impl web_sys::Element {
	fn set_class<'a>(&self, style: impl Into<Cow<'a, css::Style>>) {
		CONTEXT.with(move |ctx| {
			let element_class = ctx.style_storage.fetch(&self, style);
			self.set_attribute(web_str::class(), &element_class).unwrap();
		})
	}

	fn add_class<'a>(&self, style: impl Into<Cow<'a, css::Style>>) {
		CONTEXT.with(move |ctx| {
			let element_class = ctx.style_storage.fetch(&self, style);
			let existing_class = self.get_attribute(web_str::class()).unwrap_or_else(String::new);
			self.set_attribute(web_str::class(), &format!("{} {}", existing_class, element_class)).unwrap();
		})
	}

	fn set_style<'a>(&self, style: impl Into<Cow<'a, [css::Property]>>) {
		let _ = self.set_attribute(web_str::style(), &style.into().iter().map(std::string::ToString::to_string).collect::<String>());
	}

	fn remove_style(&self) {
		let _ = self.remove_attribute(web_str::style());
	}
}
