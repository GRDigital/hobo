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
