# hobo

<a href="https://crates.io/crates/hobo"><img alt="Crate Info" src="https://img.shields.io/crates/v/hobo.svg"/></a>
<a href="https://docs.rs/hobo/"><img alt="API Docs" src="https://img.shields.io/badge/docs.rs-hobo-yellow"/></a>

**hobo** is an opinionated, batteries-included Rust frontend framework. Works on **stable Rust**.    
**STILL WIP** although used in production by [GR Digital](https://grdigital.co.uk/)    
[Check out the Book](https://grdigital.github.io/hobo/index.html)!

### Notable features:

* **no virtual DOM** - html elements are just components added to entities and can be accessed directly via `web_sys::HtmlElement`
* no Model-View-Update (aka Elm architecture) - state management is manual, usually via Entity-Component relations
* no HTML macros - just Rust functions
* built-in macro-based styling, kind of like CSS-in-JS except it's just Rust
* **reactivity support** via [rust-signals](https://github.com/Pauan/rust-signals)
* Entity-Component based approach allowing flexible state propagation and cohesion between elements without coupling or a need for global store or state

### Sneak peek:
```rust,noplaypen
fn counter() -> impl hobo::Element {
    let counter = Mutable::new(0);

    cmp::div()
        .class((
            css::display!(flex),
            css::width!(400 px),
        ))
        .child(cmp::div()
            .text_signal(counter.signal().map(|value| format!("Counter value is: {}", value)))
        )
        .child(cmp::button()
            .text("increment")
            .on_click(move |_| *counter.lock_mut() += 1)
        )
}
```
