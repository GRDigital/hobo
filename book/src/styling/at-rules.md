# @-rules

Right now hobo only supports `@font-face` and a subset of `@media`

# `@font-face`

The block following `@font-face` is passed as initialization for `css::font_face::FontFace`. Check out the [docs](https://docs.rs/hobo_css/0.1/hobo_css/font_face/struct.FontFace.html).    
It looks something like this:

```rust
@font-face {
	src: vec![("https://fonts.gstatic.com/s/encodesans/v4/LDI2apOFNxEwR-Bd1O9uYPOreec.woff2".into(), Some(css::font_face::Format::Woff2))],
	font_family: "Encode Sans".into(),
	font_weight: (css::font_face::Weight::Number(400), None),
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
@media !All && Orientation(Portrait) && !AspectRatio(4, 3), Print && Color(4) && !Width(css::unit!(200 px)) {
	html {
		css::background_color!(rgb 0xFF_00_00),
	}
}
```

```css
@media not all and (orientation: portrait) and (not (aspect-ratio: 4/3)), print and (color: 4) and (not (width: 200px)) {
	html {
		background-color: #FF0000;
	}
}
```

Support for `@keyframes` and `@page` is planned
