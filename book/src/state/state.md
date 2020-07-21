# State

Sometimes it's convenient to do actions in response to a state change within a component. HTMl events like clicks don't necessarily correspond to a state change, so a higher level abstraction is required.

```rust
#[derive(hobo::Element, hobo::Container, hobo::EventTarget, hobo::RawElement)]
pub struct Counter {
	element: cmp::Div,
	count: State<i32>,
}

impl Counter {
	fn new() -> Self {
		let count = State::new(0);
		let text = cmp::div().text("0");

		// the return type is a SubscriptionKey
		// so we can unsub or resub throughout the component lifespan as we need
		count.subscribe_key(e!((%state count, *text) move || {
			text.set_inner_text(&count.view().to_string());
		}));

		Self {
			element: cmp::div()
				.child(cmp::button()
					.text("MINUS")
					.with_on_click(e!((%state count) move |_| {
						*count.update() -= 1;
					}))
				)
				.child(cmp::button()
					.text("PLUS")
					.with_on_click(e!((%state count) move |_| {
						*count.update() += 1;
					}))
				)
				.child(text),
			count,
		}
	}
}
```
