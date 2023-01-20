#![allow(non_snake_case)]

pub mod svg;
pub mod html;

use crate::{prelude::*, storage::Storage, AsEntity, AsElement, Entity, World};
use std::{any::TypeId, collections::HashSet};
use sugars::*;

#[cfg(test)] use wasm_bindgen_test::*;
#[cfg(test)] wasm_bindgen_test_configure!(run_in_browser);

pub fn dom_element<T, E>(world: &World, entity: T, element: &E) where
	T: AsEntity,
	E: AsRef<web_sys::Node> + AsRef<web_sys::Element> + AsRef<web_sys::EventTarget>,
{
	let entity = entity.as_entity();
	world.storage_mut::<web_sys::Node>().add(entity, (element.as_ref() as &web_sys::Node).clone());
	world.storage_mut::<web_sys::Element>().add(entity, (element.as_ref() as &web_sys::Element).clone());
	world.storage_mut::<web_sys::EventTarget>().add(entity, (element.as_ref() as &web_sys::EventTarget).clone());
}

struct DomTypes(HashSet<TypeId>);

pub fn register_handlers(world: &World) {
	world.storage_mut::<web_sys::Element>().on_removed = Some(move |_, world, entity, element| {
		world.storage_mut::<web_sys::Node>().remove(entity);
		world.storage_mut::<web_sys::EventTarget>().remove(entity);
		world.storage_mut::<DomTypes>().remove(entity);
		world.storage_mut::<Vec<crate::dom_events::EventHandler>>().remove(entity);
		element.remove();
	});

	world.storage_mut::<DomTypes>().on_removed = Some(move |_, world, entity, dom_types| {
		for t in dom_types.0 {
			// TODO: WARNING: this won't run handlers watching it
			// which isn't a problem for now
			world.storages.map_get(&t, |x| x.borrow_mut()).unwrap().dyn_remove(entity);
		}
	});
}

pub fn html_element<T: AsRef<web_sys::HtmlElement> + 'static + Clone>(element: &T) -> Entity {
	let entity = WORLD.new_entity();

	let html_element = element.as_ref().clone();
	#[cfg(debug_assertions)] html_element.set_attribute(wasm_bindgen::intern("data-entity"), &format!("{}", entity.0)).unwrap();
	dom_element(&WORLD, entity, &html_element);
	WORLD.storage_mut::<web_sys::HtmlElement>().add(entity, html_element);

	if TypeId::of::<web_sys::HtmlElement>() == TypeId::of::<T>() {
		WORLD.storage_mut::<DomTypes>().add(entity, DomTypes(hset![TypeId::of::<web_sys::HtmlElement>()]));
	} else {
		WORLD.storage_mut::<T>().add(entity, element.clone());
		WORLD.storage_mut::<DomTypes>().add(entity, DomTypes(hset![TypeId::of::<web_sys::HtmlElement>(), TypeId::of::<T>()]));
	}

	entity
}

pub fn svg_element<T: AsRef<web_sys::SvgElement> + 'static + Clone>(element: &T) -> Entity {
	let entity = WORLD.new_entity();

	let svg_element = element.as_ref().clone();
	#[cfg(debug_assertions)] svg_element.set_attribute(wasm_bindgen::intern("data-entity"), &format!("{}", entity.0)).unwrap();
	dom_element(&WORLD, entity, &svg_element);
	WORLD.storage_mut::<web_sys::SvgElement>().add(entity, svg_element);

	if TypeId::of::<web_sys::SvgElement>() == TypeId::of::<T>() {
		WORLD.storage_mut::<DomTypes>().add(entity, DomTypes(hset![TypeId::of::<web_sys::SvgElement>()]));
	} else {
		WORLD.storage_mut::<T>().add(entity, element.clone());
		WORLD.storage_mut::<DomTypes>().add(entity, DomTypes(hset![TypeId::of::<web_sys::SvgElement>(), TypeId::of::<T>()]));
	}

	entity
}

macro_rules! create {
	(
		HTML => [$($html_name:ident, $html_t:ident),*$(,)?],
		SVG => [$($svg_name:ident, $svg_t:ident),*$(,)?],
	) => {paste::item! {
		$(
			#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, AsEntity)]
			pub struct [<$html_name:camel>](pub crate::Entity);

			impl AsElement for [<$html_name:camel>] { }

			pub fn $html_name() -> [<$html_name:camel>] {
				let raw: web_sys::$html_t = wasm_bindgen::JsCast::unchecked_into(
					web_sys::window().expect("no window")
						.document().expect("no document")
						.create_element(crate::web_str::$html_name()).expect("can't create element")
				);
				[<$html_name:camel>](html_element(&raw))
			}

			#[test]
			fn [<$html_name _has_selector>]() { crate::css::macros::selector!($html_name); }
		)*

		$(
			#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, AsEntity)]
			pub struct [<$svg_name:camel>](pub crate::Entity);

			impl AsElement for [<$svg_name:camel>] { }

			pub fn $svg_name() -> [<$svg_name:camel>] {
				let raw: web_sys::$svg_t = wasm_bindgen::JsCast::unchecked_into(
					web_sys::window().expect("no window")
						.document().expect("no document")
						.create_element_ns(Some(wasm_bindgen::intern("http://www.w3.org/2000/svg")), crate::web_str::$svg_name()).expect("can't create svg element")
				);
				[<$svg_name:camel>](svg_element(&raw))
			}

			#[test]
			fn [<$svg_name _has_selector>]() { crate::css::macros::selector!($svg_name); }
		)*

		#[doc(hidden)]
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

// impl AsRef<web_sys::HtmlSelectElement> for Select {
//     fn as_ref(&self) -> &web_sys::HtmlSelectElement {
//         <web_sys::HtmlSelectElement as Component>::get(self)
//     }
// }

// TODO: sort out SVG*HTML name collisions
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
		g, SvggElement,
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
