# hobo

<a href="https://crates.io/crates/hobo"><img alt="Crate Info" src="https://img.shields.io/crates/v/hobo.svg"/></a>
<a href="https://docs.rs/hobo/"><img alt="API Docs" src="https://img.shields.io/badge/docs.rs-hobo-yellow"/></a>

**hobo** is a Rust frontend framework. Works on **stable Rust**.    
**STILL WIP AND EXPERIMENTAL** although used in production by [GR Digital](https://grdigital.co.uk/)    
[Check out the Book](https://grdigital.github.io/hobo/index.html)!

* **no virtual DOM** - html elements are managed manually and can be accessed directly via `web_sys::HtmlElement` to manage attributes/contents/etc
* no Model-View-Update (aka Elm architecture) - state management is manual, although there are helpful utilities
* no HTML macros - just Rust
* built-in macro-based styling, kind of like JS in CSS except it's just Rust

In practice, the code ends up being no more verbose than with more conventional frontend frameworks, however quite a bit more straightforward.

## Roadmap/TODO:
* **SSR**
* @-rules support
* More thorough testing
* `CSSStyleSheet.insertRule()` in release
* Threading support
