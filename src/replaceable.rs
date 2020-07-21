use crate::{prelude::*, Element};
use std::{cell::RefCell, rc::Rc};

/// Trait for all hobo components that can be replaced with a different instance of itself
pub trait Replaceable<T>: Element {
	fn replace_element(&self, element: T);
}

impl<T: Element> Replaceable<T> for RefCell<T> {
	fn replace_element(&self, element: T) {
		let mut me = self.borrow_mut();
		me.element().insert_adjacent_element(web_str::afterend(), &element.element()).expect("can't insert adjacent element");
		*me = element;
	}
}

impl<T: Element, R: Replaceable<T>> Replaceable<T> for Rc<R> {
	fn replace_element(&self, element: T) { R::replace_element(&self, element); }
}
