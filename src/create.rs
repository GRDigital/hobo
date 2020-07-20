#![allow(non_snake_case)]

use super::{basic_element::BasicElement, dom};

#[cfg(test)] use wasm_bindgen_test::*;
#[cfg(test)] wasm_bindgen_test_configure!(run_in_browser);

macro_rules! create {
	(
		HTML => [
			$($html_name:ident, $html_t:ident),*$(,)?
		],
		SVG => [
			$($svg_name:ident, $svg_t:ident),*$(,)?
		],
	) => {paste::item! {
		$(
			pub fn $html_name() -> web_sys::$html_t { wasm_bindgen::JsCast::unchecked_into(dom().create_element(crate::web_str::$html_name()).expect("can't create element")) }

			impl BasicElement<web_sys::$html_t> {
				pub fn $html_name() -> Self {
					BasicElement { element: $html_name(), children: Vec::new(), event_handlers: Default::default() }
				}
			}

			#[cfg(test)]
			#[wasm_bindgen_test]
			fn [<can_create_$html_name>]() {
				components::$html_name();
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

				pub type [<$html_name:camel>] = crate::BasicElement<web_sys::$html_t>;

				#[test]
				fn [<$html_name _has_selector>]() {
					crate::css::css_macros_decl::selector!($html_name);
				}
			)*

			$(
				pub fn $svg_name() -> crate::basic_element::BasicElement<web_sys::$svg_t> {
					crate::basic_element::BasicElement::$svg_name()
				}

				pub type [<$svg_name:camel>] = crate::BasicElement<web_sys::$svg_t>;
			)*
		}

		pub mod strings {
			$(
				pub fn $html_name() -> &'static str {
					#[cfg(debug_assertions)]
					{stringify!($html_name)}

					#[cfg(not(debug_assertions))]
					{wasm_bindgen::intern(stringify!($html_name))}
				}
			)*

			$(
				pub fn $svg_name() -> &'static str {
					#[cfg(debug_assertions)]
					{stringify!($svg_name)}

					#[cfg(not(debug_assertions))]
					{wasm_bindgen::intern(stringify!($svg_name))}
				}
			)*
		}
	}};
}

#[rustfmt::skip]
create![
	HTML => [
		html, HtmlHtmlElement,
		base, HtmlBaseElement,
		head, HtmlHeadElement,
		link, HtmlLinkElement,
		meta, HtmlMetaElement,
		style, HtmlStyleElement,
		title, HtmlTitleElement,

		body, HtmlBodyElement,

		address, HtmlElement,
		article, HtmlElement,
		aside, HtmlElement,
		footer, HtmlElement,
		header, HtmlElement,
		h1, HtmlHeadingElement,
		h2, HtmlHeadingElement,
		h3, HtmlHeadingElement,
		h4, HtmlHeadingElement,
		h5, HtmlHeadingElement,
		h6, HtmlHeadingElement,
		// hgroup, HtmlElement,
		main, HtmlElement,
		nav, HtmlElement,
		section, HtmlElement,

		blockquote, HtmlQuoteElement,
		dd, HtmlElement,
		div, HtmlDivElement,
		dl, HtmlDListElement,
		dt, HtmlElement,
		figcaption, HtmlElement,
		figure, HtmlElement,
		hr, HtmlHrElement,
		li, HtmlLiElement,
		ol, HtmlOListElement,
		p, HtmlParagraphElement,
		pre, HtmlPreElement,
		ul, HtmlUListElement,

		a, HtmlAnchorElement,
		abbr, HtmlElement,
		b, HtmlElement,
		bdi, HtmlElement,
		bdo, HtmlElement,
		br, HtmlBrElement,
		cite, HtmlElement,
		code, HtmlElement,
		data, HtmlDataElement,
		dfn, HtmlElement,
		em, HtmlElement,
		i, HtmlElement,
		kbd, HtmlElement,
		mark, HtmlElement,
		q, HtmlQuoteElement,
		rb, HtmlElement,
		rp, HtmlElement,
		rt, HtmlElement,
		rtc, HtmlElement,
		ruby, HtmlElement,
		s, HtmlElement,
		samp, HtmlElement,
		small, HtmlElement,
		span, HtmlSpanElement,
		strong, HtmlElement,
		sub, HtmlElement,
		sup, HtmlElement,
		time, HtmlTimeElement,
		u, HtmlElement,
		var, HtmlElement,
		wbr, HtmlElement,

		area, HtmlAreaElement,
		audio, HtmlAudioElement,
		img, HtmlImageElement,
		map, HtmlMapElement,
		track, HtmlTrackElement,
		video, HtmlVideoElement,

		embed, HtmlEmbedElement,
		iframe, HtmlIFrameElement,
		object, HtmlObjectElement,
		param, HtmlParamElement,
		picture, HtmlPictureElement,
		source, HtmlSourceElement,

		canvas, HtmlCanvasElement,
		noscript, HtmlElement,
		script, HtmlScriptElement,

		del, HtmlModElement,
		ins, HtmlModElement,

		caption, HtmlTableCaptionElement,
		col, HtmlTableColElement,
		colgroup, HtmlTableColElement,
		table, HtmlTableElement,
		tbody, HtmlTableSectionElement,
		td, HtmlTableCellElement,
		tfoot, HtmlTableSectionElement,
		th, HtmlTableCellElement,
		thead, HtmlTableSectionElement,
		tr, HtmlTableRowElement,

		button, HtmlButtonElement,
		datalist, HtmlDataListElement,
		fieldset, HtmlFieldSetElement,
		form, HtmlFormElement,
		input, HtmlInputElement,
		label, HtmlLabelElement,
		legend, HtmlLegendElement,
		meter, HtmlMeterElement,
		optgroup, HtmlOptGroupElement,
		option, HtmlOptionElement,
		output, HtmlOutputElement,
		progress, HtmlProgressElement,
		select, HtmlSelectElement,
		textarea, HtmlTextAreaElement,

		details, HtmlDetailsElement,
		dialog, HtmlDialogElement,
		menu, HtmlMenuElement,
		summary, HtmlElement,

		slot, HtmlSlotElement,
		template, HtmlTemplateElement,
	],
	SVG => [
		svg, SvgsvgElement,
		// a, SvgAElement,
		animate, SvgAnimateElement,
		animateMotion, SvgAnimateMotionElement,
		animateTransform, SvgAnimateTransformElement,
		circle, SvgCircleElement,
		clipPath, SvgClipPathElement,
		// color-profile, color-SvgProfileElement,
		defs, SvgDefsElement,
		desc, SvgDescElement,
		// discard, SvgDiscardElement,
		ellipse, SvgEllipseElement,
		feBlend, SvgfeBlendElement,
		feColorMatrix, SvgfeColorMatrixElement,
		feComponentTransfer, SvgfeComponentTransferElement,
		feComposite, SvgfeCompositeElement,
		feConvolveMatrix, SvgfeConvolveMatrixElement,
		feDiffuseLighting, SvgfeDiffuseLightingElement,
		feDisplacementMap, SvgfeDisplacementMapElement,
		feDistantLight, SvgfeDistantLightElement,
		feDropShadow, SvgfeDropShadowElement,
		feFlood, SvgfeFloodElement,
		feFuncA, SvgfeFuncAElement,
		feFuncB, SvgfeFuncBElement,
		feFuncG, SvgfeFuncGElement,
		feFuncR, SvgfeFuncRElement,
		feGaussianBlur, SvgfeGaussianBlurElement,
		feImage, SvgfeImageElement,
		feMerge, SvgfeMergeElement,
		feMergeNode, SvgfeMergeNodeElement,
		feMorphology, SvgfeMorphologyElement,
		feOffset, SvgfeOffsetElement,
		fePointLight, SvgfePointLightElement,
		feSpecularLighting, SvgfeSpecularLightingElement,
		feSpotLight, SvgfeSpotLightElement,
		feTile, SvgfeTileElement,
		feTurbulence, SvgfeTurbulenceElement,
		filter, SvgFilterElement,
		foreignObject, SvgForeignObjectElement,
		gSvg, SvggElement,
		// hatch, SvgHatchElement,
		// hatchpath, SvgHatchpathElement,
		image, SvgImageElement,
		line, SvgLineElement,
		linearGradient, SvgLinearGradientElement,
		marker, SvgMarkerElement,
		mask, SvgMaskElement,
		// mesh, SvgMeshElement,
		// meshgradient, SvgMeshgradientElement,
		// meshpatch, SvgMeshpatchElement,
		// meshrow, SvgMeshrowElement,
		metadata, SvgMetadataElement,
		mpath, SvgmPathElement,
		path, SvgPathElement,
		pattern, SvgPatternElement,
		polygon, SvgPolygonElement,
		polyline, SvgPolylineElement,
		radialGradient, SvgRadialGradientElement,
		rect, SvgRectElement,
		// script, SvgScriptElement,
		set, SvgSetElement,
		stop, SvgStopElement,
		// style, SvgStyleElement,
		switch, SvgSwitchElement,
		symbol, SvgSymbolElement,
		text, SvgTextElement,
		textPath, SvgTextPathElement,
		// title, SvgTitleElement,
		tspan, SvgtSpanElement,
		r#use, SvgUseElement,
		view, SvgViewElement,
	],
];
