use hobo::{cmp, enclose as e, prelude::*};
use std::{rc::Rc, cell::RefCell};

#[derive(hobo::Element, hobo::Container, hobo::EventTarget, hobo::RawElement)]
struct TestDerives {
	element: cmp::Div,
	flag: bool,
}

impl TestDerives {
	#[hobo::trick]
	fn new() -> Self {
		let element = cmp::div();
		Self { element, flag: false }.on_click_mut(&this, move |this, _| {
			this.flag = true;
		})
	}
}

#[derive(hobo::Element, hobo::Container, hobo::EventTarget)]
enum SomeEnum {
	Element(SomeElement),
	Div(cmp::Div),
}

#[derive(hobo::Element, hobo::Container, hobo::EventTarget, hobo::Replaceable)]
struct TestReplaceable {
	element: Rc<RefCell<cmp::Div>>,
}
