## Some way to do conditionals in chains, e.g.
```rust
	e::div()
		.add_child_if(condition, move || e::div().text("this only sometimes"))
```

## On-click once
rather than 
```rust
	element.add_on_click(move |_| {
		run_some_fn_taking_ownership(owned_string.clone(), some_struct.clone());
	});
```

do this
```rust
	element.add_on_click_once(move |_| {
		run_some_fn_taking_ownership(owned_string, some_struct);
	});
```

Should be possible by using `Closure::once_into_js` and also having special handlers that are also dropped as soon as the fn is called. Useful for stuff like buttons that do work and delete element, buttons that navigate to different pages, etc.
