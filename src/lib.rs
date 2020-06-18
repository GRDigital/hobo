#![feature(proc_macro_hygiene, trait_alias)]

mod basic_element;
mod element;
mod enclose;
pub mod prelude;
mod svg_element;
pub mod web_str;
// pub mod components;
pub mod create;
pub mod svg_create;
pub mod events;
pub mod state_slice;

pub use basic_element::BasicElement;
pub use css;
pub use element::Element;
pub use hobo_derive::*;
pub use paste;
use std::{
	rc::Rc,
	cell::RefCell,
	collections::HashMap,
	hash::{Hash, Hasher},
	borrow::Cow,
};
pub use web_sys;
pub use create::components as cmp;
pub use create::*;
pub use svg_create::*;
pub use events::*;

pub type Color = (u8, u8, u8, u8);

fn dom() -> web_sys::Document {
	web_sys::window().expect("no window").document().expect("no document")
}

#[derive(Default)]
pub struct Context {
	style_storage: StyleStorage,
}

thread_local! {
	static CONTEXT: Context = Default::default();
}

#[derive(Default)]
struct StyleStorage {
	map: RefCell<HashMap<css::AtRules, u64>>,
}

// TODO: right now if the same style is reused in multiple windows - won't work, need to track style insertion per window
// it checks if the style is already inserted as css into <style>
// if yes, just returns the class name
// if no, inserts it into <style> and then returns the class name
impl StyleStorage {
	fn fetch<'a>(&self, element: &web_sys::Element, at_rules: impl Into<Cow<'a, css::AtRules>>) -> String {
		let at_rules = at_rules.into();
		if let Some(id) = self.map.borrow().get(&at_rules) {
			return format!("s{}", id);
		}
		let mut hasher = std::collections::hash_map::DefaultHasher::new();
		at_rules.hash(&mut hasher);
		let id = hasher.finish();
		self.map.borrow_mut().insert(at_rules.clone().into_owned(), id);
		let class = format!("s{}", id);
		let mut at_rules = at_rules.into_owned();
		for rule in at_rules.0.iter_mut() {
			for style_rule in rule.style.0.iter_mut() {
				for selector_component in (style_rule.0).0.iter_mut() {
					if *selector_component == css::selector::SelectorComponent::ClassPlaceholder {
						*selector_component = css::selector::SelectorComponent::Class(class.clone());
					}
				}
			}
		}
		let dom = element.owner_document().expect("element not attached to a dom");
		let head = dom.head().expect("dom has no head");
		let style_element = if let Some(x) = head.get_elements_by_tag_name("style").get_with_index(0) {
			x
		} else {
			let element = dom.create_element(web_str::style()).unwrap();
			head.append_child(&element).unwrap();
			element
		};
		style_element.append_with_str_1(&at_rules.to_string()).unwrap();
		class
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

pub trait Container: Element {
	fn children(&self) -> &Vec<Box<dyn Element>>;
	fn children_mut(&mut self) -> &mut Vec<Box<dyn Element>>;

	fn attach_child(&mut self, child: impl Element + 'static) where Self: Sized {
		self.attach_child_box(Box::new(child))
	}

	fn attach_child_box(&mut self, child: Box<dyn Element + 'static>) {
		self.append(&child);
		self.children_mut().push(child);
	}

	fn child(mut self, child: impl crate::Element + 'static) -> Self where Self: Sized {
		self.attach_child(child);
		self
	}

	fn child_ref(self, child: &(impl crate::Element + 'static)) -> Self where Self: Sized {
		self.element().append_child(&child.element()).expect("can't append child");
		self
	}
}

pub trait Basic: Element + Container + EventTarget { }
impl<T: Element + Container + EventTarget> Basic for T {}

pub trait Replaceable<T>: Basic {
	fn replace_element(&self, element: T) where Self: Sized;
}

impl Element for Box<dyn Basic> {
	fn element(&self) -> std::borrow::Cow<'_, web_sys::Element> {
		self.as_ref().element()
	}
}

impl Container for Box<dyn Basic> {
	fn children(&self) -> &Vec<Box<dyn Element>> { self.as_ref().children() }
	fn children_mut(&mut self) -> &mut Vec<Box<dyn Element>> { self.as_mut().children_mut() }
}

#[derive(Clone)]
pub struct Slot(pub Rc<RefCell<Box<dyn Basic>>>);

impl Slot {
	pub fn new(element: impl Basic + 'static) -> Self {
		Self(Rc::new(RefCell::new(Box::new(element))))
	}
}

impl Container for Slot {
	fn children(&self) -> &Vec<Box<dyn Element>> { unsafe { self.0.try_borrow_unguarded() }.expect("rc is mutably borrowed").children() }
	fn children_mut(&mut self) -> &mut Vec<Box<dyn Element>> { Rc::get_mut(&mut self.0).expect("rc is mutably borrowed").get_mut().children_mut() }
}

impl<T: Element> Element for Rc<RefCell<T>> {
	fn element(&self) -> std::borrow::Cow<'_, web_sys::Element> {
		std::borrow::Cow::Owned(self.borrow().element().into_owned())
	}
}

impl<T: Container> Container for Rc<RefCell<T>> {
	fn children(&self) -> &Vec<Box<dyn Element>> { unsafe { self.try_borrow_unguarded() }.expect("rc is mutably borrowed").children() }
	fn children_mut(&mut self) -> &mut Vec<Box<dyn Element>> { Rc::get_mut(self).expect("rc is mutably borrowed").get_mut().children_mut() }
}

impl<T: EventTarget> EventTarget for Rc<RefCell<T>> {
	fn event_handlers(&self) -> std::cell::RefMut<Vec<EventHandler>> {
		unsafe { self.try_borrow_unguarded() }.expect("rc is mutably borrowed").event_handlers()
	}
}

impl<T: Basic> Replaceable<T> for Rc<RefCell<T>> {
	fn replace_element(&self, element: T) {
		let mut me = self.borrow_mut();
		me.element().insert_adjacent_element(web_str::afterend(), &element.element()).unwrap();
		*me = element;
	}
}

impl Element for Slot {
	fn element(&self) -> std::borrow::Cow<'_, web_sys::Element> {
		std::borrow::Cow::Owned(self.0.borrow().element().into_owned())
	}
}

impl EventTarget for Slot {
	fn event_handlers(&self) -> std::cell::RefMut<Vec<EventHandler>> {
		unsafe { self.0.try_borrow_unguarded() }.expect("rc is mutably borrowed").event_handlers()
	}
}

impl<T: Basic + 'static> Replaceable<T> for Slot {
	fn replace_element(&self, element: T) {
		let mut me = self.0.borrow_mut();
		me.element().insert_adjacent_element(web_str::afterend(), &element.element()).unwrap();
		*me = Box::new(element);
	}
}
