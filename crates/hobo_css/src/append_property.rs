use crate::Property;

/// A trait implemented by everything that can be added to a list of properties
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

macro_rules! append_tuples {
	() => {};
	($first:ident $($rest:ident)*) => {
		#[allow(non_snake_case)]
		impl<$first: AppendProperty, $($rest: AppendProperty),*> AppendProperty for ($first, $($rest),*) {
			fn append_property(self, decls: &mut Vec<Property>) {
				let ($first, $($rest),*) = self;
				$first.append_property(decls);
				$($rest.append_property(decls);)*
			}
		}

		append_tuples! {$($rest)*}
	};
}

append_tuples! {
	T1  T2  T3  T4  T5  T6  T7  T8  T9  T10
	T11 T12 T13 T14 T15 T16 T17 T18 T19 T20
	T21 T22 T23 T24 T25 T26 T27 T28 T29 T30
	T31 T32 T33 T34 T35 T36 T37 T38 T39 T40
	T41 T42 T43 T44 T45 T46 T47 T48 T49 T50
	T51 T52 T53 T54 T55 T56 T57 T58 T59 T60
	T61 T62 T63 T64 T65 T66 T67 T68 T69 T70
	T71 T72 T73 T74 T75 T76 T77 T78 T79 T80
	T81 T82 T83 T84 T85 T86 T87 T88 T89 T90
	T91 T92 T93 T94 T95 T96 T97 T98 T99 T100
}

impl<T: AppendProperty> From<T> for crate::Style {
	fn from(other: T) -> Self { crate::style!(.& { other }) }
}
