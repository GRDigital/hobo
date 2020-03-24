use crate::prelude::*;
use super::{Element, EventHandler, EventHandlers, EventTarget};
use std::borrow::Cow;

pub struct BasicElement<T: AsRef<web_sys::Element>> {
	pub element: T,
	pub children: Vec<Box<dyn Element>>,
	pub event_handlers: EventHandlers,
}

impl<T: AsRef<web_sys::Element>> AsRef<T> for BasicElement<T> {
	fn as_ref(&self) -> &T { &self.element }
}

impl<T: AsRef<web_sys::Element>> EventTarget for BasicElement<T> {
	fn event_handlers(&self) -> std::cell::RefMut<Vec<EventHandler>> { self.event_handlers.borrow_mut() }
}

impl<T: AsRef<web_sys::Element>> BasicElement<T> {
	pub fn new(element: T) -> Self {
		Self {
			element,
			children: vec![],
			event_handlers: EventHandlers::default(),
		}
	}

	pub fn attach_child(&mut self, child: impl Element + 'static) {
		self.append(&child);
		self.children.push(Box::new(child));
	}
}

impl<T: AsRef<web_sys::Node> + AsRef<web_sys::Element> + wasm_bindgen::JsCast> BasicElement<T> {
	pub fn clone_html(&self) -> Self {
		let node: &web_sys::Node = self.element.as_ref();
		Self { element: node.clone_node_with_deep(true).unwrap().dyn_into().unwrap(), children: vec![], event_handlers: crate::EventHandlers::default() }
	}
}

impl<T: AsRef<web_sys::Element>> Drop for BasicElement<T> {
	fn drop(&mut self) { self.element.as_ref().remove(); }
}

impl<T: AsRef<web_sys::Element>> Element for BasicElement<T> {
	fn element(&self) -> Cow<'_, web_sys::Element> { Cow::Borrowed(self.element.as_ref()) }
}
