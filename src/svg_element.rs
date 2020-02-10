use super::Element;

impl<'a> From<roxmltree::Document<'a>> for crate::BasicElement<web_sys::SvgElement> {
	fn from(doc: roxmltree::Document) -> Self { doc.root_element().into() }
}

impl<'a, 'b> From<roxmltree::Node<'a, 'b>> for crate::BasicElement<web_sys::SvgElement> {
	fn from(node: roxmltree::Node) -> Self {
		let dom = crate::dom();
		let element = web_sys::SvgElement::from(wasm_bindgen::JsValue::from(dom.create_element_ns(Some("http://www.w3.org/2000/svg"), node.tag_name().name()).unwrap()));
		for attribute in node.attributes() {
			element.set_attribute(attribute.name(), attribute.value()).unwrap();
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
		let me = Self { children, element, event_handlers: crate::EventHandlers::default() };
		for child in me.children.iter() {
			me.append(child.as_ref());
		}
		me
	}
}
