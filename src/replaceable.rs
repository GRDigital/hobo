use crate::{prelude::*, Element};
use std::cell::RefCell;
use std::rc::Rc;

pub trait Replaceable<T>: Element {
	fn replace_element(&self, element: T) where Self: Sized;
}

impl<T: Element> Replaceable<T> for Rc<RefCell<T>> {
	fn replace_element(&self, element: T) {
		let mut me = self.borrow_mut();
		me.element().insert_adjacent_element(web_str::afterend(), &element.element()).unwrap();
		*me = element;
	}
}
