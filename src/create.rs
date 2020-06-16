#![allow(non_snake_case)]

use super::{dom, cmp, basic_element::BasicElement, EventHandlers};

macro_rules! html_create {
	($($name:ident, $t:ident),+$(,)*) => {paste::item!{
		$(
			pub fn [<raw_$name>]() -> web_sys::$t { web_sys::$t::from(wasm_bindgen::JsValue::from(dom().create_element(crate::web_str::$name()).expect("can't create element"))) }

			impl BasicElement<web_sys::$t> {
				pub fn $name() -> Self {
					BasicElement { element: [<raw_$name>](), children: Vec::new(), event_handlers: EventHandlers::default() }
				}
			}
		)+

		$(
			pub fn $name<'a>() -> cmp::Builder<'a, web_sys::$t> {
				cmp::Builder::new([<raw_$name>]())
			}
		)+

		// impl<'a> cmp::Builder<'a> {
		//     $(
		//         pub fn $name(self) -> BasicElement<web_sys::$t> {
		//             self.build_raw($name())
		//         }
		//     )+
		// }
	}};
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
