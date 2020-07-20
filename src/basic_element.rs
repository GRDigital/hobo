use super::{Container, Element, EventHandler, EventHandlers, EventTarget};
use crate::prelude::*;
use std::borrow::Cow;

/// This is the most common kind of hobo element - both children and event handlign
pub struct BasicElement<T: AsRef<web_sys::Element> + 'static> {
	pub element: T,
	pub children: Vec<Box<dyn Element>>,
	pub event_handlers: EventHandlers,
}

pub trait RawElement {
	type RawElementType: AsRef<web_sys::Element>;

	fn raw_element(&self) -> &Self::RawElementType;
}

impl<T: AsRef<web_sys::Element> + 'static> RawElement for BasicElement<T> {
	type RawElementType = T;

	fn raw_element(&self) -> &T { &self.element }
}

impl<T: AsRef<web_sys::Element> + 'static> EventTarget for BasicElement<T> {
	fn event_handlers(&self) -> std::cell::RefMut<Vec<EventHandler>> { self.event_handlers.borrow_mut() }
}

impl<T: AsRef<web_sys::Element> + 'static> BasicElement<T> {
	pub fn new(element: T) -> Self {
		Self { element, children: Vec::new(), event_handlers: EventHandlers::default() }
	}
}

impl<T: AsRef<web_sys::Node> + AsRef<web_sys::Element> + 'static + wasm_bindgen::JsCast> BasicElement<T> {
	pub fn clone_html(&self) -> Self {
		let node: &web_sys::Node = self.element.as_ref();
		Self { element: node.clone_node_with_deep(true).expect("can't clone_node_with_deep").dyn_into().expect("can't convert after clone_node_with_deep"), children: Vec::new(), event_handlers: crate::EventHandlers::default() }
	}
}

impl<T: AsRef<web_sys::Element> + 'static> Drop for BasicElement<T> {
	fn drop(&mut self) { self.element.as_ref().remove(); }
}

impl<T: AsRef<web_sys::Element> + 'static> Element for BasicElement<T> {
	fn element(&self) -> Cow<'_, web_sys::Element> { Cow::Borrowed(self.element.as_ref()) }
}

impl<T: AsRef<web_sys::Element> + 'static> Container for BasicElement<T> {
	fn children(&self) -> &Vec<Box<dyn Element>> { &self.children }

	fn children_mut(&mut self) -> &mut Vec<Box<dyn Element>> { &mut self.children }
}
