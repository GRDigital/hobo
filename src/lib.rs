#![feature(proc_macro_hygiene)]
#![recursion_limit="1024"]

mod basic_element;
mod element;
mod enclose;
pub mod prelude;
mod svg_element;
pub mod web_str;
pub mod components;

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
// pub use svg_element::SvgElement;
use wasm_bindgen::{prelude::*, JsCast as _};
pub use web_sys;
pub use components as cmp;

thread_local! {
	static CONTEXT: Context = Default::default();
}

#[derive(Default)]
struct StyleStorage {
	map: RefCell<HashMap<css::Style, u64>>,
}

// TODO: right now if the same style is reused in multiple windows - won't work, need to track style insertion per window
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
		let dom = element.owner_document().unwrap();
		let head = dom.head().unwrap();
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

#[derive(Default)]
pub struct Context {
	style_storage: StyleStorage,
	// classes: RefCell<HashMap<u64, String>>,
}

macro_rules! generate_events {
	($($event_kind:path, $name:ident, $trait:ident, $f:ident);+$(;)*) => {paste::item!{
		pub trait EventTarget: Element {
			fn event_handlers(&self) -> std::cell::RefMut<Vec<EventHandler>>;
			$(
				fn $f(&self, f: impl FnMut($event_kind) + 'static) where Self: Sized {
					use event_raw_exts::*;

					let handler = self.element().$f(f);
					self.event_handlers().push(handler);
				}

				#[allow(clippy::missing_safety_doc)]
				unsafe fn [<unsafe_ $f>]<'a>(&'a self, f: impl FnMut($event_kind) + 'a) where Self: Sized {
					use event_raw_exts::*;

					let handler = self.element().[<unsafe_ $f>](f);
					self.event_handlers().push(handler);
				}
			)+
		}

		pub mod event_raw_exts {
			use super::*;

			$(
				#[extend::ext(pub, name = [<Raw $trait>])]
				impl web_sys::EventTarget {
					#[must_use]
					fn $f(&self, f: impl FnMut($event_kind) + 'static) -> EventHandler where Self: Sized {
						let fbox: Box<dyn FnMut($event_kind) + 'static> = Box::new(f);
						let handler = Closure::wrap(fbox);
						self.add_event_listener_with_callback(web_str::$name(), handler.as_ref().unchecked_ref()).unwrap();
						EventHandler(Box::new(handler))
					}

					#[must_use]
					#[allow(clippy::missing_safety_doc)]
					unsafe fn [<unsafe_ $f>]<'a>(&'a self, f: impl FnMut($event_kind) + 'a) -> EventHandler where Self: Sized {
						let fbox: Box<dyn FnMut($event_kind) + 'a> = Box::new(f);
						let long_fbox: Box<dyn FnMut($event_kind) + 'static> = std::mem::transmute(fbox);
						let handler = Closure::wrap(long_fbox);
						self.add_event_listener_with_callback(web_str::$name(), handler.as_ref().unchecked_ref()).unwrap();
						EventHandler(Box::new(handler))
					}
				}
			)+
		}
	}};
}

pub struct EventHandler(Box<dyn std::any::Any>);
pub type EventHandlers = RefCell<Vec<EventHandler>>;

generate_events! {
	web_sys::MouseEvent,    click,       OnClick,       on_click;
	web_sys::MouseEvent,    contextmenu, OnContextMenu, on_context_menu;
	web_sys::MouseEvent,    dblclick,    OnDblClick,    on_dbl_click;
	web_sys::MouseEvent,    mousedown,   OnMouseDown,   on_mouse_down;
	web_sys::MouseEvent,    mouseenter,  OnMouseEnter,  on_mouse_enter;
	web_sys::MouseEvent,    mouseleave,  OnMouseLeave,  on_mouse_leave;
	web_sys::MouseEvent,    mousemove,   OnMouseMove,   on_mouse_move;
	web_sys::MouseEvent,    mouseover,   OnMouseOver,   on_mouse_over;
	web_sys::MouseEvent,    mouseout,    OnMouseOut,    on_mouse_out;
	web_sys::MouseEvent,    mouseup,     OnMouseUp,     on_mouse_up;
	web_sys::KeyboardEvent, keydown,     OnKeyDown,     on_key_down;
	web_sys::KeyboardEvent, keyup,       OnKeyUp,       on_key_up;
	web_sys::Event,         change,      OnChange,      on_change;
	web_sys::Event,         scroll,      OnScroll,      on_scroll;
}

#[extend::ext(name = RawSetClass)]
impl web_sys::Element {
	fn set_class<'a>(self, style: impl Into<Cow<'a, css::Style>>) {
		CONTEXT.with(move |ctx| {
			let element_class = ctx.style_storage.fetch(&self, style);
			self.set_attribute(web_str::class(), &element_class).unwrap();
		})
	}

	fn add_class<'a>(self, style: impl Into<Cow<'a, css::Style>>) {
		CONTEXT.with(move |ctx| {
			let element_class = ctx.style_storage.fetch(&self, style);
			let existing_class = self.get_attribute(web_str::class()).unwrap_or_else(String::new);
			self.set_attribute(web_str::class(), &format!("{} {}", existing_class, element_class)).unwrap();
		})
	}
}

fn dom() -> web_sys::Document {
	web_sys::window().unwrap().document().unwrap()
}

macro_rules! html_create {
	($($name:ident, $t:ident),+$(,)*) => {
		#[allow(non_snake_case)]
		pub mod create {
			use super::dom;

			$(
				pub fn $name() -> web_sys::$t { web_sys::$t::from(wasm_bindgen::JsValue::from(dom().create_element(crate::web_str::$name()).expect("can't create element"))) }
			)+
		}

		$(
			#[allow(non_snake_case)]
			impl basic_element::BasicElement<web_sys::$t> {
				pub fn $name() -> Self {
					BasicElement { element: create::$name(), children: vec![], event_handlers: EventHandlers::default() }
				}
			}

			#[allow(non_snake_case)]
			impl<'a> cmp::Builder<'a> {
				pub fn $name(self) -> BasicElement<web_sys::$t> {
					self.build(create::$name())
				}
			}
		)+
	};
}

macro_rules! svg_create {
	($($name:ident, $t:ident),+$(,)*) => {
		#[allow(non_snake_case)]
		pub mod create_svg {
			use super::dom;

			$(
				pub fn $name() -> web_sys::$t { web_sys::$t::from(wasm_bindgen::JsValue::from(dom().create_element_ns(Some(wasm_bindgen::intern("http://www.w3.org/2000/svg")), crate::web_str::$name()).expect("can't create svg element"))) }
			)+
		}

		$(
			#[allow(non_snake_case)]
			impl basic_element::BasicElement<web_sys::$t> {
				pub fn $name() -> Self {
					BasicElement { element: create_svg::$name(), children: vec![], event_handlers: EventHandlers::default() }
				}
			}

			#[allow(non_snake_case)]
			impl<'a> cmp::Builder<'a> {
				pub fn $name(self) -> BasicElement<web_sys::$t> {
					self.build_svg(create_svg::$name())
				}
			}
		)+
	};
}

#[rustfmt::skip]
html_create![
	div, HtmlDivElement,
	span, HtmlSpanElement,
	input, HtmlInputElement,
	a, HtmlAnchorElement,
	img, HtmlImageElement,
	textarea, HtmlTextAreaElement,
	script, HtmlScriptElement,
	iframe, HtmlIFrameElement,
	object, HtmlObjectElement,
	embed, HtmlEmbedElement,
	select, HtmlSelectElement,
	option, HtmlOptionElement,
	nav, HtmlElement,
	footer, HtmlElement,
	address, HtmlElement,
	h1, HtmlHeadingElement,
	h2, HtmlHeadingElement,
	h3, HtmlHeadingElement,
	h4, HtmlHeadingElement,
	h5, HtmlHeadingElement,
	h6, HtmlHeadingElement,
	p, HtmlParagraphElement,
];

#[rustfmt::skip]
svg_create![
	svg, SvgsvgElement,
	filter, SvgFilterElement,
	feColorMatrix, SvgfeColorMatrixElement,
];
