# Styling facilities

Most **Elements** will be styled with either `.class()` or `.style()` functions, where either `css::style!()` or a property tuple will be used.

* `.style()` and `.set_style()` use the `style` attribute, which can only take a bunch of properties without any selectors, so a property tuple is used.
* `.class()`, `.set_class()` and `tagged` or `typed` variants use the `class` attribute:

For example, here's a style:

```rust,noplaypen
hobo::components::div()
	.class(css::style!(
		.& {
			css::height!(393 px),
			css::Display::Flex, // can also be `css::display!(flex)`
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

		.& > :not(:nth_child(0, 1)) { // nth_child will convert to An+B syntax
			css::z_index!(200),
		}

		.& > div:not(:nth_child(0, 1)) {
			css::width!(17.5%),
			css::height!(100%),
			css::Display::Flex,
			css::AlignItems::Center,
		}

		// doubling up on the class name increases specificity
		.&.& > :nth_child(0, 5) { 
			css::width!(30%),
		}

		.& > *:nth_child(0, 3) > img,
		.& > *:nth_child(0, 4) > img,
		.& > svg:last_child {
			css::TransformFunction::TranslateX(css::unit!(50%)),
		}

		.& >> img { // this is same as `.& img` selector in css
			css::height!(100%),
		}
	))
```

> **Chaining vs non-chaining syntax:** `.style()` is the chaining syntax, `.set_style()` is the non-chaining alternative. Similarly, `.class()` and `.set_class()`. More about chaining vs non-chaining syntax in [Building the DOM](../building-the-dom.md#chaining-vs-non-chaining-syntax).
