use crate::{prelude::*, Element, Replaceable};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub struct Slot(pub Rc<RefCell<Box<dyn Element>>>);

impl Slot {
	pub fn new(element: impl Element + 'static) -> Self { Self(Rc::new(RefCell::new(Box::new(element)))) }
}

impl Element for Slot {
	fn element(&self) -> std::borrow::Cow<'_, web_sys::Element> { std::borrow::Cow::Owned(self.0.borrow().element().into_owned()) }
}

impl<T: Element + 'static> Replaceable<T> for Slot {
	fn replace_element(&self, element: T) {
		let mut me = self.0.borrow_mut();
		me.element().insert_adjacent_element(web_str::afterend(), &element.element()).expect("can't insert adjacent element");
		*me = Box::new(element);
	}
}
