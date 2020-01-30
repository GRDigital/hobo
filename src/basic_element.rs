use super::{Element, EventHandler, EventHandlers, EventTarget};

pub struct BasicElement<T: AsRef<web_sys::Element>> {
	pub element: T,
	pub children: Vec<Box<dyn Element>>,
	pub event_handlers: EventHandlers,
}

impl<T: AsRef<web_sys::Element>> AsRef<web_sys::Element> for BasicElement<T> {
	fn as_ref(&self) -> &web_sys::Element { self.element.as_ref() }
}

impl<T: AsRef<web_sys::Element>> EventTarget for BasicElement<T> {
	fn event_handlers(&self) -> std::cell::RefMut<Vec<EventHandler>> { self.event_handlers.borrow_mut() }
}

impl<T: AsRef<web_sys::Element>> BasicElement<T> {
	pub fn attach_child(&mut self, child: impl Element + 'static) {
		self.append(&child);
		self.children.push(Box::new(child));
	}
}

impl<T: AsRef<web_sys::Element>> Drop for BasicElement<T> {
	fn drop(&mut self) { self.element.as_ref().remove(); }
}

impl<T: AsRef<web_sys::Element>> Element for BasicElement<T> {
	fn element(&self) -> &web_sys::Element { &self.element.as_ref() }
}

impl<T: AsRef<web_sys::Element> + AsRef<web_sys::HtmlElement>> BasicElement<T> {
	pub fn builder<'a>() -> BasicElementBuilder<'a, T> {
		BasicElementBuilder {
			text: None,
			attributes: None,
			style: None,
			pd: std::marker::PhantomData,
		}
	}
}

pub struct BasicElementBuilder<'a, T: AsRef<web_sys::Element> + AsRef<web_sys::HtmlElement>> {
	pub text: Option<&'a str>,
	pub attributes: Option<Vec<[&'a str; 2]>>,
	pub style: Option<crate::css::Style>,
	pd: std::marker::PhantomData<T>,
}

impl<'a, T: AsRef<web_sys::Element> + AsRef<web_sys::HtmlElement> + 'static> BasicElementBuilder<'a, T> {
	pub fn text(mut self, x: &'a str) -> Self {
		self.text = Some(x);
		self
	}

	pub fn style(mut self, x: crate::css::Style) -> Self {
		self.style = Some(x);
		self
	}

	pub fn attr(mut self, key: &'a str, value: &'a str) -> Self {
		let mut v = self.attributes.unwrap_or_else(Vec::new);
		v.push([key, value]);
		self.attributes = Some(v);
		self
	}

	pub fn build(self, element: T) -> BasicElement<T> {
		let html_element: &web_sys::HtmlElement = element.as_ref();
		if let Some(x) = self.text { html_element.set_inner_text(x) };
		if let Some(x) = self.attributes {
			for [k, v] in x {
				html_element.set_attribute(k, v).unwrap();
			}
		};
		let cmp = BasicElement { element, children: vec![], event_handlers: EventHandlers::default() };
		if let Some(x) = self.style { cmp.set_class(&x); };
		cmp
	}
}
