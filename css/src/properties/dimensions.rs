use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Dimension {
	Zero,
	Auto,
	Initial,
	Inherit,
	Unset,
	Revert,
	Some(Unit),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum DimensionExtremity {
	Zero,
	Initial,
	Inherit,
	Unset,
	Revert,
	Some(Unit),
	None,
	MaxContent,
	MinContent,
}

#[rustfmt::skip]
impl ToString for Dimension {
	fn to_string(&self) -> String {
		match self {
			Self::Zero       => "0".to_owned(),
			Self::Auto       => "auto".to_owned(),
			Self::Initial    => "initial".to_owned(),
			Self::Inherit    => "inherit".to_owned(),
			Self::Unset      => "unset;".to_owned(),
			Self::Revert     => "revert;".to_owned(),
			Self::Some(unit) => unit.to_string(),
		}
	}
}

#[rustfmt::skip]
impl ToString for DimensionExtremity {
	fn to_string(&self) -> String {
		match self {
			Self::Zero       => "0".to_owned(),
			Self::Initial    => "initial".to_owned(),
			Self::Inherit    => "inherit".to_owned(),
			Self::Unset      => "unset;".to_owned(),
			Self::Revert     => "revert;".to_owned(),
			Self::Some(unit) => unit.to_string(),
			Self::None       => "none".to_owned(),
			Self::MaxContent => "max-content".to_owned(),
			Self::MinContent => "min-content".to_owned(),
		}
	}
}

#[rustfmt::skip]
#[macro_export]
#[doc(hidden)]
macro_rules! __dimension {
	($kind:ident, 0)           => {$crate::Property::$kind($crate::Dimension::Zero)};
	($kind:ident, auto)        => {$crate::Property::$kind($crate::Dimension::Auto)};
	($kind:ident, initial)     => {$crate::Property::$kind($crate::Dimension::Initial)};
	($kind:ident, inherit)     => {$crate::Property::$kind($crate::Dimension::Inherit)};
	($kind:ident, unset)       => {$crate::Property::$kind($crate::Dimension::Unset)};
	($kind:ident, revert)      => {$crate::Property::$kind($crate::Dimension::Revert)};
	($kind:ident, $($val:tt)+) => {$crate::Property::$kind($crate::Dimension::Some($crate::unit!($($val)+)))};
}

#[macro_export] macro_rules! width { ($($tt:tt)+) => {$crate::__dimension!(Width, $($tt)+)} }
#[macro_export] macro_rules! height { ($($tt:tt)+) => {$crate::__dimension!(Height, $($tt)+)} }

#[rustfmt::skip]
#[macro_export]
#[doc(hidden)]
macro_rules! __dimension_extremity {
	($kind:ident, 0)           => {$crate::Property::$kind($crate::DimensionExtremity::Zero)};
	($kind:ident, initial)     => {$crate::Property::$kind($crate::DimensionExtremity::Initial)};
	($kind:ident, inherit)     => {$crate::Property::$kind($crate::DimensionExtremity::Inherit)};
	($kind:ident, unset)       => {$crate::Property::$kind($crate::DimensionExtremity::Unset)};
	($kind:ident, revert)      => {$crate::Property::$kind($crate::DimensionExtremity::Revert)};
	($kind:ident, none)        => {$crate::Property::$kind($crate::DimensionExtremity::None)};
	($kind:ident, max-content) => {$crate::Property::$kind($crate::DimensionExtremity::MaxContent)};
	($kind:ident, min-content) => {$crate::Property::$kind($crate::DimensionExtremity::MinContent)};
	($kind:ident, $($val:tt)+) => {$crate::Property::$kind($crate::DimensionExtremity::Some($crate::unit!($($val)+)))};
}

#[macro_export] macro_rules! min_width { ($($tt:tt)+) => { $crate::__dimension_extremity!(MinWidth, $($tt)+)} }
#[macro_export] macro_rules! max_width { ($($tt:tt)+) => { $crate::__dimension_extremity!(MaxWidth, $($tt)+)} }
#[macro_export] macro_rules! min_height { ($($tt:tt)+) => { $crate::__dimension_extremity!(MinHeight, $($tt)+)} }
#[macro_export] macro_rules! max_height { ($($tt:tt)+) => { $crate::__dimension_extremity!(MaxHeight, $($tt)+)} }
