#![allow(non_snake_case)]

//! everything that has to do with raw HTML elements
//!
//! all of these functions return the most fitting web_sys element types

use crate::{dom, prelude::*, World, Entity, storage::{Storage, DynStorage}, Element, Component, AsEntity};
use std::collections::HashSet;
use std::any::TypeId;
use sugars::*;

#[cfg(test)] use wasm_bindgen_test::*;
#[cfg(test)] wasm_bindgen_test_configure!(run_in_browser);

pub fn dom_element<T, E>(world: &'static World, entity: T, element: &E) where
	T: AsEntity,
	E: AsRef<web_sys::Node> + AsRef<web_sys::Element> + AsRef<web_sys::EventTarget>
{
	let entity = entity.as_entity();
	world.storage_mut::<web_sys::Node>().add(entity, (element.as_ref() as &web_sys::Node).clone());
	world.storage_mut::<web_sys::Element>().add(entity, (element.as_ref() as &web_sys::Element).clone());
	world.storage_mut::<web_sys::EventTarget>().add(entity, (element.as_ref() as &web_sys::EventTarget).clone());
}

struct DomTypes(HashSet<TypeId>);

pub fn register_systems(world: &World) {
	world.new_system(<(Removed<(web_sys::Element,)>,)>::run(move |entity| {
		log::info!("element remove start {:?}", entity);
		WORLD.storage_mut::<web_sys::Element>().take_removed(entity).unwrap().remove();
		WORLD.storage_mut::<web_sys::Node>().remove(entity);
		WORLD.storage_mut::<web_sys::EventTarget>().remove(entity);
		DomTypes::storage_mut().remove(entity);
		WORLD.storage_mut::<Vec<crate::events::EventHandler>>().remove(entity);
		log::info!("element remove end {:?}", entity);
	}));
	world.new_system(<(Removed<(DomTypes,)>,)>::run(move |entity| {
		log::info!("domtypes remove start {:?}", entity);
		for t in DomTypes::storage_mut().take_removed(entity).unwrap().0 {
			WORLD.storages.borrow_mut()[&t].borrow_mut().dyn_remove(entity);
		}
		log::info!("domtypes remove end {:?}", entity);
	}));
}

pub fn html_element<T: AsRef<web_sys::HtmlElement> + Component + Clone>(element: &T) -> Entity {
	let entity = WORLD.new_entity();
	let html_element = element.as_ref().clone();
	dom_element(&WORLD, entity, &html_element);
	web_sys::HtmlElement::storage_mut().add(entity, html_element);

	if TypeId::of::<web_sys::HtmlElement>() == TypeId::of::<T>() {
		DomTypes::storage_mut().add(entity, DomTypes(hset![TypeId::of::<web_sys::HtmlElement>()]));
	} else {
		T::storage_mut().add(entity, element.clone());
		DomTypes::storage_mut().add(entity, DomTypes(hset![TypeId::of::<web_sys::HtmlElement>(), TypeId::of::<T>()]));
	}

	entity
}

pub fn svg_element<T: AsRef<web_sys::SvgElement> + Component + Clone>(element: &T) -> Entity {
	let entity = WORLD.new_entity();
	let svg_element = element.as_ref().clone();
	dom_element(&WORLD, entity, &svg_element);
	web_sys::SvgElement::storage_mut().add(entity, svg_element);

	if TypeId::of::<web_sys::SvgElement>() == TypeId::of::<T>() {
		DomTypes::storage_mut().add(entity, DomTypes(hset![TypeId::of::<web_sys::SvgElement>()]));
	} else {
		T::storage_mut().add(entity, element.clone());
		DomTypes::storage_mut().add(entity, DomTypes(hset![TypeId::of::<web_sys::SvgElement>(), TypeId::of::<T>()]));
	}

	entity
}

macro_rules! create {
	(
		HTML => [$($html_name:ident, $html_t:ident),*$(,)?],
		SVG => [$($svg_name:ident, $svg_t:ident),*$(,)?],
	) => {paste::item! {
		$(
			pub fn $html_name() -> web_sys::$html_t { wasm_bindgen::JsCast::unchecked_into(dom().create_element(crate::web_str::$html_name()).expect("can't create element")) }

			#[cfg(test)]
			#[wasm_bindgen_test]
			fn [<can_create_$html_name>]() { components::$html_name(); }
		)*

		$(
			pub fn $svg_name() -> web_sys::$svg_t { wasm_bindgen::JsCast::unchecked_into(dom().create_element_ns(Some(wasm_bindgen::intern("http://www.w3.org/2000/svg")), crate::web_str::$svg_name()).expect("can't create svg element")) }

			#[cfg(test)]
			#[wasm_bindgen_test]
			fn [<can_create_$svg_name>]() { components::$svg_name(); }
		)*

		pub mod components {
			use super::*;

			$(
				#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Element)]
				pub struct [<$html_name:camel>](pub crate::Entity);

				pub fn $html_name() -> [<$html_name:camel>] { [<$html_name:camel>](html_element(&super::$html_name())) }

				#[test]
				fn [<$html_name _has_selector>]() { crate::css::macros::selector!($html_name); }
			)*

			$(
				#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Element)]
				pub struct [<$svg_name:camel>](pub crate::Entity);

				pub fn $svg_name() -> [<$svg_name:camel>] { [<$svg_name:camel>](svg_element(&super::$svg_name())) }

				#[test]
				fn [<$svg_name _has_selector>]() { crate::css::macros::selector!($svg_name); }
			)*
		}

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

pub trait StringValue {
	fn value(&self) -> String;
	fn set_value(&self, x: &str);
}

impl StringValue for components::Input {
	fn value(&self) -> String { <web_sys::HtmlInputElement as Component>::get(self).value() }
	fn set_value(&self, x: &str) { <web_sys::HtmlInputElement as Component>::get(self).set_value(x) }
}

impl StringValue for components::Textarea {
	fn value(&self) -> String { <web_sys::HtmlTextAreaElement as Component>::get(self).value() }
	fn set_value(&self, x: &str) { <web_sys::HtmlTextAreaElement as Component>::get(self).set_value(x) }
}

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
