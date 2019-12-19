use super::{Element, EventHandlers, EventHandler, EventTarget};

pub struct BasicElement<T: AsRef<web_sys::Element>> {
	pub element: T,
	pub children: Vec<Box<dyn Element>>,
	pub event_handlers: EventHandlers,
}

impl<T: AsRef<web_sys::Element>> AsRef<web_sys::Element> for BasicElement<T> {
	fn as_ref(&self) -> &web_sys::Element {
		self.element.as_ref()
	}
}

impl<T: AsRef<web_sys::Element>> EventTarget for BasicElement<T> {
	fn event_handlers(&self) -> std::cell::RefMut<Vec<EventHandler>> {
		self.event_handlers.borrow_mut()
	}
}

impl<T: AsRef<web_sys::Element>> BasicElement<T> {
	pub fn attach_child(&mut self, child: impl Element + 'static) {
		self.append(&child);
		self.children.push(Box::new(child));
	}
}

impl<T: AsRef<web_sys::Element>> Drop for BasicElement<T> {
	fn drop(&mut self) {
		self.element.as_ref().remove();
	}
}

impl<T: AsRef<web_sys::Element>> Element for BasicElement<T> {
	fn element(&self) -> &web_sys::Element { &self.element.as_ref() }
}
