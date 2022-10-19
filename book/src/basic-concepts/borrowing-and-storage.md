# Borrowing and Storage

Components for entites are stored in a simple map - `HashMap<Entity, Component>` (see, `hobo::storage`).

(This also makes searching for components via `hobo::find_one` *very* cheap).

Rust's ownership rules ensure that a mutable borrow is exclusive, which means that we cannot have mutable references 
to components while immutable ones exists (or vice-versa).

Here's an example of how this affects hobo:

```rust,noplaypen
// src\example_file.rs

mod example_module {
    pub use hobo::{prelude::*, create as e};

    struct Foo;

    pub fn test() -> impl hobo::AsElement {
        e::div()
            .component(Foo)
            .with(|&element| {
                // Ok
                let foo1 = element.get_cmp::<Foo>();
                // Still ok
                let foo2 = element.get_cmp::<Foo>();
                // Panic!
                let foo3 = element.get_cmp_mut::<Foo>();
            })
    }
}
```

This, of course, also applies to queries/find/etc.

This can be a bit tricky to debug in Wasm, which is why when compiling in debug mode, 
hobo will display the following helpful message in the browser's console if a borrow-related runtime panic is encountered:

```
panicked at ''already borrowed': Trying to mutably borrow `example_module::Foo` storage at `src\example_file.rs:16:50` while other borrows to it already exist:

(mut) src\example_file.rs:16:50
      src\example_flib.rs:14:50
      src\example_flib.rs:12:50
```
This will list **only** the currently active borrows, as well as the mutable one, in order of access.

Every type we store as a component will have it's own storage, so it is fine to mutable borrows of different types.

The way to circumvent this would be similar to how one would for any other ownership issue:

You can clone the value for reference if you don't need to know what it is after mutation:

```rust,noplaypen
#[derive(Clone)]
struct Foo;

pub fn test() -> impl hobo::AsElement {
    e::div()
        .component(Foo)
        .with(|&element| {
            let foo1 = element.get_cmp::<Foo>().clone();
            let foo3 = element.get_cmp_mut::<Foo>();
        })
}
```

Or, you can drop the guard, ensuring that no references conflict.

```rust,noplaypen
struct Foo;

pub fn test() -> impl hobo::AsElement {
    e::div()
        .component(Foo)
        .with(|&element| {
            {
                let foo1 = element.get_cmp::<Foo>();
            }
            
            let foo2 = element.get_cmp_mut::<Foo>();
            std::mem::drop(foo2);

            let foo3 = element.get_cmp::<Foo>();
        })
}
```
