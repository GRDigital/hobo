use crate::prelude::*;
use std::{cell::RefCell, collections::HashMap};
pub use sugars::hash;

#[derive(Default)]
pub struct StyleStorage {
	map: RefCell<HashMap<css::Style, u64>>,
}

// replace the ClassPlaceholder with actual element class
fn fixup_class_placeholders(style: &mut css::Style, class: String) {
	for rule in style.0.iter_mut() {
		match rule {
			css::Rule::Style(style_rule) => {
				for selector_component in (style_rule.0).0.iter_mut() {
					if *selector_component == css::selector::SelectorComponent::ClassPlaceholder {
						*selector_component = css::selector::SelectorComponent::Class(class.clone());
					}
				}
			},
			css::Rule::Media(_, style) => fixup_class_placeholders(style, class.clone()),
			_ => {},
		}
	}
}

// TODO: right now if the same style is reused in multiple windows - won't work, need to track style insertion per window
// it checks if the style is already inserted as css into <style>
// if yes, just returns the class name
// if no, inserts it into <style> and then returns the class name
impl StyleStorage {
	pub fn fetch(&self, mut style: css::Style) -> String {
		// check if style exists in cache, in which case it's already inserted - just retrieve clas name
		if let Some(id) = self.map.borrow().get(&style) {
			return format!("s{}", id);
		}

		// just getting the u64 hash from style
		let id = hash!(style);

		// caching the id
		self.map.borrow_mut().insert(style.clone(), id);
		let class = format!("s{}", id);

		fixup_class_placeholders(&mut style, class.clone());

		let dom = crate::dom();
		let head = dom.head().expect("dom has no head");

		// either get or construct a <style> element
		let style_element = if let Some(x) = head.get_elements_by_tag_name("style").get_with_index(0) {
			x
		} else {
			let element = dom.create_element(web_str::style()).expect("can't create style element");
			head.append_child(&element).expect("can't append child");
			element
		};

		// insert the stringified generated css into the style tag
		style_element.append_with_str_1(&style.to_string()).expect("can't append css string");
		class
	}
}
