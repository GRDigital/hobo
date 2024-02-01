use crate::prelude::*;
use futures_signals::signal::{Signal, SignalExt};
pub use hobo_derive::AsElement;
use std::{
	any::TypeId,
	borrow::Cow,
	collections::{HashMap, HashSet},
};

/// An `Element` with specific type erased
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, AsElement)]
pub struct Element(pub Entity);

#[derive(Default, Debug)]
pub(crate) struct Classes {
	pub(crate) marks: HashSet<TypeId>,

	/// A HashMap of:
	///
	/// * key:     `u64`        - Tag hash.
	/// * value.0: `css::Style` - The style of the class.
	/// * value.1: `usize`      - Ordinal number.
	///
	/// For example, if `.class` was called 4 times on an element, the ordinal number of the last class would be 3 (the index).
	/// This is used for precedence.
	pub(crate) styles: HashMap<u64, (css::Style, usize)>,
}

#[cfg(feature = "experimental")]
pub struct InDom;

#[cfg(feature = "experimental")]
impl InDom {
	fn infect(element: Element) {
		if element.is_dead() { log::warn!("InDom::infect dead {:?}", element.as_entity()); return; }
		element.add_component(InDom);

		let callbacks = element.try_get_cmp_mut::<OnDomAttachCbs>().map(|mut x| std::mem::take(&mut x.0));
		if let Some(callbacks) = callbacks {
			element.remove_cmp::<OnDomAttachCbs>();
			for cb in callbacks { cb(); }
		}

		let children = element.try_get_cmp::<Children>().map(|x| x.0.clone());
		if let Some(children) = children {
			for child in children { InDom::infect(Element(child)); }
		}
	}
}

#[cfg(feature = "experimental")]
#[derive(Default)]
struct OnDomAttachCbs(Vec<Box<dyn FnOnce() + Send + Sync + 'static>>);

#[cfg(feature = "experimental")]
impl OnDomAttachCbs {
	fn handle_parenting(parent: Element, child: Element) {
		if !child.has_cmp::<InDom>() && parent.has_cmp::<InDom>() {
			InDom::infect(child);
		}
	}
}

#[derive(Default)]
struct SignalHandlesCollection(Vec<discard::DiscardOnDrop<futures_signals::CancelableFutureHandle>>);

// NOTE: This is so we can clear child signals on Children::clear
// Kind of a hack, ideally we should be able to just steal components instead of replacing the entire entity
// on replace_with. But handlers would be invalidated. But maybe that's ok? One of life's mysteries.
#[derive(Default)]
pub(crate) struct ChildSignalHandlesCollection(pub(crate) Vec<discard::DiscardOnDrop<futures_signals::CancelableFutureHandle>>);

#[cfg(debug_assertions)]
pub(crate) struct OrphanComplainer(i32, Closure<dyn Fn()>);

#[cfg(debug_assertions)]
impl OrphanComplainer {
	pub fn new(entity: Entity) -> Self {
		let f = Closure::wrap(Box::new(move || {
			// taken from console_error_panic_hook
			// unfortunately, we can't build PanicInfo to use their hook soooo just ctrlc ctrlv time
			#[wasm_bindgen]
			extern {
				type Error;
				#[wasm_bindgen(constructor)] fn new() -> Error;
				#[wasm_bindgen(structural, method, getter)] fn stack(error: &Error) -> String;
			}

			// we can't get location here because location is set in .add_child
			log::warn!("[OrphanComplainer] Element {} wasn't parented in 1 sec, it's probably a bug\n\nStack:\n\n{}", entity.0, Error::new().stack());
		}) as Box<dyn Fn()>);
		let id = web_sys::window().unwrap().set_interval_with_callback_and_timeout_and_arguments_0(f.as_ref().unchecked_ref(), 1000).unwrap();

		OrphanComplainer(id, f)
	}
}

#[cfg(debug_assertions)]
impl Drop for OrphanComplainer {
	fn drop(&mut self) {
		web_sys::window().unwrap().clear_interval_with_handle(self.0);
	}
}

impl Element {
	#[track_caller]
	fn add_child(self, child: Element) {
		if self.is_dead() { log::warn!("add_child parent dead {:?}", self.as_entity()); return; }
		if child.is_dead() { log::warn!("add_child child dead {:?}", child.as_entity()); return; }

		self.get_cmp_mut_or_default::<Children>().push(child.as_entity());
		child.get_cmp_mut_or_default::<Parent>().0 = self.as_entity();

		if let (Some(parent_node), Some(child_node)) = (self.try_get_cmp::<web_sys::Node>(), child.try_get_cmp::<web_sys::Node>()) {
			parent_node.append_child(&child_node).expect("can't append child");
		} else {
			let parent_has = if self.has_cmp::<web_sys::Node>() { "has" } else { "doesn't have" };
			let child_has = if child.has_cmp::<web_sys::Node>() { "has" } else { "doesn't have" };
			log::warn!("trying to add_child, but child {child_has} web_sys::Node and parent {parent_has} web_sys::Node");
		}

		#[cfg(debug_assertions)] {
			let caller = std::panic::Location::caller();
			child.set_attr("data-location", &format!("{}:{}", caller.file(), caller.line()));
		}

		#[cfg(feature = "experimental")]
		OnDomAttachCbs::handle_parenting(self, child);

		#[cfg(debug_assertions)]
		child.remove_cmp::<OrphanComplainer>();
	}

	fn leave_parent(self) {
		if self.is_dead() { log::warn!("leave_parent child dead {:?}", self.as_entity()); return; }
		let parent = self.get_cmp::<Parent>().0;
		if parent.is_dead() { log::warn!("leave_parent parent dead {:?}", self.as_entity()); return; }

		if let (Some(parent_node), Some(child_node)) = (parent.try_get_cmp::<web_sys::Node>(), self.try_get_cmp::<web_sys::Node>()) {
			parent_node.remove_child(&child_node).expect("can't remove child");
		}

		self.remove_cmp::<Parent>();
		let mut siblings = parent.get_cmp_mut::<Children>();
		if let Some(child_pos) = siblings.0.iter().position(|&x| x == self.as_entity()) {
			siblings.0.remove(child_pos);
		}
	}

	#[track_caller]
	fn add_child_at(self, at_index: usize, child: Element) {
		if self.is_dead() { log::warn!("add_child_at parent dead {:?}", self.as_entity()); return; }
		if child.is_dead() { log::warn!("add_child_at child dead {:?}", child.as_entity()); return; }

		let mut children = self.get_cmp_mut_or_default::<Children>();
		let shifted_sibling = children.get(at_index).copied();
		children.insert(at_index, child.as_entity());
		child.get_cmp_mut_or_default::<Parent>().0 = self.as_entity();

		if let (Some(parent_node), Some(child_node), shifted_sibling_node) = (
			self.try_get_cmp::<web_sys::Node>(),
			child.try_get_cmp::<web_sys::Node>(),
			shifted_sibling.and_then(|x| x.try_get_cmp::<web_sys::Node>()),
		) {
			parent_node
				.insert_before(&child_node, shifted_sibling_node.as_ref().map(|x| &**x as &web_sys::Node))
				.expect("can't append child");


			#[cfg(debug_assertions)] {
				let caller = std::panic::Location::caller();
				child.set_attr("data-location", &format!("{}:{}", caller.file(), caller.line()));
			}
		}
	}

	// this track_caller doesn't work exactly how I'd want, the `data-location` attr for the child is set to the `.replace_with` line
	#[track_caller]
	fn add_child_signal<S>(self, signal: S) where
		S: Signal<Item = Element> + 'static,
	{
		// placeholder at first
		let mut child = crate::create::div().class(crate::css::display::none).as_element();
		self.add_child(child);
		let (handle, fut) = futures_signals::cancelable_future(signal.for_each(move |new_child| {
			let new_child = new_child.as_element();
			child.replace_with(new_child);
			child = new_child;
			std::future::ready(())
		}), Default::default);

		wasm_bindgen_futures::spawn_local(fut);
		self.get_cmp_mut_or_default::<ChildSignalHandlesCollection>().0.push(handle);
	}

	#[track_caller]
	fn replace_with(self, other: Element) {
		if self.is_dead() { log::warn!("replace_with dead {:?}", self.as_entity()); return; }

		if let (Some(this), Some(other)) = (self.try_get_cmp::<web_sys::Element>(), other.try_get_cmp::<web_sys::Node>()) {
			this.replace_with_with_node_1(&other).unwrap();
		} else {
			let self_has = if self.has_cmp::<web_sys::Node>() { "has" } else { "doesn't have" };
			let other_has = if other.has_cmp::<web_sys::Node>() { "has" } else { "doesn't have" };
			log::warn!("trying to replace_with, but self {self_has} web_sys::Node and other {other_has} web_sys::Node");
		}

		#[cfg(debug_assertions)] {
			let caller = std::panic::Location::caller();
			other.set_attr("data-location", &format!("{}:{}", caller.file(), caller.line()));
		}

		// Fix up reference in parent
		if let Some(parent) = self.try_get_cmp::<Parent>().map(|x| x.0) {
			if parent.is_dead() { log::warn!("replace_with parent dead {:?}", parent); return; }
			{
				let mut children = parent.get_cmp_mut::<Children>();
				let position = children.0.iter().position(|&x| x == self.as_entity()).expect("entity claims to be a child while missing in parent");
				children.0[position] = other.as_entity();
				other.get_cmp_mut_or_default::<Parent>().0 = parent;
			}

			#[cfg(debug_assertions)]
			other.remove_cmp::<OrphanComplainer>();

			#[cfg(feature = "experimental")]
			OnDomAttachCbs::handle_parenting(Element(parent), other);
		}

		self.remove();
	}

	#[track_caller]
	fn allow_no_parent(self) -> Self {
		#[cfg(debug_assertions)] self.remove_cmp::<OrphanComplainer>();
		self
	}
}

/// Marker trait for an entity that has `web_sys::Node`, `web_sys::Element`, `web_sys::EventTarget` and one of `web_sys::HtmlElement` or `web_sys::SvgElement` as attached components
pub trait AsElement: AsEntity + Sized {
	#[cfg(feature = "experimental")]
	const MARK: Option<fn() -> std::any::TypeId> = None;

	#[cfg(all(debug_assertions, feature = "experimental"))]
	const TYPE: Option<fn() -> &'static str> = None;

	#[track_caller]
	fn add_child<T: AsElement>(&self, child: T) {
		#[cfg(feature = "experimental")]
		if let Some(mark) = T::MARK { child.get_cmp_mut_or_default::<Classes>().marks.insert(mark()); }

		#[cfg(all(debug_assertions, feature = "experimental"))]
		if let Some(type_id) = T::TYPE { child.set_attr("data-type", type_id()); }

		Element::add_child(self.as_element(), child.as_element());
	}
	#[track_caller] #[must_use] fn child(self, child: impl AsElement) -> Self { self.add_child(child); self }
	#[track_caller] #[must_use] fn with_child<T: AsElement>(self, f: impl FnOnce(&Self) -> T) -> Self { let c = f(&self); self.child(c) }
	#[track_caller] fn add_children<Item: AsElement>(&self, children: impl IntoIterator<Item = Item>) { for child in children.into_iter() { self.add_child(child); } }
	#[track_caller] #[must_use] fn children<Item: AsElement>(self, children: impl IntoIterator<Item = Item>) -> Self { self.add_children(children); self }
	fn leave_parent(self) { Element::leave_parent(self.as_element()) }

	/// add a child at an index, useful to update tables without regenerating the whole container element
	#[track_caller]
	fn add_child_at<T: AsElement>(&self, at_index: usize, child: T) {
		#[cfg(feature = "experimental")]
		if let Some(mark) = T::MARK { child.get_cmp_mut_or_default::<Classes>().marks.insert(mark()); }

		#[cfg(all(debug_assertions, feature = "experimental"))]
		if let Some(type_id) = T::TYPE { child.set_attr("data-type", type_id()); }

		Element::add_child_at(self.as_element(), at_index, child.as_element());
	}

	// be mindful about holding child references with this one
	#[track_caller]
	fn add_child_signal<S, E>(&self, signal: S) where
		E: AsElement,
		S: Signal<Item = E> + 'static,
	{
		Element::add_child_signal(self.as_element(), signal.map(|x| {
			#[cfg(feature = "experimental")]
			if let Some(mark) = E::MARK { x.get_cmp_mut_or_default::<Classes>().marks.insert(mark()); }

			#[cfg(all(debug_assertions, feature = "experimental"))]
			if let Some(type_id) = E::TYPE { x.set_attr("data-type", type_id()); }

			x.as_element()
		}));
	}
	#[track_caller]
	#[must_use]
	fn child_signal<S, E>(self, signal: S) -> Self where
		E: AsElement,
		S: Signal<Item = E> + 'static,
	{ self.add_child_signal(signal); self }

	fn set_class_tagged<Tag: std::hash::Hash + 'static>(&self, tag: Tag, style: impl Into<css::Style>) {
		if self.is_dead() { log::warn!("set_class_tagged dead {:?}", self.as_entity()); return; }

		// tested and different types with same byte-level representation hash to the same thing (not surprising)
		// i.e. the type is not taken into account when hashing so I have to do it manually
		let tag_hash = {
			use std::hash::{Hash, Hasher};
			let mut hasher = std::collections::hash_map::DefaultHasher::new();
			TypeId::of::<Tag>().hash(&mut hasher);
			tag.hash(&mut hasher);
			hasher.finish()
		};

		let mut classes = self.get_cmp_mut_or_default::<Classes>();
		let len = classes.styles.len();
		classes.styles.insert(tag_hash, (style.into(), len));
	}
	fn set_class_typed<Type: 'static>(&self, style: impl Into<css::Style>) { self.set_class_tagged(TypeId::of::<Type>(), style) }
	fn set_class(&self, style: impl Into<css::Style>) { self.set_class_tagged(0u64, style); }
	fn add_class(&self, style: impl Into<css::Style>) {
		let id = self.try_get_cmp::<Classes>().map(|x| x.styles.len() as u64).unwrap_or(0);
		self.set_class_tagged(id, style);
	}
	#[must_use] fn class(self, style: impl Into<css::Style>) -> Self { self.add_class(style); self }
	#[must_use] fn class_tagged<Tag: std::hash::Hash + 'static>(self, tag: Tag, style: impl Into<css::Style>) -> Self { self.set_class_tagged(tag, style); self }
	#[must_use] fn class_typed<Type: 'static>(self, style: impl Into<css::Style>) -> Self { self.set_class_typed::<Type>(style); self }

	fn set_class_signal<S, I>(&self, signal: S) where
		I: Into<css::Style>,
		S: Signal<Item = I> + 'static,
	{
		let entity = self.as_entity();
		if entity.is_dead() { log::warn!("set_class_signal dead entity {:?}", entity); return; }
		let (handle, fut) = futures_signals::cancelable_future(signal.for_each(move |class| {
			Element(entity).set_class(class);
			std::future::ready(())
		}), Default::default);

		wasm_bindgen_futures::spawn_local(fut);
		self.get_cmp_mut_or_default::<SignalHandlesCollection>().0.push(handle);
	}
	#[must_use]
	fn class_signal<S, I>(self, signal: S) -> Self where
		I: Into<css::Style>,
		S: Signal<Item = I> + 'static,
	{ self.set_class_signal(signal); self }

	fn set_class_typed_signal<Type, S, I>(&self, signal: S) where
		Type: 'static,
		I: Into<css::Style>,
		S: Signal<Item = I> + 'static,
	{
		let entity = self.as_entity();
		if entity.is_dead() { log::warn!("set_class_signal dead entity {:?}", entity); return; }
		let (handle, fut) = futures_signals::cancelable_future(signal.for_each(move |class| {
			Element(entity).set_class_typed::<Type>(class.into());
			std::future::ready(())
		}), Default::default);

		wasm_bindgen_futures::spawn_local(fut);
		self.get_cmp_mut_or_default::<SignalHandlesCollection>().0.push(handle);
	}
	#[must_use]
	fn class_typed_signal<Type, S, I>(self, signal: S) -> Self where
		Type: 'static,
		I: Into<css::Style>,
		S: Signal<Item = I> + 'static,
	{ self.set_class_typed_signal::<Type, S, I>(signal); self }

	fn set_class_tagged_signal<Tag, S, I>(&self, tag: Tag, signal: S) where
		Tag: std::hash::Hash + Copy + 'static,
		I: Into<css::Style>,
		S: Signal<Item = I> + 'static,
	{
		let entity = self.as_entity();
		if entity.is_dead() { log::warn!("set_class_signal dead entity {:?}", entity); return; }
		let (handle, fut) = futures_signals::cancelable_future(signal.for_each(move |class| {
			Element(entity).set_class_tagged(tag, class);
			std::future::ready(())
		}), Default::default);

		wasm_bindgen_futures::spawn_local(fut);
		self.get_cmp_mut_or_default::<SignalHandlesCollection>().0.push(handle);
	}
	#[must_use]
	fn class_tagged_signal<Tag, S, I>(self, tag: Tag, signal: S) -> Self where
		Tag: std::hash::Hash + Copy + 'static,
		I: Into<css::Style>,
		S: Signal<Item = I> + 'static,
	{ self.set_class_tagged_signal::<Tag, S, I>(tag, signal); self }

	fn get_attr<'k>(&self, key: impl Into<Cow<'k, str>>) -> Option<String> {
		if self.is_dead() { log::warn!("get_attr dead {:?}", self.as_entity()); return None; }
		let key = key.into();
		self.get_cmp::<web_sys::Element>().get_attribute(&key)
	}
	fn set_attr<'k, 'v>(&self, key: impl Into<Cow<'k, str>>, value: impl Into<Cow<'v, str>>) {
		if self.is_dead() { log::warn!("set_attr dead {:?}", self.as_entity()); return; }
		let key = key.into();
		let value = value.into();
		self.get_cmp::<web_sys::Element>().set_attribute(&key, &value).unwrap_or_else(|_| panic!("can't set attribute {} to {}", key, value));
	}
	#[must_use] fn attr<'k, 'v>(self, key: impl Into<Cow<'k, str>>, value: impl Into<Cow<'v, str>>) -> Self { self.set_attr(key, value); self }
	fn set_bool_attr<'k>(&self, key: impl Into<Cow<'k, str>>, value: bool) { if value { self.set_attr(key, "") } else { self.remove_attr(key) } }
	#[must_use] fn bool_attr<'k>(self, key: impl Into<Cow<'k, str>>, value: bool) -> Self { self.set_bool_attr(key, value); self }
	fn remove_attr<'k>(&self, key: impl Into<Cow<'k, str>>) {
		if self.is_dead() { log::warn!("remove_attr dead {:?}", self.as_entity()); return; }
		self.get_cmp::<web_sys::Element>().remove_attribute(&key.into()).expect("can't remove attribute");
	}

	fn set_attr_signal<'k, 'v, S, K, V>(&self, attr: K, signal: S) where
		K: Into<Cow<'k, str>>,
		V: Into<Cow<'v, str>>,
		S: Signal<Item = V> + 'static,
	{
		let entity = self.as_entity();
		if entity.is_dead() { log::warn!("set_attr_signal dead entity {:?}", entity); return; }
		let attr = attr.into().into_owned();
		let (handle, fut) = futures_signals::cancelable_future(signal.for_each(move |v| {
			Element(entity).set_attr(&attr, v);
			std::future::ready(())
		}), Default::default);

		wasm_bindgen_futures::spawn_local(fut);
		self.get_cmp_mut_or_default::<SignalHandlesCollection>().0.push(handle);
	}
	#[must_use]
	fn attr_signal<'k, 'v, S, K, V>(self, attr: K, signal: S) -> Self where
		K: Into<Cow<'k, str>>,
		V: Into<Cow<'v, str>>,
		S: Signal<Item = V> + 'static,
	{ self.set_attr_signal(attr, signal); self }

	fn set_bool_attr_signal<'k, S, K>(&self, attr: K, signal: S) where
		K: Into<Cow<'k, str>>,
		S: Signal<Item = bool> + 'static,
	{
		let entity = self.as_entity();
		if entity.is_dead() { log::warn!("set_attr_signal dead entity {:?}", entity); return; }
		let attr = attr.into().into_owned();
		let (handle, fut) = futures_signals::cancelable_future(signal.for_each(move |v| {
			Element(entity).set_bool_attr(&attr, v);
			std::future::ready(())
		}), Default::default);

		wasm_bindgen_futures::spawn_local(fut);
		self.get_cmp_mut_or_default::<SignalHandlesCollection>().0.push(handle);
	}
	#[must_use]
	fn bool_attr_signal<'k, S, K>(self, attr: K, signal: S) -> Self where
		K: Into<Cow<'k, str>>,
		S: Signal<Item = bool> + 'static,
	{ self.set_bool_attr_signal(attr, signal); self }

	fn set_text<'a>(&self, text: impl Into<std::borrow::Cow<'a, str>>) {
		if self.is_dead() { log::warn!("set_text dead entity {:?}", self.as_entity()); return; }
		self.get_cmp::<web_sys::Node>().set_text_content(Some(&text.into()));
	}
	#[must_use] fn text<'a>(self, x: impl Into<std::borrow::Cow<'a, str>>) -> Self { self.set_text(x); self }

	fn set_text_signal<'a, S, I>(&self, signal: S) where
		I: Into<Cow<'a, str>>,
		S: Signal<Item = I> + 'static,
	{
		let entity = self.as_entity();
		if entity.is_dead() { log::warn!("set_text_signal dead entity {:?}", entity); return; }
		let (handle, fut) = futures_signals::cancelable_future(signal.for_each(move |text| {
			Element(entity).set_text(text);
			std::future::ready(())
		}), Default::default);

		wasm_bindgen_futures::spawn_local(fut);
		self.get_cmp_mut_or_default::<SignalHandlesCollection>().0.push(handle);
	}
	#[must_use]
	fn text_signal<'a, S, I>(self, x: S) -> Self where
		I: Into<Cow<'a, str>>,
		S: Signal<Item = I> + 'static,
	{ self.set_text_signal(x); self }

	fn set_style(&self, style: impl AppendProperty) {
		let mut props = Vec::new();
		style.append_property(&mut props);
		self.set_attr(web_str::style(), props.iter().map(std::string::ToString::to_string).collect::<String>());
	}
	#[must_use] fn style(self, style: impl AppendProperty) -> Self { self.set_style(style); self }
	fn remove_style(&self) { self.remove_attr(web_str::style()); }

	fn set_style_signal<S, I>(&self, signal: S) where
		I: AppendProperty,
		S: Signal<Item = I> + 'static,
	{
		let entity = self.as_entity();
		if entity.is_dead() { log::warn!("set_style_signal dead entity {:?}", entity); return; }
		let (handle, fut) = futures_signals::cancelable_future(signal.for_each(move |style| {
			Element(entity).set_style(style);
			std::future::ready(())
		}), Default::default);

		wasm_bindgen_futures::spawn_local(fut);
		self.get_cmp_mut_or_default::<SignalHandlesCollection>().0.push(handle);
	}
	#[must_use]
	fn style_signal<S, I>(self, signal: S) -> Self where
		I: AppendProperty,
		S: Signal<Item = I> + 'static,
	{ self.set_style_signal(signal); self }

	fn mark<T: 'static>(self) -> Self {
		if self.is_dead() { log::warn!("mark dead {:?}", self.as_entity()); return self; }
		self.get_cmp_mut_or_default::<Classes>().marks.insert(TypeId::of::<T>());
		self
	}
	fn unmark<T: 'static>(self) -> Self {
		if self.is_dead() { log::warn!("unmark dead {:?}", self.as_entity()); return self; }
		self.get_cmp_mut_or_default::<Classes>().marks.remove(&TypeId::of::<T>());
		self
	}
	#[must_use]
	fn mark_signal<T: 'static, S>(self, signal: S) -> Self where
		S: Signal<Item = bool> + 'static,
	{
		let entity = self.as_entity();
		if entity.is_dead() { log::warn!("mark_signal dead entity {:?}", entity); return self; }
		let (handle, fut) = futures_signals::cancelable_future(signal.for_each(move |enabled| {
			if enabled { Element(entity).mark::<T>(); } else { Element(entity).unmark::<T>(); }
			std::future::ready(())
		}), Default::default);

		wasm_bindgen_futures::spawn_local(fut);
		self.get_cmp_mut_or_default::<SignalHandlesCollection>().0.push(handle);
		self
	}

	#[must_use] fn with_component<T: 'static>(self, f: impl FnOnce(&Self) -> T) -> Self { self.add_component(f(&self)); self }

	// can't steal components because handlers would get invalidated
	#[track_caller]
	fn replace_with<T: AsElement>(&self, other: T) -> T {
		#[cfg(feature = "experimental")]
		if let Some(mark) = T::MARK { other.get_cmp_mut_or_default::<Classes>().marks.insert(mark()); }

		#[cfg(all(debug_assertions, feature = "experimental"))]
		if let Some(type_id) = T::TYPE { other.set_attr("data-type", type_id()); }

		Element::replace_with(self.as_element(), other.as_element());
		other
	}

	fn parent(&self) -> Element {
		let parent = self.get_cmp::<Parent>().0;
		debug_assert!(parent.try_get_cmp::<web_sys::HtmlElement>().is_some());
		Element(parent)
	}

	#[cfg(feature = "experimental")]
	fn add_on_dom_attach(&self, cb: impl FnOnce() + Send + Sync + 'static) {
		if self.has_cmp::<InDom>() { cb(); return; }
		self.get_cmp_mut_or_default::<OnDomAttachCbs>().0.push(Box::new(cb));
	}
	#[cfg(feature = "experimental")]
	fn on_dom_attach(self, cb: impl FnOnce() + Send + Sync + 'static) -> Self { self.add_on_dom_attach(cb); self }

	#[deprecated = "use .tap() instead"]
	fn with(self, f: impl FnOnce(&Self)) -> Self { f(&self); self }
	fn as_element(&self) -> Element { Element(self.as_entity()) }

	fn allow_no_parent(self) -> Self { Element::allow_no_parent(self.as_element()); self }
}

impl<T: AsElement> AsElement for &T {}
impl<T: AsElement> AsElement for &mut T {}
