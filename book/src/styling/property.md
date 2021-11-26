# Property

hobo property-collecting `css::properties!()` macro (internally used in `css::class()` and `css::style!()`) accepts anything which implements `hobo::css::AppendProperty`, which includes:

* `css::Property` such as created by the `css::<prop>()` family of macros (e.g. `css::width!()`, `css::flex_shrink!()`, etc)
* `Vec<css::Property>`
* `()`
* `&'static str` and `String` as escape hatches
* `FnOnce(&mut Vec<Property>)` for rare complex logic
* Unique types used as tuple variants in `css::Property`, e.g. `css::Display::Flex` is of type `css::Display` yet the variant is `css::Property::Display(css::Display)` so just `css::Display` can be used as a `hobo::css::AppendProperty` as well

Conditional property inclusion is usually expressed as different `Vec<css::Property>` where one is empty, e.g.

```rust
css::properties!(
	if active {
		vec![css::background_color!(0x00_00_FF_FF)],
	} else {
		vec![],
	},
)
```
