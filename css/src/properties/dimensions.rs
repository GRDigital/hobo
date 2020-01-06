use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Dimension {
	Zero,
	Auto,
	Initial,
	Inherit,
	Some(Unit),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum DimensionExtremity {
	Zero,
	Initial,
	Inherit,
	Some(Unit),
}

#[rustfmt::skip]
impl ToString for Dimension {
	fn to_string(&self) -> String {
		match self {
			Self::Zero       => "0".to_owned(),
			Self::Auto       => "auto".to_owned(),
			Self::Initial    => "initial".to_owned(),
			Self::Inherit    => "inherit".to_owned(),
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
			Self::Some(unit) => unit.to_string(),
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
	($kind:ident, $($val:tt)+) => {$crate::Property::$kind($crate::DimensionExtremity::Some($crate::unit!($($val)+)))};
}

#[macro_export] macro_rules! min_width { ($($tt:tt)+) => { $crate::__dimension_extremity!(MinWidth, $($tt)+)} }
#[macro_export] macro_rules! max_width { ($($tt:tt)+) => { $crate::__dimension_extremity!(MaxWidth, $($tt)+)} }
#[macro_export] macro_rules! min_height { ($($tt:tt)+) => { $crate::__dimension_extremity!(MinHeight, $($tt)+)} }
#[macro_export] macro_rules! max_height { ($($tt:tt)+) => { $crate::__dimension_extremity!(MaxHeight, $($tt)+)} }
