# SVGs

To conveniently serve SVGs inline without having to rewrite your icons in `hobo`'s syntax, `hobo::BasicElement<web_sys::SvgElement>` can be converted from a `roxmltree::Document` and then cloned with `hobo::BasicElement::clone_html`.

```rust
macro_rules! svg {
	($($name:ident => $address:expr),*$(,)*) => {paste::item!{
		thread_local!{$(
			static [<$name _static>]: hobo::BasicElement<hobo::web_sys::SvgElement> =
				roxmltree::Document::parse(include_str!($address)).unwrap().into();
		)*}

		$(
			#[must_use]
			pub fn $name() -> hobo::BasicElement<hobo::web_sys::SvgElement> {
				[<$name _static>].with(hobo::BasicElement::clone_html)
			}
		)*
	}};
}

svg![
	plus_circle => r"images/plus-circle.svg",
	minus_circle => r"images/minus-circle.svg",
	mail => r"images/mail.svg",
];
```

## Constructing inline SVGs

Of course, if you need to algorithmically construct an svg, such as if you're making a chart, you can do that too:

```rust
let mut svg = cmp::svg()
	.attr("viewBox", "-1 -1 2 2")
	.child(cmp::circle()
		.attr("cx", "0")
		.attr("cy", "0")
		.attr("r", "1")
		.class(css::class!(
			css::fill!(...colors::gray6),
		))
	);
```
