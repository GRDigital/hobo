# @-rules

Right now hobo only supports `@font-face` and a subset of `@media`

# `@font-face`

The block following `@font-face` is passed as initialization for `css::font_face::FontFace`. Check out the [docs](https://docs.rs/hobo_css/0.1/hobo_css/font_face/struct.FontFace.html).    
It looks something like this:

```rust
@font-face {
	src: vec![("https://fonts.gstatic.com/.../....woff2".into(), Some(Format::Woff2))],
	font_family: "Encode Sans".into(),
	font_weight: (Weight::Number(400), None),
}
```

# `@media`

The syntax is different to `@media` rules in css:

* specifying media type is not optional
* `!` instead of `not`
* `CamelCase` instead of `kebab-case`
* `&&` instead of `and`
* no grouping rules in `not` clauses

So these two would be equivalent:

```rust
@media All && MaxWidth(css::unit!(1023 px)) {
	html {
		css::background_color!(rgb 0xFF_00_00),
	}
}
```

```css
@media all and (max-width: 1023px) {
	html {
		background-color: #FF0000;
	}
}
```

Support for `@keyframes` and `@page` is planned, but meanwhile the escape hatch raw string syntax can be used if really necessary.
