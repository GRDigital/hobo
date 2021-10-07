#![allow(non_snake_case)]

//! everything that has to do with raw HTML elements
//!
//! all of these functions return the most fitting web_sys element types

use crate::{dom, prelude::*, World, Entity, storage::Storage, Element, AsEntity};
use std::collections::HashSet;
use std::any::TypeId;
use sugars::*;

#[cfg(test)] use wasm_bindgen_test::*;
#[cfg(test)] wasm_bindgen_test_configure!(run_in_browser);

pub fn dom_element<T, E>(world: &mut World, entity: T, element: &E) where
	T: AsEntity,
	E: AsRef<web_sys::Node> + AsRef<web_sys::Element> + AsRef<web_sys::EventTarget>
{
	let entity = entity.as_entity();
	world.storage_mut::<web_sys::Node>().add(entity, (element.as_ref() as &web_sys::Node).clone());
	world.storage_mut::<web_sys::Element>().add(entity, (element.as_ref() as &web_sys::Element).clone());
	world.storage_mut::<web_sys::EventTarget>().add(entity, (element.as_ref() as &web_sys::EventTarget).clone());
}

struct DomTypes(HashSet<TypeId>);

pub fn register_systems(world: &mut World) {
	world.new_system(<(Removed<(web_sys::Element,)>,)>::run(move |world, entity| {
		// World::mark_borrow_mut();
		// let world = unsafe { &mut *WORLD.get() as &mut World };
		world.storage_mut::<web_sys::Element>().take_removed(entity).unwrap().remove();
		world.storage_mut::<web_sys::Node>().remove(entity);
		world.storage_mut::<web_sys::EventTarget>().remove(entity);
		world.storage_mut::<DomTypes>().remove(entity);
		world.storage_mut::<Vec<crate::dom_events::EventHandler>>().remove(entity);
		// World::unmark_borrow_mut();
	}));
	world.new_system(<(Removed<(DomTypes,)>,)>::run(move |world, entity| {
		// World::mark_borrow_mut();
		// let world = unsafe { &mut *WORLD.get() as &mut World };
		let removeds = world.storage_mut::<DomTypes>().take_removed(entity).unwrap().0;
		for t in removeds {
			// TODO: WARNING: this won't notify systems watching it
			// which isn't a problem for now
			world.storages[&t].borrow_mut().dyn_remove(entity);
		}
		// World::unmark_borrow_mut();
	}));
}

pub fn html_element<T: AsRef<web_sys::HtmlElement> + 'static + Clone>(element: &T) -> Entity {
	World::mark_borrow_mut();
	let mut world = unsafe { &mut *WORLD.get() as &mut World };
	let entity = world.new_entity();

	let html_element = element.as_ref().clone();
	dom_element(&mut world, entity, &html_element);
	world.storage_mut::<web_sys::HtmlElement>().add(entity, html_element);

	if TypeId::of::<web_sys::HtmlElement>() == TypeId::of::<T>() {
		world.storage_mut::<DomTypes>().add(entity, DomTypes(hset![TypeId::of::<web_sys::HtmlElement>()]));
	} else {
		world.storage_mut::<T>().add(entity, element.clone());
		world.storage_mut::<DomTypes>().add(entity, DomTypes(hset![TypeId::of::<web_sys::HtmlElement>(), TypeId::of::<T>()]));
	}

	World::unmark_borrow_mut();
	entity
}

pub fn svg_element<T: AsRef<web_sys::SvgElement> + 'static + Clone>(element: &T) -> Entity {
	World::mark_borrow_mut();
	let mut world = unsafe { &mut *WORLD.get() as &mut World };
	let entity = world.new_entity();
	let svg_element = element.as_ref().clone();
	dom_element(&mut world, entity, &svg_element);
	world.storage_mut::<web_sys::SvgElement>().add(entity, svg_element);

	if TypeId::of::<web_sys::SvgElement>() == TypeId::of::<T>() {
		world.storage_mut::<DomTypes>().add(entity, DomTypes(hset![TypeId::of::<web_sys::SvgElement>()]));
	} else {
		world.storage_mut::<T>().add(entity, element.clone());
		world.storage_mut::<DomTypes>().add(entity, DomTypes(hset![TypeId::of::<web_sys::SvgElement>(), TypeId::of::<T>()]));
	}
	World::unmark_borrow_mut();
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
	fn value(&self) -> String { self.get_cmp::<web_sys::HtmlInputElement>().value() }
	fn set_value(&self, x: &str) { self.get_cmp::<web_sys::HtmlInputElement>().set_value(x) }
}

impl StringValue for components::Textarea {
	fn value(&self) -> String { self.get_cmp::<web_sys::HtmlTextAreaElement>().value() }
	fn set_value(&self, x: &str) { self.get_cmp::<web_sys::HtmlTextAreaElement>().set_value(x) }
}

impl components::Select {
	pub fn selected_index(&self) -> i32 {
		self.get_cmp::<web_sys::HtmlSelectElement>().selected_index()
	}
}

impl components::Input {
	pub async fn file_data(&self, id: u32) -> Option<Vec<u8>> {
		let file = self.get_cmp::<web_sys::HtmlInputElement>().files()?.get(id)?;
		let arr_buffer: js_sys::ArrayBuffer = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await.ok()?.dyn_into().ok()?;
		let vec = js_sys::Uint8Array::new(&arr_buffer).to_vec();
		Some(vec)
	}
}

// impl AsRef<web_sys::HtmlSelectElement> for components::Select {
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
