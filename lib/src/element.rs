use crate::{prelude::*, STYLE_STORAGE};
use std::{
	borrow::Cow,
	cell::RefCell,
	hash::{Hash, Hasher},
	rc::Rc,
	collections::HashMap,
};

/// Core hobo trait for all components
pub trait Element {
	fn element(&self) -> Cow<'_, web_sys::Element>;
	fn classes(&self) -> Rc<RefCell<HashMap<u64, css::Style>>>;

	fn type_class_string() -> String where Self: Sized + 'static {
		std::any::TypeId::of::<Self>().type_class_string("t")
	}

	fn set_attr<'a>(&self, key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) where Self: Sized + 'static {
		self.element().set_attribute(&key.into(), &value.into()).expect("can't set attribute");
	}

	/// Set an attribute with an empty string value
	/// https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes#Boolean_Attributes
	fn set_bool_attr<'a>(&self, key: impl Into<Cow<'a, str>>, value: bool) where Self: Sized + 'static {
		if value {
			self.set_attr(key, "");
		} else {
			self.remove_attr(key);
		}
	}

	fn remove_attr<'a>(&self, key: impl Into<Cow<'a, str>>) where Self: Sized + 'static {
		self.element().remove_attribute(&key.into()).expect("can't set attribute");
	}

	/// Set a tagged class, which means that if a different tag is used - a new style will be applied
	/// alternatively, if a tag that's been used previously is used again - that style is overwritten
	fn set_class_tagged<'a, Tag: Hash + 'static>(&self, tag: Tag, style: impl Into<Cow<'a, css::Style>>) where Self: Sized + 'static {
		let style = style.into().into_owned();
		let element = self.element();

		#[cfg(debug_assertions)]
		element.set_attribute(wasm_bindgen::intern("data-hobo-type"), &std::any::type_name::<Self>()).unwrap();

		// tested and different types with same byte-level representation hash to the same thing (not surprising)
		// i.e. the type is not taken into account when hashing so I have to do it manually
		let tag_hash = {
			let mut hasher = std::collections::hash_map::DefaultHasher::new();
			std::any::TypeId::of::<Tag>().hash(&mut hasher);
			tag.hash(&mut hasher);
			hasher.finish()
		};

		self.classes().borrow_mut().insert(tag_hash, style);

		let mut final_class = Self::type_class_string();
		for x in self.classes().borrow().values().map(|style| STYLE_STORAGE.with(|style_storage| style_storage.fetch(style.clone()))) {
			final_class.push_str(" ");
			final_class.push_str(&x);
		}

		element.set_attribute(web_str::class(), &final_class).expect("can't set attribute");
	}

	/// Set a stlye using the `style` tag rather than creating a class
	fn set_style<'a>(&self, style: impl Into<Cow<'a, [css::Property]>>) where Self: Sized { self.element().set_style(style.into()); }
	/// Remove `style` tag altogether
	fn remove_style(&self) { self.element().remove_style(); }

	/// Set a style with a `__default` tag
	fn set_class<'a>(&self, style: impl Into<Cow<'a, css::Style>>) where Self: Sized + 'static { self.set_class_tagged("__default", style) }
	/// Chaining alternative to `set_class`
	fn class<'a>(self, style: impl Into<Cow<'a, css::Style>>) -> Self where Self: Sized + 'static { self.set_class(style); self }
	/// Chaining alternative to `set_class_tagged`
	fn class_tagged<'a>(self, tag: impl Hash + 'static, style: impl Into<Cow<'a, css::Style>>) -> Self where Self: Sized + 'static { self.set_class_tagged(tag, style); self }
	/// Chaining alternative to `set_style`
	fn style<'a>(self, style: impl Into<Cow<'a, [css::Property]>>) -> Self where Self: Sized + 'static { self.set_style(style); self }
	/// Chaining alternative to `set_attr`
	fn attr<'a>(self, key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> Self where Self: Sized + 'static { self.set_attr(key, value); self }
	/// Chaining alternative to `bool_attr`
	fn bool_attr<'a>(self, key: impl Into<Cow<'a, str>>, value: bool) -> Self where Self: Sized + 'static { self.set_bool_attr(key, value); self }
}

impl Element for RefCell<dyn Element> {
	fn element(&self) -> Cow<'_, web_sys::Element> { Cow::Owned(self.borrow().element().into_owned()) }
	fn classes(&self) -> Rc<RefCell<HashMap<u64, css::Style>>> { self.borrow().classes() }
}

impl Element for Box<dyn Element> {
	fn element(&self) -> Cow<'_, web_sys::Element> { self.as_ref().element() }
	fn classes(&self) -> Rc<RefCell<HashMap<u64, css::Style>>> { self.as_ref().classes() }
}

impl<T: Element> Element for RefCell<T> {
	fn element(&self) -> Cow<'_, web_sys::Element> { Cow::Owned(self.borrow().element().into_owned()) }
	fn classes(&self) -> Rc<RefCell<HashMap<u64, css::Style>>> { self.borrow().classes() }
}

impl<T: Element> Element for Rc<T> {
	fn element(&self) -> Cow<'_, web_sys::Element> { T::element(&self) }
	fn classes(&self) -> Rc<RefCell<HashMap<u64, css::Style>>> { T::classes(&self) }
}

#[extend::ext(pub, name = HashToClassString)]
impl<T: Hash> T {
	fn type_class_string(&self, prefix: &str) -> String {
		let mut hasher = std::collections::hash_map::DefaultHasher::new();
		self.hash(&mut hasher);
		let id = hasher.finish();
		format!("{}{}", prefix, id)
	}
}

#[doc(hidden)]
#[extend::ext(pub, name = RawSetClass)]
impl web_sys::Element {
	fn set_class<'a>(&self, style: impl Into<Cow<'a, css::Style>>) {
		STYLE_STORAGE.with(move |style_storage| {
			let element_class = style_storage.fetch(style.into().into_owned());
			self.set_attribute(web_str::class(), &element_class).expect("can't set attribute");
		})
	}

	fn set_style<'a>(&self, style: impl Into<Cow<'a, [css::Property]>>) {
		let _ = self.set_attribute(web_str::style(), &style.into().iter().map(std::string::ToString::to_string).collect::<String>());
	}

	fn remove_style(&self) { let _ = self.remove_attribute(web_str::style()); }
}
