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
					.on_click(e!((%state count) move |_| {
						*count.update() -= 1;
					}))
				)
				.child(cmp::button()
					.text("PLUS")
					.on_click(e!((%state count) move |_| {
						*count.update() += 1;
					}))
				)
				.child(text),
			count,
		}
	}
}
```

## Global state

Some operations are cross-cutting through the whole application, like caching DB responses and populating data which may be used across many components. One way to handle this is some kind of global state, this is what Redux and the like achieve.

In `hobo` we can use a static `StateSlice` to achieve a similar result. 

```rust
use once_cell::sync::Lazy;

#[derive(Default)]
pub struct GlobalState {
	pub foo: HashMap<String, i32>,
	pub bar: u32,
}

pub static STATE: Lazy<StateSlice<GlobalState>> = Lazy::new(GlobalState::default);
	
// ...

#[derive(hobo::Element)]
struct FooCmp {
	element: cmp::Div,
	// ...
	subscription: Subscription,
}

impl FooCmp {
	fn new() -> Self {
		let subscription = STATE.subscribe(e!((...) move || {
			// ...
		}));

		// ...

		Self { element, subscription }
	}
}
```

Later, when `FooCmp` is dropped, `subscription` will be unsubscribed as well. This is the difference between `.subscribe()` and `.and subscribe_key()` - the latter only returns a key which has to be used manually, while the former returns both the key and a reference to `StateSlice` it was taken from so the unsubscribing happens on drop automatically.    
`GlobalState` can have nested `StateSlice`s for a more fine-grained control. Similarly, you can have more than one `GlobalState` - there's no hidden shared state to worry about.

> **Note:** updating `State` or `StateSlice` from a subscription function will re-trigger all subscriptions once the update is finished, so be mindful of infinite update loops.
