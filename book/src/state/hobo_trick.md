# #[hobo::trick]

This macro is usually applied to a component's `new()` constructor. Through some careful `MaybeUninit` trickery, it allows a special syntax for event handlers, which get a conveniently syntactically-sweetened `&mut Self` rather than `Rc<RefCell<Self>>` or rather than having to pass a bunch of `Rc<RefCell<FieldT>>` into the event handler like `e!((%field1, %field2) move |_| { ... })`.

It injects a variable `this` of type `Rc<MaybeUninit<RefCell<Self>>>` which is not meant to be interacted with, but can be passed to `with_<event>_mut` and `<event>_mut` family of functions, and then the event handler can take `this` of type `&mut Self` as the first argument. To bring back the simplified Counter example from the introduction:

```rust
#[derive(hobo::Element, hobo::Container, hobo::EventTarget, hobo::RawElement)]
pub struct Counter {
	element: cmp::Div,
	count: i32,
}

impl Counter {
	#[hobo::trick]
	fn new() -> Self {
		Self { element: cmp::div(), count: 0 }
			.with_on_click_mut(&this, move |this, _| {
				this.count += 1;
			})
	}
}
```

is expanded to something like:

```rust
impl Counter {
	fn new() -> Rc<RefCell<Self>> {
		let mut this: Rc<MaybeUninit<RefCell<Self>>> = Rc::new(MaybeUninit::uninit());
		let new_this = {
			let new_this = Self { element: cmp::div(), count: 0 };
			// this is just showing roughly what would .with_on_click_mut translate to
			let weak = Rc::downgrade(this);
			new_this
				.with_on_click(move |event| {
					let strong = if let Some(x) = weak.upgrade() { x } else { return; };
					let inited: Rc<RefCell<T>> = unsafe {
						Rc::from_raw((&*Rc::into_raw(strong)).as_ptr())
					};
					let this = &mut inited.borrow_mut();

					// original handler block
					{
						this.count += 1;
					}
				})
		};
		unsafe {
			let raw_uninit = Rc::into_raw(this) as *mut MaybeUninit<_>;
			let raw_init = (&mut *raw_uninit).as_mut_ptr();
			std::ptr::write(raw_init, RefCell::new(new_this));
			Rc::from_raw(raw_init)
		}
	}
}
```
