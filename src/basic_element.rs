use super::{Element, EventHandler, EventHandlers, EventTarget};

pub struct BasicElement<T: AsRef<web_sys::Element> + 'static> {
	pub element: T,
	pub children: Vec<Box<dyn Element>>,
	pub event_handlers: EventHandlers,
}

impl<T: AsRef<web_sys::Element> + 'static> AsRef<web_sys::Element> for BasicElement<T> {
	fn as_ref(&self) -> &web_sys::Element { self.element.as_ref() }
}

impl<T: AsRef<web_sys::Element> + 'static> EventTarget for BasicElement<T> {
	fn event_handlers(&self) -> std::cell::RefMut<Vec<EventHandler>> { self.event_handlers.borrow_mut() }
}

impl<T: AsRef<web_sys::Element> + 'static> BasicElement<T> {
	pub fn attach_child(&mut self, child: impl Element + 'static) {
		self.append(&child);
		self.children.push(Box::new(child));
	}
}

impl<T: AsRef<web_sys::Element> + 'static> Drop for BasicElement<T> {
	fn drop(&mut self) { self.element.as_ref().remove(); }
}

impl<T: AsRef<web_sys::Element> + 'static> Element for BasicElement<T> {
	fn element(&self) -> &web_sys::Element { &self.element.as_ref() }
}

impl<T: AsRef<web_sys::Node> + AsRef<web_sys::Element> + wasm_bindgen::JsCast + 'static> Clone for BasicElement<T> {
	fn clone(&self) -> Self {
		use wasm_bindgen::JsCast;

		let node: &web_sys::Node = self.element.as_ref();
		Self { element: node.clone_node_with_deep(true).unwrap().dyn_into().unwrap(), children: vec![], event_handlers: crate::EventHandlers::default() }
	}
}
