use crate::prelude::*;

macro_rules! declare_basic_shortcuts {
	($($name:ident => $element:ident),+$(,)*) => {$(
		pub type $name = crate::BasicElement<web_sys::$element>;
	)+};
}

declare_basic_shortcuts! {
	Div => HtmlDivElement,
	Span => HtmlSpanElement,
	Option => HtmlOptionElement,
	Select => HtmlSelectElement,
	Input => HtmlInputElement,
	Anchor => HtmlAnchorElement,
	IFrame => HtmlIFrameElement,
	Element => HtmlElement,
}

pub fn build<'a>() -> Builder<'a> {
	Builder::default()
}

#[derive(Default)]
pub struct Builder<'a> {
	pub text: std::option::Option<&'a str>,
	pub attributes: std::option::Option<Vec<[&'a str; 2]>>,
	pub class: std::option::Option<std::borrow::Cow<'a, crate::css::Style>>,
	pub children: Vec<Box<dyn crate::Element>>,
}

impl<'a> Builder<'a> {
	pub fn text(mut self, x: &'a str) -> Self {
		self.text = Some(x);
		self
	}

	pub fn class(mut self, x: impl Into<std::borrow::Cow<'a, crate::css::Style>>) -> Self {
		self.class = Some(x.into());
		self
	}

	pub fn attr(mut self, key: &'a str, value: &'a str) -> Self {
		let mut v = self.attributes.unwrap_or_else(Vec::new);
		v.push([key, value]);
		self.attributes = Some(v);
		self
	}

	pub fn child(mut self, child: impl crate::Element + 'static) -> Self {
		self.children.push(Box::new(child));
		self
	}

	pub fn build<T: AsRef<web_sys::Element> + AsRef<web_sys::HtmlElement> + 'static>(self, element: T) -> crate::BasicElement<T> {
		let html_element: &web_sys::HtmlElement = element.as_ref();
		if let Some(x) = self.text { html_element.set_inner_text(x) };
		if let Some(x) = self.attributes {
			for [k, v] in x {
				html_element.set_attribute(k, v).unwrap();
			}
		};
		for child in &self.children {
			html_element.append_child(child.element()).expect("Can't append child");
		}
		let cmp = crate::BasicElement { element, children: self.children, event_handlers: crate::EventHandlers::default() };
		if let Some(x) = self.class { cmp.set_class(x); };
		cmp
	}
}
