use crate::{CONTEXT, prelude::*};
use std::hash::{Hash, Hasher};
use std::cell::RefCell;
use std::rc::Rc;
use std::borrow::Cow;

pub trait Element {
	// should probably be subsumed by BasicElement, which would also probably give me more control over ssr
	fn element(&self) -> Cow<'_, web_sys::Element>;

	fn to_class_string() -> String where
		Self: Sized + 'static,
	{
		std::any::TypeId::of::<Self>().to_class_string("t")
	}

	fn append(&self, child: &dyn Element) { self.element().append_child(&child.element()).expect("Can't append child"); }

	fn class<'a>(self, style: impl Into<Cow<'a, css::AtRules>>) -> Self where
		Self: Sized + 'static,
	{
		self.set_class(style);
		self
	}

	fn style<'a>(self, style: impl Into<Cow<'a, [css::Property]>>) -> Self where
		Self: Sized + 'static,
	{
		self.set_style(style);
		self
	}

	fn attr<'a>(self, key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> Self where
		Self: Sized + 'static
	{
		self.element().set_attribute(&key.into(), &value.into()).expect("can't set attribute");
		self
	}

	fn set_class<'a>(&self, style: impl Into<Cow<'a, css::AtRules>>) -> &Self where
		Self: Sized + 'static,
	{
		CONTEXT.with(move |ctx| {
			let element = self.element();
			let element_class = ctx.style_storage.fetch(&element, style);
			element.set_attribute(web_str::class(), &format!("{} {}", Self::to_class_string(), element_class)).unwrap();
			self
		})
	}

	fn set_style<'a>(&self, style: impl Into<Cow<'a, [css::Property]>>) where
		Self: Sized,
	{
		self.element().set_style(style.into());
	}

	fn remove_style(&self) {
		self.element().remove_style();
	}

	fn add_class<'a>(&self, style: impl Into<Cow<'a, css::AtRules>>) -> &Self where
		Self: Sized + 'static,
	{
		CONTEXT.with(move |ctx| {
			let element = self.element();
			let element_class = ctx.style_storage.fetch(&element, style);
			let existing_class = element.get_attribute(web_str::class()).unwrap_or_else(Self::to_class_string);
			element.set_attribute(web_str::class(), &format!("{} {}", existing_class, element_class)).unwrap();
			self
		})
	}
}

impl Element for RefCell<dyn Element> {
	fn element(&self) -> Cow<'_, web_sys::Element> {
		Cow::Owned(self.borrow().element().into_owned())
	}
}

impl Element for Box<dyn Element> {
	fn element(&self) -> Cow<'_, web_sys::Element> {
		self.as_ref().element()
	}
}

impl<T: Element> Element for Rc<RefCell<T>> {
	fn element(&self) -> Cow<'_, web_sys::Element> {
		Cow::Owned(self.borrow().element().into_owned())
	}
}

#[extend::ext(pub, name = HashToClassString)]
impl<T: Hash> T {
	fn to_class_string(&self, prefix: &str) -> String {
		let mut hasher = std::collections::hash_map::DefaultHasher::new();
		self.hash(&mut hasher);
		let id = hasher.finish();
		format!("{}{}", prefix, id)
	}
}

#[doc(hidden)]
#[extend::ext(pub, name = RawSetClass)]
impl web_sys::Element {
	fn set_class<'a>(&self, style: impl Into<Cow<'a, css::AtRules>>) {
		CONTEXT.with(move |ctx| {
			let element_class = ctx.style_storage.fetch(&self, style);
			self.set_attribute(web_str::class(), &element_class).unwrap();
		})
	}

	fn add_class<'a>(&self, style: impl Into<Cow<'a, css::AtRules>>) {
		CONTEXT.with(move |ctx| {
			let element_class = ctx.style_storage.fetch(&self, style);
			let existing_class = self.get_attribute(web_str::class()).unwrap_or_else(String::new);
			self.set_attribute(web_str::class(), &format!("{} {}", existing_class, element_class)).unwrap();
		})
	}

	fn set_style<'a>(&self, style: impl Into<Cow<'a, [css::Property]>>) {
		let _ = self.set_attribute(web_str::style(), &style.into().iter().map(std::string::ToString::to_string).collect::<String>());
	}

	fn remove_style(&self) {
		let _ = self.remove_attribute(web_str::style());
	}
}
