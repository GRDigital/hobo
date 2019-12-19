use wasm_bindgen::JsCast as _;
use super::Element;

pub struct SvgElement {
	element: web_sys::Element,
	children: Vec<SvgElement>,
}

impl Element for SvgElement {
	fn element(&self) -> &web_sys::Element {
		&self.element
	}
}

impl Drop for SvgElement {
	fn drop(&mut self) {
		self.element.remove();
	}
}

impl Clone for SvgElement {
	fn clone(&self) -> Self {
		Self {
			element: self.element.clone_node_with_deep(true).unwrap().dyn_into().unwrap(),
			children: self.children.clone(),
		}
	}
}

impl<'a> From<roxmltree::Document<'a>> for SvgElement {
	fn from(doc: roxmltree::Document) -> Self {
		doc.root_element().into()
	}
}

impl<'a, 'b> From<roxmltree::Node<'a, 'b>> for SvgElement {
	fn from(node: roxmltree::Node) -> Self {
		let dom = web_sys::window().unwrap().document().unwrap();
		let element = dom.create_element_ns(Some("http://www.w3.org/2000/svg"), node.tag_name().name()).unwrap();
		for attribute in node.attributes() {
			element.set_attribute(attribute.name(), attribute.value()).unwrap();
		}
		let children = node.children().filter_map(|child| -> Option<SvgElement> {
			match child.node_type() {
				roxmltree::NodeType::Element => Some(child.into()),
				_ => None,
			}
		}).collect::<Vec<_>>();
		let me = Self { children, element };
		for child in me.children.iter() {
			me.append(child);
		}
		me
	}
}
