# Building the DOM

Assembling components is usually done via function chaining, but every function has a non-chained variant for use in loops or in case ownership gets tricky.

Here's an example of a somewhat involved component:

```rust
#[derive(hobo::Element, hobo::Container, hobo::EventTarget)]
pub struct Input {
	element: cmp::Div,
	pub input: cmp::Input,
}

impl Input {
	pub fn new(caption_text: &str, svg: Option<cmp::Svg>) -> Self {
		let input = cmp::input()
			.attr(web_str::r#type(), web_str::text())
			.class(css::class! {
				// some style
			});

		let caption_text = cmp::div()
			.text(caption_text)
			.class(css::class! {
				// some style
			});

		let mut element = cmp::div()
			.child_ref(&input)
			.child(caption_text);

		if let Some(svg) = svg {
			element.add_child(
				svg.class(css::class! {
					// some style
				})
			);
		}

		Self { element, input }
			.class(css::style! {
				// some style
			})
	}
}
```

## Difference between `.child()`/`.add_child()` and `.child_ref()`/`.add_child_ref()`

`.child()` takes a component by value, while `.child_ref()` by reference. Taking a component by value ties that component's lifespan to the lifespan of its container, in other words when the container will be dropped - the child will as well. However, sometimes you need to mount a component first but later capture it in an event handler, expose it to parent or replace it later. In those cases, ownership of the component must be passed elsewhere.

## Why is `.class()` called after `Self { element, input }` and not while `element` is being constructed?

Class assignment in `hobo` is double duty - apart from applying the style, it also assigns class that's generated from the component's type so it can later be selected with a `.[T]` selector. If class where assigned at the `element`'s construction site - it would get the class of type for `cmp::Div` (alias for `hobo::BasicElement<web_sys::HtmlDivElement>`) rather than `Input`.
