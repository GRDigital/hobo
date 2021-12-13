# Elements that change

Since there's no VDOM, rebuilding the DOM is done manually by literally rebuilding the altered parts. It is on the developer to minimize this to maintain element focus, scroll position, performance, etc. The same goes for styling - any complex modification is best expressed as recreating the whole style.   

However, most modifications can often be expressed with signals, with some child, style or text of an element just being a result of some computation based on one or multiple Mutables. With regards to styling in particular, most of the style is probably not going to change, with only minor changes based on something like theme.

```rust,noplaypen
.class_typed_signal::<Theme, _, _>(theme.signal().map(|theme| {
	match theme {
		Theme::Light => css::background_color!(css::color::WHITE),
		Theme::Dark => css::background_color!(css::color::BLACK),
	}
}))
```
