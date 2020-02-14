use crate::prelude::*;
pub use builder::*;

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
	Svg => SvgElement,
}

pub fn build<'a>() -> Builder<'a> {
	Builder::default()
}

pub mod builder {
	use crate::prelude::*;

	pub enum BuilderChild<'a> {
		Owned(Box<dyn crate::Element>),
		Ref(&'a dyn crate::Element),
	}

	#[derive(Default)]
	pub struct Builder<'a> {
		pub text: Option<&'a str>,
		pub attributes: Option<Vec<[&'a str; 2]>>,
		pub class: Option<std::borrow::Cow<'a, crate::css::Style>>,
		pub style: Option<Vec<crate::css::Property>>,
		pub children: Vec<BuilderChild<'a>>,
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

		pub fn style(mut self, x: impl Into<Vec<crate::css::Property>>) -> Self {
			self.style = Some(x.into());
			self
		}

		pub fn attr(mut self, key: &'a str, value: &'a str) -> Self {
			let mut v = self.attributes.unwrap_or_else(Vec::new);
			v.push([key, value]);
			self.attributes = Some(v);
			self
		}

		pub fn child(mut self, child: impl crate::Element + 'static) -> Self {
			self.children.push(BuilderChild::Owned(Box::new(child)));
			self
		}

		pub fn child_ref(mut self, child: &'a (impl crate::Element + 'static)) -> Self {
			self.children.push(BuilderChild::Ref(child));
			self
		}

		pub fn build<T: AsRef<web_sys::Element> + AsRef<web_sys::HtmlElement> + 'static, E: std::borrow::BorrowMut<crate::BasicElement<T>>>(self, mut element: E) -> E {
			{
				let element = element.borrow_mut();
				let html_element: &web_sys::HtmlElement = element.element.as_ref();
				if let Some(x) = self.text { html_element.set_inner_text(x) };
				if let Some(x) = self.attributes {
					for [k, v] in x {
						html_element.set_attribute(k, v).unwrap();
					}
				};
				for child in self.children.into_iter() {
					match child {
						BuilderChild::Owned(x) => {
							element.append(&*x);
							element.children.push(x);
						},
						BuilderChild::Ref(x) => element.append(x),
					}
				}
				if let Some(x) = self.class { element.add_class(x); };
				if let Some(x) = self.style { element.set_style(x); };
			}
			element
		}

		pub fn build_svg<T: AsRef<web_sys::Element> + AsRef<web_sys::SvgElement> + 'static, E: std::borrow::BorrowMut<crate::BasicElement<T>>>(self, mut element: E) -> E {
			{
				let element = element.borrow_mut();
				let svg_element: &web_sys::SvgElement = element.element.as_ref();
				if let Some(x) = self.attributes {
					for [k, v] in x {
						svg_element.set_attribute(k, v).unwrap();
					}
				};
				for child in self.children.into_iter() {
					match child {
						BuilderChild::Owned(x) => {
							element.append(&*x);
							element.children.push(x);
						},
						BuilderChild::Ref(x) => element.append(x),
					}
				}
				if let Some(x) = self.class { element.add_class(x); };
				if let Some(x) = self.style { element.set_style(x); };
			}
			element
		}

		pub fn build_raw<T: AsRef<web_sys::Element> + AsRef<web_sys::HtmlElement> + 'static>(self, element: T) -> crate::BasicElement<T> {
			let html_element: &web_sys::HtmlElement = element.as_ref();
			if let Some(x) = self.text { html_element.set_inner_text(x) };
			if let Some(x) = self.attributes {
				for [k, v] in x {
					html_element.set_attribute(k, v).unwrap();
				}
			};
			for child in &self.children {
				html_element.append_child(match child {
					BuilderChild::Owned(x) => x.element(),
					BuilderChild::Ref(x) => x.element(),
				}).expect("Can't append child");
			}
			let cmp = crate::BasicElement {
				element,
				children: self.children.into_iter().filter_map(|c| if let BuilderChild::Owned(x) = c { Some(x) } else { None }).collect::<Vec<_>>(),
				event_handlers: crate::EventHandlers::default(),
			};
			if let Some(x) = self.class { cmp.set_class(x); };
			if let Some(x) = self.style { cmp.set_style(x); };
			cmp
		}

		pub fn build_raw_svg<T: AsRef<web_sys::Element> + AsRef<web_sys::SvgElement> + 'static>(self, element: T) -> crate::BasicElement<T> {
			let svg_element: &web_sys::SvgElement = element.as_ref();
			if let Some(x) = self.attributes {
				for [k, v] in x {
					svg_element.set_attribute(k, v).unwrap();
				}
			};
			for child in &self.children {
				svg_element.append_child(match child {
					BuilderChild::Owned(x) => x.element(),
					BuilderChild::Ref(x) => x.element(),
				}).expect("Can't append child");
			}
			let cmp = crate::BasicElement {
				element,
				children: self.children.into_iter().filter_map(|c| if let BuilderChild::Owned(x) = c { Some(x) } else { None }).collect::<Vec<_>>(),
				event_handlers: crate::EventHandlers::default(),
			};
			if let Some(x) = self.class { cmp.set_class(x); };
			if let Some(x) = self.style { cmp.set_style(x); };
			cmp
		}
	}
}
