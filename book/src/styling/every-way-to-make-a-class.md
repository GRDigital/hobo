# Every way to make a class

Apart from regular `.class()`/`.set_class()` options there's several others:

* `.mark::<T>()`/`.unmark::<T>()` - can generate classes from any type for targeted selection. 

```rust,noplaypen
use hobo::create as e;

struct ButtonMarker;

e::div()
    .class(css::style!(
        .& >> .[ButtonMarker] {
            css::cursor!(pointer),
        }
    ))
    .child(e::div()
        .mark::<ButtonMarker>()
        .text("button 1")
    )
    .child(e::div()
        .mark::<ButtonMarker>()
        .text("button 2")
    )
```

Every call to `.class()`/`.set_class()` will append a new class - if you want to override an existing one, there are two options:

* `.set_class_typed::<Type>(style)` - generates a tag from a `Type`. This is usually the preferred method, in the rare case that you need to override classes.

```rust,noplaypen
use hobo::create as e;

struct Flexible;

e::div()
	.class(css::display!(flex))
	.class_typed::<Flexible>((
		css::flex_direction!(row),
		css::width!(100 px),
	))
	.on_click(|&element| {
		element
			.set_class_typed::<Flexible>((
				css::flex_direction!(row),
				css::width!(100 px),
			))
	})
```

* `.set_class_tagged::<Tag: Hash>(tag, style)` - Similar to `.set_class_tagged`, but uses an instance of a type rather than `Type`. The regular `.class()` method uses this internally with just an incrementing `u64` for a tag.

```rust,noplaypen
use hobo::create as e;

e::div()
	.class(css::display!(flex))
	.class_tagged("Flexible", (
		css::flex_direction!(row),
		css::width!(100 px),
	))
	.on_click(|&element| {
		element
			.set_class_tagged("Flexible", (
				css::flex_direction!(column),
				css::height!(100 px),
			))
	})
```

Prefer using this over `.set_class_typed` if your tag is computed at runtime.

* signals - you can have your classes be set reactively, in response to some changes in a `Mutable`. This is the preferred method for anything reactive, such as switching between themes:

```rust,noplaypen
enum Theme {
	Light,
	Dark,
}

let theme = Mutable::new(Theme::Light);

e::div()
	.class_typed_signal::<Theme, _, _>(theme.signal().map(|theme| {
		match theme {
			Theme::Light => css::background_color!(css::color::WHITE),
			Theme::Dark => css::background_color!(css::color::BLACK),
		}
	}))
	.component(theme)
```
