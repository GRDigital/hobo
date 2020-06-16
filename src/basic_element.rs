use crate::prelude::*;
use super::{Element, EventHandler, EventHandlers, EventTarget, Container};
use std::borrow::Cow;

pub trait Bound = AsRef<web_sys::Element> + 'static;

pub struct BasicElement<T: Bound> {
	pub element: T,
	pub children: Vec<Box<dyn Element>>,
	pub event_handlers: EventHandlers,
}

// impl<T: Bound> AsRef<web_sys::Element> for BasicElement<T> {
//     fn as_ref(&self) -> &web_sys::Element { &self.element }
// }

impl<T: Bound> AsRef<T> for BasicElement<T> {
	fn as_ref(&self) -> &T { &self.element }
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

	pub fn with_child(mut self, child: impl crate::Element + 'static) -> Self {
		self.element().append_child(&child.element()).expect("can't append child");
		self.children_mut().push(Box::new(child));
		self
	}

	pub fn wth_child_ref(self, child: &(impl crate::Element + 'static)) -> Self {
		self.element().append_child(&child.element()).expect("can't append child");
		self
	}

	pub fn with_text<'a>(self, x: impl Into<Cow<'a, str>>) -> Self { self.element().unchecked_ref::<web_sys::HtmlElement>().set_inner_text(&x.into()); self }
	pub fn with_class<'a>(self, x: impl Into<Cow<'a, crate::css::AtRules>>) -> Self { self.set_class(x.into()); self }
	pub fn with_style<'a>(self, x: impl Into<Cow<'a, [crate::css::Property]>>) -> Self { self.set_style(x.into()); self }
	pub fn with_attr<'a>(self, key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> Self { self.element().set_attribute(&key.into(), &value.into()).expect("can't set attribute"); self }
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
