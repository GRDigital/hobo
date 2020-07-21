# hobo

**hobo** is a Rust frontend framework. Works on **stable Rust**.    
**STILL WIP AND EXPERIMENTAL** although used in production by [GR Digital](https://grdigital.co.uk/).

* **no virtual DOM** - html elements are managed manually and can be accessed directly via `web_sys::HtmlElement` to manage attributes/contents/etc
* no Model-View-Update (aka Elm architecture) - state management is manual, although there are helpful utilities
* no HTML macros - just Rust
* built-in macro-based styling, kind of like JS in CSS except it's just Rust

In practice, the code ends up being no more verbose than with more conventional frontend frameworks, however quite a bit more straightforward.

# Roadmap/TODO:
* **SSR**
* @-rules support
* More thorough testing
* `CSSStyleSheet.insertRule()` in release
* Threading support
