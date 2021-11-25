# Getting Started

Here's a basic counter component:

```rust,noplaypen
use hobo::{prelude::*, components as cmp};

// <div class="s-f4d1763947b5e1ff">
//   <div>Counter value is: 0</div>
//   <button>increment</button>
//   <button>decrement</button>
// </div>

fn counter() -> impl hobo::Element {
	let counter_value = Mutable::new(0_i32);

	cmp::div()
		.class((
			css::display!(flex), // enum-like properties can also be set like `css::Display::Flex`
			css::size!(200 px), // `size` is a shortcut to set `width` and `height` simultaneously
			css::background_color!(rgb 0xAA_00_00), // #AA0000FF or #AA0000 or #A00 in normal css
		))
		.child(cmp::div()
			.text_signal(counter_value.signal().map(|value| format!("Counter value is: {}", value)))
		)
		.component(counter_value)
		.with(move |&counter_div| counter_div
			.child(cmp::button()
				.text("increment")
				.on_click(move |_| *counter_div.get_cmp::<Mutable<i32>>().lock_mut() += 1)
			)
			.add_child(cmp::button() // same as .child but non-chaining
				.text("decrement")
				.on_click(move |_| *counter_div.get_cmp::<Mutable<i32>>().lock_mut() -= 1)
			)
		)
}
```
