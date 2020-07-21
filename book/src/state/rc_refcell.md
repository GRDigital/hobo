# Rc\<RefCell\<T>>

Since event handlers have to be `'static` it's, for the most part, not possible to express the lifetime relations between elements, their corresponding components, their event handlers and miscellaneous fields in those components. Thus, it's often pretinent to turn to runtime-checked `Rc<RefCell<T>>`.

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

Here's an example of a component making use of it:

```rust
#[derive(hobo::Element, hobo::Container, hobo::EventTarget)]
pub struct Checkbox {
	element: cmp::Input,
	value: Rc<RefCell<bool>>,
}

impl Checkbox {
	fn new(value: bool) -> Self {
		let element = cmp::input().attr(web_str::r#type(), web_str::checkbox());
		element.element.set_checked(value);
		let value = Rc::new(RefCell::new(value));

		element.on_change(e!((%value, *element) move |_| {
			*value.borrow_mut() = element.checked();
			// we don't have to change any styling or anything -
			// the html checkbox element's state is already definitely
			// in sync with our internal value
			// changing the internal value is necessary so the parent component
			// can recover it without using raw html interface
			// e.g. in case we later swap from checkbox inputs to a custom checkbox
		}));

		Self { element, value }
	}
}
```
