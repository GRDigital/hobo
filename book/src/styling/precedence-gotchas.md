# Precedence gotchas due to class reuse

Due to the order of classes creation, a line that you write later might have higher precedence than a line that you write earlier, perhaps accidentally.

```rust,noplaypen
use hobo::create as e;

e::div()
	.class(css::display!(none));

// -- snip --

let element = e::div();
element
	.class(css::display!(flex))
	.on_click(move |_| {
		// this won't actually change anything because `css::display!(none)`
		// class is generated first
		// the classname is reused here, then `css::display!(flex)` class is
		// generated later and has higher precedence
		element.class(css::display!(none));
	})
```

The workaround is to either use `.style` (that has the highest precedence) or to change the style to prevent class reuse. Of course, you can increase specificity by repeating the same class selector

```rust,noplaypen
let element = e::div();
element
	.class(css::display!(flex))
	.on_click(move |_| {
		element.class(css::style!(
			.&.& {
				css::display!(none),
			}
		));
	})
```
