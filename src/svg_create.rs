#![allow(non_snake_case)]

use super::{dom, cmp, basic_element::BasicElement, EventHandlers};

macro_rules! svg_create {
	($($name:ident, $t:ident),+$(,)*) => {
		$(
			pub fn $name() -> web_sys::$t { web_sys::$t::from(wasm_bindgen::JsValue::from(dom().create_element_ns(Some(wasm_bindgen::intern("http://www.w3.org/2000/svg")), crate::web_str::$name()).expect("can't create svg element"))) }

			impl BasicElement<web_sys::$t> {
				pub fn $name() -> Self {
					BasicElement { element: $name(), children: vec![], event_handlers: EventHandlers::default() }
				}
			}
		)+

		impl<'a> cmp::Builder<'a> {
			$(
				pub fn $name(self) -> BasicElement<web_sys::$t> {
					self.build_raw($name())
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
