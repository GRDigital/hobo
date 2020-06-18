#![allow(non_snake_case)]

use super::{basic_element::BasicElement, dom, EventHandlers};

macro_rules! html_create {
	($($name:ident, $t:ident),+$(,)*) => {
		$(
			pub fn $name() -> web_sys::$t { web_sys::$t::from(wasm_bindgen::JsValue::from(dom().create_element(crate::web_str::$name()).expect("can't create element"))) }

			impl BasicElement<web_sys::$t> {
				pub fn $name() -> Self {
					BasicElement { element: $name(), children: Vec::new(), event_handlers: EventHandlers::default() }
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
	ul, HtmlUListElement,
	li, HtmlLiElement,
	main, HtmlElement,
	button, HtmlButtonElement,
	label, HtmlLabelElement,
];
