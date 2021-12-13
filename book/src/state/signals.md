# Signals

Hobo has some useful reactivity facilities. The core of this is `futures_signals::signal::Mutable`, from which signals are created, check the [futures-signals](https://docs.rs/futures-signals/0.3) documentation for details on how to do that.

* `text_signal`/`set_text_signal` - calls `set_text` whenever the signal value changes
* `child_signal`/`add_child_signal` - initially creates an empty `div`, then calls `.replace_with` every time the signal value changes
* `class_signal`/`set_class_signal` and `class_typed_signal`/`set_class_typed_signal` and `class_tagged_signal`/`set_class_tagged_signal` - calls `set_class_tagged` whenever the signal value changes
	* will always replace the first class so take care
* `attr_signal`/`set_attr_signal` and `bool_attr_signal`/`set_bool_attr_signal` - calls `set_attr` whenever the signal value changes
* `style_signal`/`set_style_signal` - calls `set_style` whenever the signal value changes
* `mark_signal` - calls `mark`/`unmark` whenever the signal value changes
