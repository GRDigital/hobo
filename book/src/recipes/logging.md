# Logging

There is no stdout in the browser so the simplest way is to use the [log](https://crates.io/crates/log) crate with [wasm-logger](https://crates.io/crates/wasm-logger) and [console_error_panic_hook](https://crates.io/crates/console_error_panic_hook) to see nicely formatted errors:

```rust,noplaypen
#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    // etc init and mounting of elements
    
    log::info!("it works!");
}
```
