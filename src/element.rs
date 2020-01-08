use crate::web_str;
use std::hash::{Hash, Hasher};

pub trait Element: Drop {
	fn element(&self) -> &web_sys::Element;

	fn class() -> String
	where
		Self: Sized + 'static,
	{
		std::any::TypeId::of::<Self>().to_class_string("t")
	}

	fn append(&self, child: &dyn Element) { self.element().append_child(child.element()).expect("Can't append child"); }

	fn set_class(&self, style: &css::Style) -> &Self
	where
		Self: Sized + 'static,
	{
		super::CONTEXT.with(move |ctx| {
			let element = self.element();
			let element_class = ctx.style_storage.fetch(element, style);
			element.set_attribute(web_str::class(), &format!("{} {}", Self::class(), element_class)).unwrap();
			// TODO:
			// ctx.classes.borrow_mut().insert(0, element_class);
			self
		})
	}

	fn add_class(&self, style: &css::Style) -> &Self
	where
		Self: Sized + 'static,
	{
		super::CONTEXT.with(move |ctx| {
			let element = self.element();
			let element_class = ctx.style_storage.fetch(element, style);
			let existing_class = element.get_attribute(web_str::class()).unwrap_or_else(String::new);
			element.set_attribute(web_str::class(), &format!("{} {}", existing_class, element_class)).unwrap();
			// TODO:
			// ctx.classes.borrow_mut().insert(0, element_class);
			self
		})
	}
}

impl AsRef<web_sys::Element> for dyn Element {
	fn as_ref(&self) -> &web_sys::Element { self.element() }
}

#[extend::ext]
impl<T: Hash> T {
	fn to_class_string(&self, prefix: &str) -> String {
		let mut hasher = std::collections::hash_map::DefaultHasher::new();
		self.hash(&mut hasher);
		let id = hasher.finish();
		format!("{}{}", prefix, id)
	}
}
