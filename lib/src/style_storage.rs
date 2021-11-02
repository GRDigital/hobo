use crate::{prelude::*, RacyCell};
use once_cell::sync::Lazy;
use std::collections::HashSet;
pub use sugars::hash;

#[derive(Default)]
pub struct StyleStorage {
	inserted_style_hashes: HashSet<u64>,
	// list of <style> elements, prehaps in different windows
	style_elements: Vec<web_sys::Element>,
}

pub(crate) static STYLE_STORAGE: Lazy<RacyCell<StyleStorage>> = Lazy::new(|| RacyCell::new(StyleStorage {
	inserted_style_hashes: HashSet::new(),
	style_elements: vec![{
		let dom = web_sys::window().expect("no window").document().expect("no document");
		let head = dom.head().expect("dom has no head");
		let element = dom.create_element(web_str::style()).expect("can't create style element");
		head.append_child(&element).expect("can't append child");
		element
	}],
}));

#[extend::ext]
impl css::Style {
	// replace the ClassPlaceholder with actual element class
	fn fixup_class_placeholders(&mut self, class: &str) {
		for rule in self.0.iter_mut() {
			match rule {
				css::Rule::Style(style_rule) => {
					for selector_component in (style_rule.0).0.iter_mut() {
						if *selector_component == css::selector::SelectorComponent::ClassPlaceholder {
							*selector_component = css::selector::SelectorComponent::Class(class.to_owned());
						}
					}
				},
				css::Rule::Media(_, style) => style.fixup_class_placeholders(class),
				_ => {},
			}
		}
	}

	fn sort_properties(&mut self) {
		for rule in self.0.iter_mut() {
			match rule {
				css::Rule::Style(style_rule) => {
					style_rule.1.sort();
				},
				css::Rule::Media(_, style) => style.sort_properties(),
				_ => {},
			}
		}
	}
}

// it checks if the style is already inserted as css into <style>
// if yes, just returns the class name
// if no, inserts it into <style> and then returns the class name
impl StyleStorage {
	pub fn fetch(&mut self, mut style: css::Style) -> String {
		// if stable sort used on properties before hashing, then order of declarations would be preserved
		// but different elements that use the same properties in a different order would still reuse the same class
		// in other words, if you're specifying the same property multiple times to override it - that should still work
		// but the order of properties should no longer influence the hash result
		// style.sort_properties();

		// u64 hash from style
		let id = hash!(style);

		// recover class name
		let class = format!("s-{:x}", id);

		// check if style exists in cache, in which case it's already inserted - just return class name
		if self.inserted_style_hashes.contains(&id) { return class; }

		// caching the style id
		self.inserted_style_hashes.insert(id);

		style.fixup_class_placeholders(&class);

		let style_string = style.to_string();
		for style_element in &self.style_elements {
			// insert the stringified generated css into the style tag
			style_element.append_with_str_1(&style_string).expect("can't append css string");
		}

		class
	}

	pub fn register_window(&mut self, window: &web_sys::Window) {
		let dom = window.document().expect("window has no dom");
		let head = dom.head().expect("dom has no head");
		let element = dom.create_element(web_str::style()).expect("can't create style element");
		head.append_child(&element).expect("can't append child");

		self.style_elements.push(element);
	}
}
