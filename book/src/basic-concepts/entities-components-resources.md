# Entities, Components (and Resources)

The backbone of the framework is the Entity-Component approach of associating data. **Entities** are just incrementing `u64`s under the hood, they carry no data.    

**Elements** are no different in this regard, the only difference is that **Elements** have a compile time promise that these entities have `web_sys::Node`, `web_sys::Element`, `web_sys::EventTarget` and one of `web_sys::HtmlElement` or `web_sys::SvgElement` attached to them. As a consequence, these **Entities** can get styled, get attributes and compose into DOM.

**Resources** are same as **Components** but they are accessible globally, they aren't associated with any entity. Only one instance of a type of **Resource** can exist at any time, in this way they are similar to singletons from other programming languages.
