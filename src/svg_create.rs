#![allow(non_snake_case)]

use super::{basic_element::BasicElement, dom};

macro_rules! svg_create {
	($($name:ident, $t:ident),+$(,)*) => {
		$(
			pub fn $name() -> web_sys::$t { web_sys::$t::from(wasm_bindgen::JsValue::from(dom().create_element_ns(Some(wasm_bindgen::intern("http://www.w3.org/2000/svg")), crate::web_str::$name()).expect("can't create svg element"))) }

			impl BasicElement<web_sys::$t> {
				pub fn $name() -> Self {
					BasicElement { element: $name(), children: Vec::new(), event_handlers: Default::default() }
				}
			}
		)+

		pub mod components {
			$(
				pub fn $name() -> crate::basic_element::BasicElement<web_sys::$t> {
					crate::basic_element::BasicElement::$name()
				}

				paste::item! {
					pub type [<$name:camel>] = crate::BasicElement<web_sys::$t>;
				}
			)+
		}
	};
}

#[rustfmt::skip]
svg_create![
	svg, SvgsvgElement,
	filter, SvgFilterElement,
	feColorMatrix, SvgfeColorMatrixElement,
];
