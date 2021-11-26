# `hobo::components`

This module has a `snake_case` function which returns a corresponding `PascalCase` concrete type that implements **Element**.

```rust,noplaypen
let some_div: hobo::components::Div = hobo::components::div();
```

**Element** has methods that aren't available on regular entities.

## `hobo::Element` and `hobo::AsEntity`

Sometimes it's useful to have custom types so you can have some special capabilities on your **Entities** or **Elements**.

```rust,noplaypen
#[derive(hobo::Element, Clone, Copy /* etc */)]
struct Checkbox(hobo::components::Div);

// just an example of why you might want to do this
impl Checkbox {
	fn is_checked(&self) -> bool {
		*self.get_cmp::<bool>()
	}

	fn set_checked(&self, checked: bool) {
		*self.get_cmp_mut_or_default::<bool>() = checked;
	}

	// probably etc methods
}
```

The `hobo::Element` derive macro expects either a tuple struct or a regular struct where the **Entity** field is named `element` e.g.

```rust,noplaypen
#[derive(hobo::Element, Clone, Copy /* etc */)]
struct CustomSelect {
	element: hobo::components::Select,
	// etc
}
```

## `SomeElement` and type erasure

It's often useful to mix different types of **Elements**, for example:

```rust,noplaypen
fn content() -> impl hobo::Element {
	match tab {
		Tab::Main => main_page(), // hobo::components::Div
		Tab::Blogpost => article(), // hobo::components::Article
		// etc
	}
}
```

This won't compile, but the distinction between types in this case isn't useful. So we can erase the concrete types and get the general `SomeElement`:

```rust,noplaypen
fn content() -> impl hobo::Element {
	match tab {
		Tab::Main => main_page().erase(), // hobo::SomeElement
		Tab::Blogpost => article().erase(), // hobo::SomeElement
		// etc
	}
}
```

If you have a regular **Entity** or something that at least implements `hobo::AsEntity` - you can recover **Element** capabilities by just constructing a `SomeElement`:

```rust,noplaypen
let elem = hobo::SomeElement(some_entity);
```
