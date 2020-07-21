# hobo::Element

Anything implementing `hobo::Element` is considered a hobo component. It is derivable on structs where there is a field named `element` which also implements `hobo::Element` or on enums with single-element tuple variants, where each of those implements `hobo::Element`.

```rust
#[derive(hobo::Element)]
struct Foo {
	element: cmp::Div,
}

// also ok
#[derive(hobo::Element)]
enum Bar {
	Foo(Foo),
	Div(cmp::Div),
}
```

All basic HTML and SVG elements have a shortcut for creating them with a `cmp::<tag>` syntax. The type is a `hobo::BasicElement<T>` where `T` is the most appropriate `web_sys` type, aliased as `cmp::<Tag>`. In other words, calling `cmp::div()` creates a `cmp::Div`. It is also implemented for most smart pointers, check the docs for more.
