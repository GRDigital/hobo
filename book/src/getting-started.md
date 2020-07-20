# Getting Started

Here's a basic counter component:

```rust
#[derive(hobo::Element, hobo::Container, hobo::EventTarget, hobo::RawElement)]
pub struct Counter {
	element: cmp::Div,
	count: i32,
}

impl Counter {
	#[hobo::trick]
	fn new() -> Self {
		// we need this to be able to clone raw element reference into on_click handlers
		let text = cmp::div().text("0");

		Self {
			count: 0,
			// <div>
			//   <button>MINUS</button>
			//   <button>PLUS</button>
			//   <div>0</div>
			// </div>
			element: cmp::div()
				.child(cmp::button()
					.text("MINUS")
					// `*text` in `e!()` means just clone a reference to a html element in it
					// `this` is a variable injected by `#[hobo::trick]` and in the closure it's just `&mut Self`
					.with_on_click_mut(&this, e!((*text) move |this, _| {
						this.count -= 1;
						text.set_inner_text(&this.count.to_string());
					}))
				)
				.child(cmp::button()
					.text("PLUS")
					.with_on_click_mut(&this, e!((*text) move |this, _| {
						this.count += 1;
						text.set_inner_text(&this.count.to_string());
					}))
				)
				.child(text),
		}
			.class(css::class!(
				css::width!(128 px),
				css::height!(128 px),
				css::background_color!(0xAA_00_00_FF),
			))
	}
}
```
