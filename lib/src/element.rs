use crate::prelude::*;
pub use hobo_derive::Element;
use std::collections::HashMap;
use std::any::TypeId;
use std::borrow::Cow;
use futures_signals::signal::SignalExt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Element)]
pub struct SomeElement(pub Entity);

#[derive(Default, Debug, shrinkwraprs::Shrinkwrap, Clone, Copy, PartialEq, Eq, AsEntity)]
pub struct Parent(Entity);

#[derive(Default, Debug, shrinkwraprs::Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct Children(pub Vec<Entity>);

impl Parent {
	pub fn ancestors(entity: impl AsEntity) -> Vec<Entity> {
		if let Some(parent) = entity.try_get_cmp::<Parent>().map(|x| x.0) {
			let mut v = Self::ancestors(parent);
			v.push(parent);
			v
		} else {
			Vec::new()
		}
	}

	pub fn ancestor_with_cmp<T: 'static>(entity: Entity) -> Entity {
		let parent = entity.get_cmp::<Parent>().0;
		if WORLD.storage::<T>().has(parent) { parent } else { Parent::ancestor_with_cmp::<T>(parent) }
	}
}

impl Children {
	pub fn clear(entity: impl AsEntity) {
		if let Some(children) = entity.try_get_cmp_mut::<Children>().map(|mut x| x.0.drain(..).collect::<Vec<_>>()) {
			for child in children {
				WORLD.remove_entity(child);
			}
		}
	}
}

#[derive(Default)]
pub struct Classes {
	pub(crate) type_tag: Option<TypeId>,
	pub(crate) styles: HashMap<u64, css::Style>,
}

pub trait Element: AsEntity + Sized {
	fn add_child(&self, child: impl Element) {
		if WORLD.is_dead(self) { log::warn!("add_child parent dead {:?}", self.as_entity()); return; }
		if WORLD.is_dead(&child) { log::warn!("add_child child dead {:?}", child.as_entity()); return; }
		self.get_cmp_mut_or_default::<Children>().0.push(child.as_entity());
		child.get_cmp_mut_or_default::<Parent>().0 = self.as_entity();

		// why not unwrapping? how can this fail?
		if let (Some(parent_node), Some(child_node)) = (self.try_get_cmp::<web_sys::Node>(), child.try_get_cmp::<web_sys::Node>()) {
			parent_node.append_child(&child_node).expect("can't append child");
		}
	}
	fn child(self, child: impl Element) -> Self { self.add_child(child); self }
	fn add_children<Item: Element>(&self, children: impl IntoIterator<Item = Item>) { for child in children.into_iter() { self.add_child(child); } }
	fn children<Item: Element>(self, children: impl IntoIterator<Item = Item>) -> Self { self.add_children(children); self }

	// be mindful about holding child references with this one
	fn add_child_signal<S, E>(&self, signal: S) where
		E: Element,
		S: futures_signals::signal::Signal<Item = E> + 'static,
	{
		// placeholder at first
		let mut child = crate::create::components::div().class(crate::css::Display::None).erase();
		self.add_child(child);
		wasm_bindgen_futures::spawn_local(signal.for_each(move |new_child| {
			let new_child = new_child.erase();
			child.replace_with(new_child);
			child = new_child;
			async {}
		}));
	}
	fn child_signal<S, E>(self, signal: S) -> Self where
		E: Element,
		S: futures_signals::signal::Signal<Item = E> + 'static,
	{ self.add_child_signal(signal); self }

	fn set_class_tagged<Tag: std::hash::Hash + 'static>(&self, tag: Tag, style: impl Into<css::Style>) {
		if WORLD.is_dead(self) { log::warn!("set_class_tagged dead {:?}", self.as_entity()); return; }

		// tested and different types with same byte-level representation hash to the same thing (not surprising)
		// i.e. the type is not taken into account when hashing so I have to do it manually
		let tag_hash = {
			use std::hash::{Hash, Hasher};
			let mut hasher = std::collections::hash_map::DefaultHasher::new();
			TypeId::of::<Tag>().hash(&mut hasher);
			tag.hash(&mut hasher);
			hasher.finish()
		};

		self.get_cmp_mut_or_default::<Classes>().styles.insert(tag_hash, style.into());
	}
	// Cannot mix impl Into<css::Style> with generic type arguments
	fn set_class_typed<Type: 'static>(&self, style: css::Style) {
		self.set_class_tagged(TypeId::of::<Type>(), style)
	}
	fn set_class(&self, style: impl Into<css::Style>) { self.set_class_tagged(0u64, style); }
	fn add_class(&self, style: impl Into<css::Style>) {
		let id = self.try_get_cmp::<Classes>().map(|x| x.styles.len() as u64).unwrap_or(0);
		self.set_class_tagged(id, style);
	}
	fn class(self, style: impl Into<css::Style>) -> Self { self.add_class(style); self }
	fn class_tagged<Tag: std::hash::Hash + 'static>(self, tag: Tag, style: impl Into<css::Style>) -> Self { self.set_class_tagged(tag, style); self }
	fn class_typed<Type: 'static>(self, style: css::Style) -> Self { self.set_class_typed::<Type>(style); self }

	fn set_class_signal<S, I>(&self, signal: S) where
		I: Into<css::Style>,
		S: futures_signals::signal::Signal<Item = I> + 'static,
	{
		let entity = self.as_entity();
		if WORLD.is_dead(entity) { log::warn!("set_class_signal dead entity {:?}", entity); return; }
		wasm_bindgen_futures::spawn_local(signal.for_each(move |class| {
			SomeElement(entity).set_class(class);
			async move { }
		}));
	}
	fn class_signal<S, I>(self, signal: S) -> Self where
		I: Into<css::Style>,
		S: futures_signals::signal::Signal<Item = I> + 'static,
	{ self.set_class_signal(signal); self }

	fn set_class_typed_signal<Type, S, I>(&self, signal: S) where
		Type: 'static,
		I: Into<css::Style>,
		S: futures_signals::signal::Signal<Item = I> + 'static,
	{
		let entity = self.as_entity();
		if WORLD.is_dead(entity) { log::warn!("set_class_signal dead entity {:?}", entity); return; }
		wasm_bindgen_futures::spawn_local(signal.for_each(move |class| {
			SomeElement(entity).set_class_typed::<Type>(class.into());
			async move { }
		}));
	}
	fn class_typed_signal<Type, S, I>(self, signal: S) -> Self where
		Type: 'static,
		I: Into<css::Style>,
		S: futures_signals::signal::Signal<Item = I> + 'static,
	{ self.set_class_typed_signal::<Type, S, I>(signal); self }

	fn set_class_tagged_signal<Tag, S, I>(&self, signal: S) where
		Tag: std::hash::Hash + 'static,
		I: Into<css::Style>,
		S: futures_signals::signal::Signal<Item = (Tag, I)> + 'static,
	{
		let entity = self.as_entity();
		if WORLD.is_dead(entity) { log::warn!("set_class_signal dead entity {:?}", entity); return; }
		wasm_bindgen_futures::spawn_local(signal.for_each(move |(tag, class)| {
			SomeElement(entity).set_class_tagged(tag, class);
			async move { }
		}));
	}
	fn class_tagged_signal<Tag, S, I>(self, signal: S) -> Self where
		Tag: std::hash::Hash + 'static,
		I: Into<css::Style>,
		S: futures_signals::signal::Signal<Item = (Tag, I)> + 'static,
	{ self.set_class_tagged_signal::<Tag, S, I>(signal); self }

	fn set_attr<'k, 'v>(&self, key: impl Into<Cow<'k, str>>, value: impl Into<Cow<'v, str>>) {
		if WORLD.is_dead(self) { log::warn!("set_attr dead {:?}", self.as_entity()); return; }
		let key = key.into();
		let value = value.into();
		self.get_cmp::<web_sys::Element>().set_attribute(&key, &value).unwrap_or_else(|_| panic!("can't set attribute {} to {}", key, value));
	}
	fn attr<'k, 'v>(self, key: impl Into<Cow<'k, str>>, value: impl Into<Cow<'v, str>>) -> Self { self.set_attr(key, value); self }
	fn set_bool_attr<'k>(&self, key: impl Into<Cow<'k, str>>, value: bool) { if value { self.set_attr(key, "") } else { self.remove_attr(key) } }
	fn bool_attr<'k>(self, key: impl Into<Cow<'k, str>>, value: bool) -> Self { self.set_bool_attr(key, value); self }
	fn remove_attr<'k>(&self, key: impl Into<Cow<'k, str>>) {
		if WORLD.is_dead(self) { log::warn!("remove_attr dead {:?}", self.as_entity()); return; }
		self.get_cmp::<web_sys::Element>().remove_attribute(&key.into()).expect("can't remove attribute");
	}

	fn set_attr_signal<'k, 'v, S, K, V>(&self, signal: S) where
		K: Into<Cow<'k, str>>,
		V: Into<Cow<'v, str>>,
		S: futures_signals::signal::Signal<Item = (K, V)> + 'static,
	{
		let entity = self.as_entity();
		if WORLD.is_dead(entity) { log::warn!("set_attr_signal dead entity {:?}", entity); return; }
		wasm_bindgen_futures::spawn_local(signal.for_each(move |(k, v)| {
			SomeElement(entity).set_attr(k, v);
			async move { }
		}));
	}
	fn attr_signal<'k, 'v, S, K, V>(self, x: S) -> Self where
		K: Into<Cow<'k, str>>,
		V: Into<Cow<'v, str>>,
		S: futures_signals::signal::Signal<Item = (K, V)> + 'static,
	{ self.set_attr_signal(x); self }

	fn set_bool_attr_signal<'k, S, K>(&self, signal: S) where
		K: Into<Cow<'k, str>>,
		S: futures_signals::signal::Signal<Item = (K, bool)> + 'static,
	{
		let entity = self.as_entity();
		if WORLD.is_dead(entity) { log::warn!("set_attr_signal dead entity {:?}", entity); return; }
		wasm_bindgen_futures::spawn_local(signal.for_each(move |(k, v)| {
			SomeElement(entity).set_bool_attr(k, v);
			async move { }
		}));
	}
	fn bool_attr_signal<'k, S, K>(self, x: S) -> Self where
		K: Into<Cow<'k, str>>,
		S: futures_signals::signal::Signal<Item = (K, bool)> + 'static,
	{ self.set_bool_attr_signal(x); self }

	fn set_text<'a>(&self, text: impl Into<std::borrow::Cow<'a, str>>) {
		if WORLD.is_dead(self) { log::warn!("set_text dead entity {:?}", self.as_entity()); return; }
		self.get_cmp::<web_sys::HtmlElement>().set_inner_text(&text.into());
	}
	fn text<'a>(self, x: impl Into<std::borrow::Cow<'a, str>>) -> Self { self.set_text(x); self }

	fn set_text_signal<'a, S, I>(&self, signal: S) where
		I: Into<Cow<'a, str>>,
		S: futures_signals::signal::Signal<Item = I> + 'static,
	{
		let entity = self.as_entity();
		if WORLD.is_dead(entity) { log::warn!("set_text_signal dead entity {:?}", entity); return; }
		wasm_bindgen_futures::spawn_local(signal.for_each(move |text| {
			SomeElement(entity).set_text(text);
			async move { }
		}));
	}
	fn text_signal<'a, S, I>(self, x: S) -> Self where
		I: Into<Cow<'a, str>>,
		S: futures_signals::signal::Signal<Item = I> + 'static,
	{ self.set_text_signal(x); self }

	fn set_style(&self, style: impl AppendProperty) {
		let mut props = Vec::new();
		style.append_property(&mut props);
		self.set_attr(web_str::style(), props.iter().map(std::string::ToString::to_string).collect::<String>());
	}
	fn style(self, style: impl AppendProperty) -> Self { self.set_style(style); self }
	fn remove_style(&self) { self.remove_attr(web_str::style()); }

	fn set_style_signal<S, I>(&self, signal: S) where
		I: AppendProperty,
		S: futures_signals::signal::Signal<Item = I> + 'static,
	{
		let entity = self.as_entity();
		if WORLD.is_dead(entity) { log::warn!("set_style_signal dead entity {:?}", entity); return; }
		wasm_bindgen_futures::spawn_local(signal.for_each(move |style| {
			SomeElement(entity).set_style(style);
			async move { }
		}));
	}
	fn style_signal<S, I>(self, signal: S) -> Self where
		I: AppendProperty,
		S: futures_signals::signal::Signal<Item = I> + 'static,
	{ self.set_style_signal(signal); self }

	fn mark<T: 'static>(self) -> Self {
		if WORLD.is_dead(&self) { log::warn!("mark dead {:?}", self.as_entity()); return self; }
		self.get_cmp_mut_or_default::<Classes>().type_tag = Some(TypeId::of::<T>());
		self
	}

	// TODO: this should steal components from other and delete it
	// instead of deleting self
	// this would cause a lot less issue with invalidating stuff
	// !!!!!! NOT TRUE - any handler that was created with the new entity will be busted, so this is fine
	fn replace_with<T: AsEntity>(&self, other: T) -> T {
		let other_entity = other.as_entity();
		if WORLD.is_dead(self) { log::warn!("replace_with dead {:?}", self.as_entity()); return other; }

		// why not unwrapping? how can this fail?
		if let (Some(this), Some(other)) = (self.try_get_cmp::<web_sys::Element>(), other_entity.try_get_cmp::<web_sys::Node>()) {
			this.replace_with_with_node_1(&other).unwrap();
		}

		// Fix up reference in parent
		if let Some(parent) = self.try_get_cmp::<Parent>().map(|x| x.0) {
			if WORLD.is_dead(parent) { log::warn!("replace_with parent dead {:?}", parent); return other; }
			let mut children = parent.get_cmp_mut::<Children>();
			let position = children.0.iter().position(|&x| x == self.as_entity()).expect("entity claims to be a child while missing in parent");
			children.0[position] = other.as_entity();
			other_entity.get_cmp_mut_or_default::<Parent>().0 = parent;
		}

		WORLD.remove_entity(self);
		other
	}

	fn with(self, f: impl FnOnce(&Self)) -> Self { f(&self); self }
	fn erase(&self) -> SomeElement { SomeElement(self.as_entity()) }
}

impl<T: Element> Element for &T { }
impl<T: Element> Element for &mut T { }
