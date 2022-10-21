# Building the DOM

Assembling elements is usually done via function chaining, but every function has a non-chained variant for use in loops or in case ownership gets tricky.

Here's an example of a somewhat involved element:

```rust
pub use hobo::{prelude::*, create as e};

#[derive(hobo::Element)]
pub struct Input {
    element: e::Div,
    pub input: e::Input,
}

impl Input {
    pub fn new(caption_text: &str, svg: Option<e::Svg>) -> Self {
        let input = e::input()
            // shortcut for .attr(web_str::r#type(), web_str::text())
            .type_text()
            .class(css::class! {
                // some style
            });

        let caption_text = e::div()
            .text(caption_text)
            .class(css::class! {
                // some style
            });

        let mut element = e::div()
            .class(css::style! {
                // some style
            })
            .child(input)
            .child(caption_text);

        if let Some(svg) = svg {
            element.add_child(
                svg.class(css::class! {
                    // some style
                })
            );
        }

        Self { element, input }
    }
}
```

### `.children()`

Same as `.child()` but can consume an `impl IntoIterator`, convenient when taking a `Vec<T>` as an argument in list-like element constructors.   
There is no `.children_signal()` but it could potentially exist - PRs welcome!

### Chaining vs non-chaining syntax

Most functions have a chaining syntax, handy when constructing the element, and also non-chaining syntax for use in loops or other contexts. The convention is `.<foo>` for chaining and `.add_<foo>` for non-chaining. This goes against the more common Rust convention of `.with_<foo>` being the chaining syntax, this is because most code will be simple elements constructed in bulk, so most of these calls will be chaining so a shorter name is preferred.

* `.child()`/`.child_signal()` vs `.add_child()`/`.add_child_signal()`
* `.children()` vs `.add_children()`
* `.class()`/`.class_tagged()`/`.class_typed()`/`.class_signal()` vs `.set_class()`/`.set_class_tagged()`/`.set_class_typed()`/`.set_class_signal()`
* `.style()`/`.style_signal()` vs `.set_style()`/`.set_style_signal()`
* `.attr()`/`.bool_attr()`/`.attr_signal()`/`.bool_attr_signal()` vs `.set_attr()`/`.set_bool_attr()`/`.set_attr_signal()`/`.set_bool_attr_signal()`
* `.<event>()` vs `.add_<event>()`
* `.text()`/`.text_signal()` vs `.set_text()`/`.set_text_signal()`
* `.component()` vs `.add_component()`
