#![allow(non_snake_case)]

use super::{basic_element::BasicElement, dom};

macro_rules! html_create {
	(
		HTML => [
			$($html_name:ident, $html_t:ident),*$(,)?
		],
		SVG => [
			$($svg_name:ident, $svg_t:ident),*$(,)?
		],
	) => {
		$(
			pub fn $html_name() -> web_sys::$html_t { wasm_bindgen::JsCast::unchecked_into(dom().create_element(crate::web_str::$html_name()).expect("can't create element")) }

			impl BasicElement<web_sys::$html_t> {
				pub fn $html_name() -> Self {
					BasicElement { element: $html_name(), children: Vec::new(), event_handlers: Default::default() }
				}
			}
		)*

		$(
			pub fn $svg_name() -> web_sys::$svg_t { wasm_bindgen::JsCast::unchecked_into(dom().create_element_ns(Some(wasm_bindgen::intern("http://www.w3.org/2000/svg")), crate::web_str::$svg_name()).expect("can't create svg element")) }

			impl BasicElement<web_sys::$svg_t> {
				pub fn $svg_name() -> Self {
					BasicElement { element: $svg_name(), children: Vec::new(), event_handlers: Default::default() }
				}
			}
		)*

		pub mod components {
			$(
				pub fn $html_name() -> crate::basic_element::BasicElement<web_sys::$html_t> {
					crate::basic_element::BasicElement::$html_name()
				}

				paste::item! {
					pub type [<$html_name:camel>] = crate::BasicElement<web_sys::$html_t>;
				}
			)*

			$(
				pub fn $svg_name() -> crate::basic_element::BasicElement<web_sys::$svg_t> {
					crate::basic_element::BasicElement::$svg_name()
				}

				paste::item! {
					pub type [<$svg_name:camel>] = crate::BasicElement<web_sys::$svg_t>;
				}
			)*
		}
	};
}

#[rustfmt::skip]
html_create![
	HTML => [
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
	],
	SVG => [
		svg, SvgsvgElement,
		filter, SvgFilterElement,
		feColorMatrix, SvgfeColorMatrixElement,
		circle, SvgCircleElement,
		clipPath, SvgClipPathElement,
		defs, SvgDefsElement,
		desc, SvgDescElement,
		ellipse, SvgEllipseElement,
		g, SvggElement,
		line, SvgLineElement,
		path, SvgPathElement,
		polygon, SvgPolygonElement,
		polyline, SvgPolygonElement,
		rect, SvgRectElement,
	],
];
