#[global_allocator]
pub static ALLOC: &stats_alloc::StatsAlloc<std::alloc::System> = &stats_alloc::INSTRUMENTED_SYSTEM;

use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

use hobo::{cmp, enclose as e, prelude::*};

#[derive(hobo::Element, hobo::Container, hobo::EventTarget, hobo::RawElement)]
struct SomeElement {
	element: cmp::Div,
	flag: bool,
}

impl SomeElement {
	fn new() -> Self {
		let element = cmp::div();
		Self { element, flag: false }
	}

	#[hobo::trick]
	fn trick_new() -> Self {
		let element = cmp::div();
		Self { element, flag: false }.with_on_click_mut(&this, move |this, _| {
			this.flag = true;
		})
	}
}

#[wasm_bindgen_test]
fn trick_leak() {
	console_log::init().unwrap();
	console_error_panic_hook::set_once();

	{
		let region = stats_alloc::Region::new(ALLOC);
		for _ in 0..100000 {
			let element = SomeElement::trick_new();
			let raw_element = element.borrow().raw_element().clone();
			raw_element.click();
			assert!(element.borrow().flag);
		}
		let change = region.change();
		assert_eq!(change.bytes_allocated, change.bytes_deallocated);
	}
}
