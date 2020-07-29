use super::Element;

impl<'a> From<roxmltree::Document<'a>> for crate::BasicElement<web_sys::SvgsvgElement> {
	fn from(doc: roxmltree::Document) -> Self {
		let node = doc.root_element();
		let element = crate::create::svg();

		for attribute in node.attributes() {
			element.set_attribute(attribute.name(), attribute.value()).expect("can't set attribute on an svg element");
		}

		let children = node
			.children()
			.filter_map(|child| -> Option<Box<dyn crate::Element>> {
				match child.node_type() {
					roxmltree::NodeType::Element => Some(Box::new(crate::BasicElement::<web_sys::SvgElement>::from(child))),
					_ => None,
				}
			})
			.collect::<Vec<_>>();

		let me = Self { children, element, event_handlers: crate::EventHandlers::default(), classes: Default::default() };
		for child in me.children.iter() {
			me.element.append_child(&child.element()).expect("can't append child");
		}
		me
	}
}

impl<'a, 'b> From<roxmltree::Node<'a, 'b>> for crate::BasicElement<web_sys::SvgElement> {
	fn from(node: roxmltree::Node) -> Self {
		let element: web_sys::SvgElement = wasm_bindgen::JsCast::unchecked_into(crate::dom().create_element_ns(Some(wasm_bindgen::intern("http://www.w3.org/2000/svg")), node.tag_name().name()).expect("can't create svg element"));

		for attribute in node.attributes() {
			element.set_attribute(attribute.name(), attribute.value()).expect("can't set attribute on an svg element");
		}

		let children = node
			.children()
			.filter_map(|child| -> Option<Box<dyn crate::Element>> {
				match child.node_type() {
					roxmltree::NodeType::Element => Some(Box::new(Self::from(child))),
					_ => None,
				}
			})
			.collect::<Vec<_>>();

		let me = Self { children, element, event_handlers: crate::EventHandlers::default(), classes: Default::default() };
		for child in me.children.iter() {
			me.element.append_child(&child.element()).expect("can't append child");
		}
		me
	}
}
