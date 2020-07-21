# Getting Started

Here's a basic counter component:

```rust
use hobo::{prelude::*, enclose as e, cmp};

#[derive(hobo::Element, hobo::Container, hobo::EventTarget, hobo::RawElement)]
pub struct Counter {
	element: cmp::Div,
	count: i32,
}

// <div>
//   <button>PLUS</button>
//   <div>0</div>
// </div>
impl Counter {
	#[hobo::trick]
	fn new() -> Self {
		// we need this to be able to clone raw element reference into on_click handlers
		let text = cmp::div().text("0");

		Self { element: cmp::div(), count: 0 }
			.class(css::class!(
				css::width!(128 px),
				css::height!(128 px),
				// #AA0000FF or #AA0000 of #A00 in css
				// hobo requires full rgba though
				css::background_color!(0xAA_00_00_FF),
			))
			.child(cmp::button()
				.text("PLUS")
				.on_click_mut(&this, e!((*text) move |this, _| {
					this.count += 1;
					text.set_inner_text(&this.count.to_string());
				}))
			)
			.child(text)
	}
}
```
