# Capability traits

There are several derivable traits, usually derived alongside `hobo::Element`, which denote a provided capability of some components. That is to say, deriving all of them always isn't necessary, but more like describing what can some component do.

* `hobo::Container` - for components that can have children, which is most HTML elements except `<input>` and the like.
* `hobo::EventTarget` - for components that can handle keyboard/mouse/etc events.
* `hobo::RawElement` - for components which have a known particular web_sys element which can be extracted. `Box<dyn hobo::Element>` is a valid component, but the type of underlying element is lost.
* `hobo::Replaceable` - for components which can be replaced with an instance of themselves, usually it's used for a `Rc<RefCell<T>> where T: hobo::Element`.
