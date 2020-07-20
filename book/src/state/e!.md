# e!() macro

The `e!()`'s basic use is to just clone variables into closures, similar to what [enclose](https://crates.io/crates/enclose) does.

There are several common hobo idioms which make use of it:

```rust
e!((%foo) move || {
	foo.bar = 50;
})
```

would turn into something like

```rust
{
	let foo = Rc::downgrade(&foo);
	move || {
		let foo = if let Some(x) = Weak::upgrade(&foo) { x } else { return; };
		{
			foo.bar = 50;
		}
	}
}
```

It is primarily useful to create event handlers which can be dropped when their owning component is dropped. Otherwise, there's an Rc cycle where the component owns an Rc of the event handler and the event handler owns an Rc of the component, so they leak memory. Since event handlers must be `'static` because they're passed to the DOM - it's not possible to tie its lifetime to the lifetime of the component.
