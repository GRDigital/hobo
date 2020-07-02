use crate::prelude::*;
use std::{
	borrow::Cow,
	cell::RefCell,
	collections::HashMap,
};
pub use sugars::hash;

#[derive(Default)]
pub struct StyleStorage {
	map: RefCell<HashMap<css::AtRules, u64>>,
}

// TODO: right now if the same style is reused in multiple windows - won't work, need to track style insertion per window
// it checks if the style is already inserted as css into <style>
// if yes, just returns the class name
// if no, inserts it into <style> and then returns the class name
impl StyleStorage {
	pub fn fetch<'a>(&self, element: &web_sys::Element, at_rules: impl Into<Cow<'a, css::AtRules>>) -> String {
		let mut at_rules = at_rules.into().into_owned();

		// check if style exists in cache, in which case it's already inserted - just retrieve clas name
		if let Some(id) = self.map.borrow().get(&at_rules) {
			return format!("s{}", id);
		}

		// just getting the u64 hash from rules
		let id = hash!(at_rules);

		// caching the id
		self.map.borrow_mut().insert(at_rules.clone(), id);
		let class = format!("s{}", id);

		// replace the ClassPlaceholder with actual element class
		for rule in at_rules.0.iter_mut() {
			for style_rule in rule.style.0.iter_mut() {
				for selector_component in (style_rule.0).0.iter_mut() {
					if *selector_component == css::selector::SelectorComponent::ClassPlaceholder {
						*selector_component = css::selector::SelectorComponent::Class(class.clone());
					}
				}
			}
		}

		let dom = element.owner_document().expect("element not attached to a dom");
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
		style_element.append_with_str_1(&at_rules.to_string()).expect("can't append css string");
		class
	}
}
