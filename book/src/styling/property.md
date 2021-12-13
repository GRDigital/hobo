# Property

Most css properties will be expressed as tuples of anything that implements `hobo::css::AppendProperty`, which includes:

* `css::Property` such as created by the `css::<prop>()` family of macros (e.g. `css::width!()`, `css::flex_shrink!()`, etc)
* `Vec<css::Property>`
* `()`
* `&'static str` and `String` as escape hatches
* `FnOnce(&mut Vec<Property>)` for rare complex logic
* Other tuples of things that implement `hobo::css::AppendProperty`
* Enum-like property variants e.g. `css::Display::Flex` or `css::TextDecorationStyle::Solid`

Conditional property inclusion could be expressed as different `Vec<css::Property>` where one is empty, e.g.

```rust,noplaypen
(
	css::display!(flex),
	if active {
		vec![css::background_color!(0x00_00_FF_FF)],
	} else {
		vec![],
	},
)
```

Or alternatively, by leveraging `FnOnce`

```rust,noplaypen
(
	css::display!(flex),
	move |props| if active { props.push(css::background_color!(0x00_00_FF_FF)); },
)
```
