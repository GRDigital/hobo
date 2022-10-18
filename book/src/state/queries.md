# Queries

Queries allow finding individual **Entities** or collections of **Entities**. Best shown by examples:

```rust,noplaypen
struct Foo {
	// some fields
}

// find the first (presumably only) entity with some component Foo
let (entity, _) = hobo::find_one::<(Entity, With<Foo>)>();
let element = SomeElement(entity);
element.set_text("This entity has Foo");
```

```rust,noplaypen
struct Frobnicator {
	num_fraculations: u32,
	// other fields
}

// find all entities with a Frobnicator component and mutate it
// perhaps as a result of some combined transformation
for frobnicator in hobo::find::<&mut Frobnicator>() {
	frobnicator.num_fraculations += 1;
}
```

Queries are tuples of `& T`, `&mut T` or `With<T>` where `T` is some component or, as a special case, `Entity`. The result of `hobo::find` (or `hobo::find_one`) are tuples where each member is what was requested by the query (`With<T>` will always return `true` in its position because any entity that doesn't have `T` won't be included in the output).    

Queries are also often useful to establish relations with distant **Elements**. For example, an **Element** in one part of the DOM can get an **Element** from a completely unrelated part of the DOM.

```rust,noplaypen
use hobo::create as e;

struct SettingsData {
	speed: f32,
}

let settings_container = e::div()
	// etc
	.component(SettingsData { speed: 0.35 })

// -- snip --

let unrelated_display = e::div()
	//etc
	.text(hobo::find_one::<&SettingsData>().speed.to_string())
```
