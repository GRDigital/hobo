use crate::Element;
use std::{cell::RefCell, rc::Rc};

/// Trait for all hobo elements that can have children
pub trait Container: Element {
	fn children(&self) -> &Vec<Box<dyn Element>>;
	fn children_mut(&mut self) -> &mut Vec<Box<dyn Element>>;

	fn attach_child_box(&mut self, child: Box<dyn Element + 'static>) {
		self.element().append_child(&child.element()).expect("can't append child");
		self.children_mut().push(child);
	}

	fn add_child(&mut self, child: impl crate::Element + 'static) {
		self.attach_child_box(Box::new(child));
	}

	fn add_child_ref(&self, child: &(impl crate::Element + 'static)) {
		self.element().append_child(&child.element()).expect("can't append child");
	}

	fn child(mut self, child: impl crate::Element + 'static) -> Self where Self: Sized {
		self.add_child(child);
		self
	}

	fn child_ref(self, child: &(impl crate::Element + 'static)) -> Self where Self: Sized {
		self.add_child_ref(child);
		self
	}

	fn with_children<Item>(mut self, children: impl IntoIterator<Item = Item>) -> Self where
		Self: Sized,
		Item: crate::Element + 'static,
	{
		children.into_iter().for_each(|child| self.add_child(child));
		self
	}

	fn with_children_refs<'a, Item>(self, children: impl IntoIterator<Item = &'a Item>) -> Self where
		Self: Sized,
		Item: crate::Element + 'static,
	{
		children.into_iter().for_each(|child| self.add_child_ref(child));
		self
	}
}

impl<T: Container> Container for Rc<RefCell<T>> {
	fn children(&self) -> &Vec<Box<dyn Element>> { unsafe { self.try_borrow_unguarded() }.expect("rc is mutably borrowed").children() }

	fn children_mut(&mut self) -> &mut Vec<Box<dyn Element>> { Rc::get_mut(self).expect("rc is mutably borrowed").get_mut().children_mut() }
}
