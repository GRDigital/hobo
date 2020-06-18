use super::{Container, Element, EventHandler, EventHandlers, EventTarget};
use crate::prelude::*;
use std::borrow::Cow;

pub trait Bound = AsRef<web_sys::Element> + 'static;

pub struct BasicElement<T: Bound> {
	pub element: T,
	pub children: Vec<Box<dyn Element>>,
	pub event_handlers: EventHandlers,
}

pub trait RawElement {
	type RawElementType: AsRef<web_sys::Element>;

	fn raw_element(&self) -> &Self::RawElementType;
}

impl<T: Bound> RawElement for BasicElement<T> {
	type RawElementType = T;

	fn raw_element(&self) -> &T { &self.element }
}

impl<T: Bound> EventTarget for BasicElement<T> {
	fn event_handlers(&self) -> std::cell::RefMut<Vec<EventHandler>> { self.event_handlers.borrow_mut() }
}

impl<T: Bound> BasicElement<T> {
	pub fn new(element: T) -> Self {
		Self {
			element,
			children: vec![],
			event_handlers: EventHandlers::default(),
		}
	}

	pub fn text<'a>(self, x: impl Into<Cow<'a, str>>) -> Self { self.element().unchecked_ref::<web_sys::HtmlElement>().set_inner_text(&x.into()); self }
}

impl<T: AsRef<web_sys::Node> + Bound + wasm_bindgen::JsCast> BasicElement<T> {
	pub fn clone_html(&self) -> Self {
		let node: &web_sys::Node = self.element.as_ref();
		Self { element: node.clone_node_with_deep(true).unwrap().dyn_into().unwrap(), children: vec![], event_handlers: crate::EventHandlers::default() }
	}
}

impl<T: Bound> Drop for BasicElement<T> {
	fn drop(&mut self) { self.element.as_ref().remove(); }
}

impl<T: Bound> Element for BasicElement<T> {
	fn element(&self) -> Cow<'_, web_sys::Element> { Cow::Borrowed(self.element.as_ref()) }
}

impl<T: Bound> Container for BasicElement<T> {
	fn children(&self) -> &Vec<Box<dyn Element>> { &self.children }

	fn children_mut(&mut self) -> &mut Vec<Box<dyn Element>> { &mut self.children }
}
