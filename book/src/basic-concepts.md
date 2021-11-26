# Core Concepts

This chapter outlines core types, traits and styling facilities that `hobo` employs.

Some note on terms used:
- **Entity**: a (usually) copyable id, that has components associated with it
- **Element**: not to be confused with HTML elements, an entity that has HTML or SVG components (which represent HTML elements) associated with it and so can have children, class, attributes, etc
- **Component**: not to be confused with `hobo::components` which is just a poorly named module for creating different HTML elements, any kind of data that may be associated with an Entity
- **Mutable**: not to be confused with Rust's notion of mutability, a type from `futures_signals` that can be used to produce signals
