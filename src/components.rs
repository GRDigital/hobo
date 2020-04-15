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
	SvgRoot => SvgsvgElement,
	Main => HtmlElement,
	Button => HtmlButtonElement,
	Label => HtmlLabelElement,
}

pub fn build<'a>() -> Builder<'a> {
	Builder::default()
}

pub mod builder {
	use crate::prelude::*;
	use std::borrow::{BorrowMut, Cow};

	pub enum BuilderChild<'a> {
		Owned(Box<dyn crate::Element>),
		Ref(&'a dyn crate::Element),
	}

	#[derive(Default)]
	pub struct Builder<'a> {
		pub text: Option<Cow<'a, str>>,
		pub attributes: Option<Vec<[Cow<'a, str>; 2]>>,
		pub class: Option<Cow<'a, crate::css::Style>>,
		pub style: Option<Cow<'a, [crate::css::Property]>>,
		pub children: Vec<BuilderChild<'a>>,
	}

	impl<'a> Builder<'a> {
		pub fn text(mut self, x: impl Into<Cow<'a, str>>) -> Self { self.text = Some(x.into()); self }
		pub fn class(mut self, x: impl Into<Cow<'a, crate::css::Style>>) -> Self { self.class = Some(x.into()); self }
		pub fn style(mut self, x: impl Into<Cow<'a, [crate::css::Property]>>) -> Self { self.style = Some(x.into()); self }
		pub fn child(mut self, child: impl crate::Element + 'static) -> Self { self.children.push(BuilderChild::Owned(Box::new(child))); self }
		pub fn child_ref(mut self, child: &'a (impl crate::Element + 'static)) -> Self { self.children.push(BuilderChild::Ref(child)); self }
		pub fn children<E: crate::Element + 'static, I: IntoIterator<Item = E>>(mut self, children: I) -> Self {
			for child in children.into_iter() {
				self.children.push(BuilderChild::Owned(Box::new(child)));
			}
			self
		}
		pub fn children_ref<E: crate::Element + 'static, I: IntoIterator<Item = &'a E>>(mut self, children: I) -> Self {
			for child in children.into_iter() {
				self.children.push(BuilderChild::Ref(child));
			}
			self
		}

		pub fn attr(mut self, key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> Self {
			let mut v = self.attributes.unwrap_or_else(Vec::new);
			v.push([key.into(), value.into()]);
			self.attributes = Some(v);
			self
		}

		pub fn build<T: AsRef<web_sys::Element> + 'static, E: BorrowMut<crate::BasicElement<T>>>(self, mut component: E) -> E {
			{
				let component = component.borrow_mut();
				let element: &web_sys::Element = component.element.as_ref();
				if let Some(x) = self.text {
					element.unchecked_ref::<web_sys::HtmlElement>().set_inner_text(&x)
				};
				if let Some(x) = self.attributes {
					for [k, v] in x {
						element.set_attribute(&k, &v).expect("can't set attributes");
					}
				};
				for child in self.children.into_iter() {
					match child {
						BuilderChild::Owned(x) => {
							component.append(&*x);
							component.children.push(x);
						},
						BuilderChild::Ref(x) => component.append(x),
					}
				}
				if let Some(x) = self.class { component.add_class(x); };
				if let Some(x) = self.style { component.set_style(x); };
			}
			component
		}

		pub fn build_raw<T: AsRef<web_sys::Element> + 'static>(self, element: T) -> crate::BasicElement<T> {
			{
				let element: &web_sys::Element = element.as_ref();
				if let Some(x) = self.text {
					element.unchecked_ref::<web_sys::HtmlElement>().set_inner_text(&x)
				};
				if let Some(x) = self.attributes {
					for [k, v] in x {
						element.set_attribute(&k, &v).expect("can't set attributes");
					}
				};
				for child in &self.children {
					element.append_child(match child {
						BuilderChild::Owned(x) => x.element(),
						BuilderChild::Ref(x) => x.element(),
					}.as_ref()).expect("Can't append child");
				}
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
