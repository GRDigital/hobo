# @-rules

Right now hobo only supports `@font-face`, but in the nearest future support for `@media`, `@import` and other is planned.

```rust
@font-face {
	// raw string because this property hasn't been properly integrated into hobo yet
	"src:url(...) format('woff2');",
	css::font_family!("Encode Sans"),
	css::font_weight!(400),
	css::FontStyle::Normal,
}
```
