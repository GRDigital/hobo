use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum Dimension {
	Auto,
	Initial,
	Inherit,
	Unset,
	Some(Unit),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum DimensionExtremity {
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
		}
	}
}

#[rustfmt::skip]
impl std::fmt::Display for DimensionExtremity {
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

#[rustfmt::skip]
#[macro_export]
#[doc(hidden)]
macro_rules! __dimension {
	($kind:ident, auto)        => {$crate::Property::$kind($crate::Dimension::Auto)};
	($kind:ident, initial)     => {$crate::Property::$kind($crate::Dimension::Initial)};
	($kind:ident, inherit)     => {$crate::Property::$kind($crate::Dimension::Inherit)};
	($kind:ident, unset)       => {$crate::Property::$kind($crate::Dimension::Unset)};
	($kind:ident, $($val:tt)+) => {$crate::Property::$kind($crate::Dimension::Some($crate::unit!($($val)+)))};
}

#[macro_export] macro_rules! width { ($($tt:tt)+) => {$crate::__dimension_extremity!(Width, $($tt)+)} }
#[macro_export] macro_rules! height { ($($tt:tt)+) => {$crate::__dimension_extremity!(Height, $($tt)+)} }

#[rustfmt::skip]
#[macro_export]
#[doc(hidden)]
macro_rules! __dimension_extremity {
	($kind:ident, auto)        => {$crate::Property::$kind($crate::DimensionExtremity::Auto)};
	($kind:ident, initial)     => {$crate::Property::$kind($crate::DimensionExtremity::Initial)};
	($kind:ident, inherit)     => {$crate::Property::$kind($crate::DimensionExtremity::Inherit)};
	($kind:ident, unset)       => {$crate::Property::$kind($crate::DimensionExtremity::Unset)};
	($kind:ident, none)        => {$crate::Property::$kind($crate::DimensionExtremity::None)};
	($kind:ident, max-content) => {$crate::Property::$kind($crate::DimensionExtremity::MaxContent)};
	($kind:ident, min-content) => {$crate::Property::$kind($crate::DimensionExtremity::MinContent)};
	($kind:ident, $($val:tt)+) => {$crate::Property::$kind($crate::DimensionExtremity::Some($crate::unit!($($val)+)))};
}

#[macro_export] macro_rules! min_width { ($($tt:tt)+) => { $crate::__dimension_extremity!(MinWidth, $($tt)+)} }
#[macro_export] macro_rules! max_width { ($($tt:tt)+) => { $crate::__dimension_extremity!(MaxWidth, $($tt)+)} }
#[macro_export] macro_rules! min_height { ($($tt:tt)+) => { $crate::__dimension_extremity!(MinHeight, $($tt)+)} }
#[macro_export] macro_rules! max_height { ($($tt:tt)+) => { $crate::__dimension_extremity!(MaxHeight, $($tt)+)} }
