# Events

There is a simple way to fire and respond to events.

```rust,noplaypen
struct MyEvent(u64);

// -- snip --

fn make_foo() -> impl hobo::Element {
	e::div()
		// etc children and styles
		.component(hobo::events::on(move |&MyEvent(x)| {
			// do something with x
		}))
}

// -- snip --

hobo::events::fire(&MyEvent(123));
```

The subscribers are notified based on event type, so it's better to create new types for different events rather than fire an event with a string or an enum.
