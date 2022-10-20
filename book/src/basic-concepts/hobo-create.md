# `hobo::create`

This module has a `snake_case` function which returns a corresponding `PascalCase` concrete type that implements **Element**.

```rust,noplaypen
let some_div: hobo::create::Div = hobo::create::div();
```

**Element** has methods that aren't available on regular entities.

## `hobo::AsElement` and `hobo::AsEntity`

Sometimes it's useful to have custom types so you can have some special capabilities on your **Entities** or **Elements**.

```rust,noplaypen
#[derive(hobo::AsElement, Clone, Copy /* etc */)]
struct Checkbox(hobo::create::Div);

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

The `hobo::AsElement` derive macro expects either a tuple struct or a regular struct where the **Entity** field is named `element` e.g.

```rust,noplaypen
#[derive(hobo::AsElement, Clone, Copy /* etc */)]
struct CustomSelect {
	element: hobo::create::Select,
	// etc
}
```

## `Element` and type erasure

It's often useful to mix different types of **Elements**, for example:

```rust,noplaypen
fn content() -> impl hobo::Element {
	match tab {
		Tab::Main => main_page(), // hobo::create::Div
		Tab::Blogpost => article(), // hobo::create::Article
		// etc
	}
}
```

This won't compile, but the distinction between types in this case isn't useful. So we can erase the concrete types and get the general `Element`:

```rust,noplaypen
fn content() -> impl hobo::Element {
	match tab {
		Tab::Main => main_page().erase(), // hobo::Element
		Tab::Blogpost => article().erase(), // hobo::Element
		// etc
	}
}
```

If you have a regular **Entity** or something that at least implements `hobo::AsEntity` - you can recover **Element** capabilities by just constructing a `Element`:

```rust,noplaypen
let elem = hobo::Element(some_entity);
```

This pattern is often useful when using queries to find elements, as queries often return entities (more on them in [queries](../state/queries.md))

```rust,noplaypen
let (entity, _) = hobo::find_ond::<Entity, With<ComponentFoo>>();
// We know that this enetity is a Div we've made, but we need e.g. it's type to be a Div, not Entity
let element = hobo::create::Div(entity);
```
One can think of it almost as casting - we're fetching an entity which we, as the writer,
know is a Div - however, we need to specify that type in code, or need Element capabilities, etc.
