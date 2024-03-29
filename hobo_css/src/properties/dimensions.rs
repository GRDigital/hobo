use crate::prelude::*;

// doesn't work in safari
crate::macros::easy_enum! {aspect-ratio auto [float]}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum Dimension {
	Auto,
	Initial,
	Inherit,
	Unset,
	Some(Unit),
	None,
	MaxContent,
	MinContent,
}

#[rustfmt::skip]
impl std::fmt::Display for Dimension {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Auto       => "auto".fmt(f),
			Self::Initial    => "initial".fmt(f),
			Self::Inherit    => "inherit".fmt(f),
			Self::Unset      => "unset".fmt(f),
			Self::Some(unit) => unit.fmt(f),
			Self::None       => "none".fmt(f),
			Self::MaxContent => "max-content".fmt(f),
			Self::MinContent => "min-content".fmt(f),
		}
	}
}

macro_rules! decl_dimensions {
	($($dims:ident),*) => {paste::paste!{$(
		pub struct $dims;
		impl $dims {
			insert_enumlike![crate::Property::[<$dims:camel>],
				(initial, Dimension::Initial),
				(inherit, Dimension::Inherit),
				(unset, Dimension::Unset),
				(auto, Dimension::Auto),
				(min_content, Dimension::MinContent),
				(max_content, Dimension::MaxContent),
			];
			insert_unitlike!(crate::Property::[<$dims:camel>], Dimension::Some);
		}
	)*}};
}
decl_dimensions![width, height, min_width, min_height];

pub struct max_width;
impl max_width {
	insert_enumlike![crate::Property::MaxWidth,
		(initial, Dimension::Initial),
		(inherit, Dimension::Inherit),
		(unset, Dimension::Unset),
		(none, Dimension::None),
		(min_content, Dimension::MinContent),
		(max_content, Dimension::MaxContent),
	];
	insert_unitlike!(crate::Property::MaxWidth, Dimension::Some);
}

pub struct max_height;
impl max_height {
	insert_enumlike![crate::Property::MaxHeight,
		(initial, Dimension::Initial),
		(inherit, Dimension::Inherit),
		(unset, Dimension::Unset),
		(none, Dimension::None),
		(min_content, Dimension::MinContent),
		(max_content, Dimension::MaxContent),
	];
	insert_unitlike!(crate::Property::MaxHeight, Dimension::Some);
}

crate::macros::easy_join!(size, (width, height), (auto, max-content, min-content, [unit]));

/*
pub mod property_exploration {
	use super::*;
	use num_traits::cast::AsPrimitive;
	use dyn_partial_eq::DynPartialEq;

	pub trait DynHash {
		fn dyn_hash(&self, state: &mut dyn std::hash::Hasher);
	}

	#[dyn_partial_eq::dyn_partial_eq]
	pub trait Property: dyn_clone::DynClone + DynHash + std::fmt::Display {
		fn discriminant(&self) -> PropertyDiscriminant;
	}

	pub trait PropertyExt: Property + Sized + 'static {
		fn boxed(self) -> Box<dyn Property> { Box::new(self) }
	}

	impl<P: Property + Sized + 'static> PropertyExt for P {}

	impl<T: std::hash::Hash + ?Sized> DynHash for T {
		fn dyn_hash(&self, mut state: &mut dyn std::hash::Hasher) { self.hash(&mut state); }
	}

	impl std::hash::Hash for dyn Property {
		fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
			self.discriminant().hash(state);
			self.dyn_hash(state);
		}
	}

	impl Eq for Box<dyn Property> {}
	impl PartialOrd for Box<dyn Property> {
		fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.discriminant().cmp(&other.discriminant())) }
	}
	impl Ord for Box<dyn Property> {
		fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.discriminant().cmp(&other.discriminant()) }
	}
	impl Clone for Box<dyn Property> {
		fn clone(&self) -> Self { dyn_clone::clone(self) }
	}

	#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Debug)]
	pub enum PropertyDiscriminant {
		Width,
		Height,
	}

	pub mod width {
		use super::*;

		#[derive(Clone, PartialEq, Eq, Hash, Debug, DynPartialEq)]
		pub struct Width(DimensionExtremity);

		impl Property for Width { fn discriminant(&self) -> PropertyDiscriminant { PropertyDiscriminant::Width } }

		impl std::fmt::Display for Width {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				"width:".fmt(f)?;
				self.0.fmt(f)
			}
		}

		pub fn auto() -> Width { Width(DimensionExtremity::Auto) }
		pub fn frac(val: impl AsPrimitive<f32>) -> Width { Width(DimensionExtremity::Some(Unit::pct(val.as_() * 100.))) }
		pub fn px(val: impl AsPrimitive<f32>) -> Width { Width(DimensionExtremity::Some(Unit::px(val))) }
	}

	pub mod height {
		use super::*;

		#[derive(Clone, PartialEq, Eq, Hash, Debug, DynPartialEq)]
		pub struct Height(DimensionExtremity);

		impl Property for Height { fn discriminant(&self) -> PropertyDiscriminant { PropertyDiscriminant::Height } }

		impl std::fmt::Display for Height {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				"height:".fmt(f)?;
				self.0.fmt(f)
			}
		}

		pub fn auto() -> Height { Height(DimensionExtremity::Auto) }
		pub fn frac(val: impl AsPrimitive<f32>) -> Height { Height(DimensionExtremity::Some(Unit::pct(val.as_() * 100.))) }
		pub fn px(val: impl AsPrimitive<f32>) -> Height { Height(DimensionExtremity::Some(Unit::px(val))) }
	}

	macro_rules! props {
		($($prop:expr),* $(,)?) => { vec![$($prop.boxed()),*] };
	}

	fn foo() {
		use std::hash::Hasher;
		use std::hash::Hash;

		let some_prop = props![width::px(150), height::frac(0.4)];
		let mut some_prop_clone = some_prop.clone();
		some_prop_clone.sort();

		let mut hasher = std::collections::hash_map::DefaultHasher::new();
		for x in &some_prop_clone {
			x.hash(&mut hasher);
		}
		let hash_res = hasher.finish();
	}
}
*/
