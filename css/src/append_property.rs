use crate::Property;

#[doc(hidden)]
pub trait AppendProperty {
	fn append_property(self, decls: &mut Vec<Property>);
}

impl AppendProperty for () {
	fn append_property(self, _: &mut Vec<Property>) {}
}

impl AppendProperty for Vec<Property> {
	fn append_property(mut self, decls: &mut Vec<Property>) { decls.append(&mut self); }
}

impl AppendProperty for Property {
	fn append_property(self, decls: &mut Vec<Property>) { decls.push(self); }
}

impl AppendProperty for String {
	fn append_property(self, decls: &mut Vec<Property>) { decls.push(Property::Raw(self)); }
}

impl AppendProperty for &'static str {
	fn append_property(self, decls: &mut Vec<Property>) { decls.push(Property::Raw(self.into())); }
}

impl<F: FnOnce(&mut Vec<Property>)> AppendProperty for F {
	fn append_property(self, decls: &mut Vec<Property>) { self(decls); }
}
