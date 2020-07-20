# hobo

**hobo** is a Rust frontend framework.    
**STILL WIP AND EXPERIMENTAL** although used in production by [GR Digital](https://grdigital.co.uk/).

* no virtual DOM - you manage html elements yourself and have direct access to them via `web_sys::HtmlElement` to manage attributes/contents/etc
* no Elm-like architecture - state management is manual, although there are helpful facilities for that
* no HTML macros - just Rust
* built-in macro-based styling, kind of like JS in CSS except it's just Rust
