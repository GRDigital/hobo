use crate::{prelude::*, Element};
use std::{cell::RefCell, rc::Rc};

pub trait Replaceable<T>: Element {
	fn replace_element(&self, element: T) where Self: Sized;
}

impl<T: Element> Replaceable<T> for RefCell<T> {
	fn replace_element(&self, element: T) {
		let mut me = self.borrow_mut();
		me.element().insert_adjacent_element(web_str::afterend(), &element.element()).unwrap();
		*me = element;
	}
}

impl<T: Element, R: Replaceable<T>> Replaceable<T> for Rc<R> {
	fn replace_element(&self, element: T) {
		R::replace_element(&self, element);
	}
}
