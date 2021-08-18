use crate::prelude::*;
use std::collections::HashMap;
pub use sugars::hash;
use std::cell::RefCell;

#[derive(Default)]
pub struct StyleStorage {
	map: HashMap<css::Style, u64>,
	style_elements: Vec<web_sys::Element>,
}

thread_local! {
	pub static STYLE_STORAGE: RefCell<StyleStorage> = RefCell::new(StyleStorage {
		map: HashMap::new(),
		style_elements: vec![{
			let dom = crate::dom();
			let head = dom.head().expect("dom has no head");
			let element = dom.create_element(web_str::style()).expect("can't create style element");
			head.append_child(&element).expect("can't append child");
			element
		}],
	});
}

#[extend::ext]
impl css::Style {
	// replace the ClassPlaceholder with actual element class
	fn fixup_class_placeholders(&mut self, class: String) {
		for rule in self.0.iter_mut() {
			match rule {
				css::Rule::Style(style_rule) => {
					for selector_component in (style_rule.0).0.iter_mut() {
						if *selector_component == css::selector::SelectorComponent::ClassPlaceholder {
							*selector_component = css::selector::SelectorComponent::Class(class.clone());
						}
					}
				},
				css::Rule::Media(_, style) => style.fixup_class_placeholders(class.clone()),
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
		// check if style exists in cache, in which case it's already inserted - just retrieve class name
		if let Some(id) = self.map.get(&style) {
			return format!("s{}", id);
		}

		// just getting the u64 hash from style
		let id = hash!(style);

		// caching the id
		self.map.insert(style.clone(), id);
		let class = format!("s{}", id);

		style.fixup_class_placeholders(class.clone());

		for style_element in &self.style_elements {
			// insert the stringified generated css into the style tag
			style_element.append_with_str_1(&style.to_string()).expect("can't append css string");
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
