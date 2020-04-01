use crate::{web_str, RawSetClass};
use std::hash::{Hash, Hasher};
use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Element {
	// should probably be subsumed by BasicElement, which would also probably give me more control over ssr
	fn element(&self) -> Cow<'_, web_sys::Element>;

	fn class() -> String
	where
		Self: Sized + 'static,
	{
		std::any::TypeId::of::<Self>().to_class_string("t")
	}

	fn append(&self, child: &dyn Element) { self.element().append_child(&child.element()).expect("Can't append child"); }

	fn set_class<'a>(&self, style: impl Into<Cow<'a, css::Style>>) -> &Self
	where
		Self: Sized + 'static,
	{
		super::CONTEXT.with(move |ctx| {
			let element = self.element();
			let element_class = ctx.style_storage.fetch(&element, style);
			element.set_attribute(web_str::class(), &format!("{} {}", Self::class(), element_class)).unwrap();
			self
		})
	}

	fn set_style<'a>(&self, style: impl Into<Cow<'a, [css::Property]>>)
	where
		Self: Sized,
	{
		self.element().set_style(style.into());
	}

	fn remove_style(&self) {
		self.element().remove_style();
	}

	fn add_class<'a>(&self, style: impl Into<Cow<'a, css::Style>>) -> &Self
	where
		Self: Sized+ 'static,
	{
		super::CONTEXT.with(move |ctx| {
			let element = self.element();
			let element_class = ctx.style_storage.fetch(&element, style);
			let existing_class = element.get_attribute(web_str::class()).unwrap_or_else(Self::class);
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

// impl<T: Element> Element for Rc<RefCell<T>> {
//     fn element(&self) -> Cow<'_, web_sys::Element> {
//         Cow::Owned(self.borrow().element().into_owned())
//     }
// }

// impl AsRef<web_sys::Element> for dyn Element {
//     fn as_ref(&self) -> &web_sys::Element { self.element() }
// }

#[extend::ext(pub, name = HashToClassString)]
impl<T: Hash> T {
	fn to_class_string(&self, prefix: &str) -> String {
		let mut hasher = std::collections::hash_map::DefaultHasher::new();
		self.hash(&mut hasher);
		let id = hasher.finish();
		format!("{}{}", prefix, id)
	}
}
