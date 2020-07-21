# Slot

`Slot` is essentially a `Rc<RefCell<Box<dyn Element>>` with a more convenient constructor. It is useful if you have a component of unknown type and that you wish to replace time to time, e.g. main page container, a table cell or if there's a lot of variant types and you don't want to make a tuple because you don't particularly care about preserving the type information.
