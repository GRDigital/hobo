use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Appearance {
	Initial,
	Inherit,
	Unset,
	None,
	Auto,
}

#[rustfmt::skip]
impl std::fmt::Display for Appearance {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Initial => "appearance:initial;-webkit-appearance:initial;-moz-appearance:initial;".fmt(f),
			Self::Inherit => "appearance:inherit;-webkit-appearance:inherit;-moz-appearance:inherit;".fmt(f),
			Self::Unset   => "appearance:unset;-webkit-appearance:unset;-moz-appearance:unset;".fmt(f),
			Self::None    => "appearance:none;-webkit-appearance:none;-moz-appearance:none;".fmt(f),
			Self::Auto    => "appearance:auto;-webkit-appearance:auto;-moz-appearance:auto;".fmt(f),
		}
	}
}

#[macro_export]
macro_rules! appearance {
	(initial) => {$crate::Property::Appearance($crate::Appearance::Initial)};
	(inherit) => {$crate::Property::Appearance($crate::Appearance::Inherit)};
	(unset)   => {$crate::Property::Appearance($crate::Appearance::Unset)};
	(none)    => {$crate::Property::Appearance($crate::Appearance::None)};
	(auto)    => {$crate::Property::Appearance($crate::Appearance::Auto)};
}

#[test]
fn appearance_values() {
	assert_eq!(appearance!(initial).to_string(), "appearance:initial;-webkit-appearance:initial;-moz-appearance:initial;");
	assert_eq!(appearance!(inherit).to_string(), "appearance:inherit;-webkit-appearance:inherit;-moz-appearance:inherit;");
	assert_eq!(appearance!(unset).to_string(), "appearance:unset;-webkit-appearance:unset;-moz-appearance:unset;");
	assert_eq!(appearance!(none).to_string(), "appearance:none;-webkit-appearance:none;-moz-appearance:none;");
	assert_eq!(appearance!(auto).to_string(), "appearance:auto;-webkit-appearance:auto;-moz-appearance:auto;");
}
