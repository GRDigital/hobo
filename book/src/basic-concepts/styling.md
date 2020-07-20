# Styling facilities

Most hobo components will be styled with either `.class()` or `.style()` functions, where either `css::class!()`, `css::style!()` or `css::properties!()` macros. The naming is confusing somewhat, but the distinction is important.

* `.style()` and `.set_style()` use the `style` attribute, which can only take a bunch of properties without any selectors, so `css::properties!()` macro is used.
* `.class()`, `.set_class()` and `.add_class()` use the `class` attribute:
	* `css::style!()` uses a css-like `{ <selector> { <properties> } <selector> { <properties> } }` syntax
	* `css::class!()` is `css::style!(.& { <properties> })` or in other words - it's a bunch of properties applied just to the element being styled, similar to what can go in a `style` attribute, just using a class to refer to it.

For example, here's a style:

```rust
cmp::div()
	.class(css::style!(
		.& {
			css::height!(393 px),
			css::Display::Flex,
			css::AlignItems::Center,
			css::Position::Relative,
		}

		.& > svg {
			css::width!(12 px),
			css::height!(100%),
			css::Cursor::Pointer,
			css::flex_shrink!(0),
			css::UserSelect::None,
		}

		.& > :not(:nth_child(1)) {
			css::z_index!(200),
		}

		.& > div:not(:nth_child(1)) {
			css::width!(17.5%),
			css::height!(100%),
			css::Display::Flex,
			css::AlignItems::Center,
		}

		.&.& > :nth_child(5) {
			css::width!(30%),
		}

		.& > *:nth_child(3) > img,
		.& > *:nth_child(4) > img,
		.& > svg:last_child {
			css::Transform::Some(vec![css::TransformFunction::ScaleX((-1.).into())])
		}

		.& >> img {
			css::height!(100%),
		}
	))
```
