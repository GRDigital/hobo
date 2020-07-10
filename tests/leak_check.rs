#![feature(try_blocks, proc_macro_hygiene, async_closure, new_uninit, maybe_uninit_extra, get_mut_unchecked)]
#![recursion_limit="1024"]

use stats_alloc::{StatsAlloc, Stats, Region};
use std::alloc::System;

#[global_allocator]
pub static ALLOC: &StatsAlloc<System> = &stats_alloc::INSTRUMENTED_SYSTEM;

use wasm_bindgen_test::*;
use hobo::{enclose as e, prelude::*, cmp};

wasm_bindgen_test_configure!(run_in_browser);

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
		Self { element, flag: false }
			.with_on_click_mut(&this, move |this, _| {
				this.flag = true;
			})
	}
}

#[wasm_bindgen_test]
fn trick_leak() {
	console_log::init().unwrap();
	console_error_panic_hook::set_once();

	{
		let region = Region::new(ALLOC);
		for _ in 0 .. 100000 {
			let element = SomeElement::trick_new();
			let raw_element = element.borrow().raw_element().clone();
			raw_element.click();
			assert!(element.borrow().flag);
		}
		let change = region.change();
		assert_eq!(change.bytes_allocated, change.bytes_deallocated);
	}
}

// #[wasm_bindgen_test]
// fn fail() {
//     assert_eq!(1, 2);
// }
