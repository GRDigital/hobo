## Detecting Rc cycles
* An alternative `e!` macro with a special character for Rc's, always downgrades and upgrades when entering closure, early exiting if any are dropped
* Idk if possible, some kind of `#[hobo::evict]` attribute to make sure that the strong count is 0 after drop
