use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone, Copy)]
pub enum Dimension {
	#[default]
	None,
	Auto,
	Initial,
	Inherit,
	Some(css::Unit),
}

#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone, Copy)]
pub enum DimensionExtremity {
	#[default]
	None,
	Initial,
	Inherit,
	Some(css::Unit),
}

impl ToString for Dimension {
	fn to_string(&self) -> String {
		match self {
			Self::None       => "0".to_owned(),
			Self::Auto       => "auto".to_owned(),
			Self::Initial    => "initial".to_owned(),
			Self::Inherit    => "inherit".to_owned(),
			Self::Some(unit) => unit.to_string(),
		}
	}
}

impl ToString for DimensionExtremity {
	fn to_string(&self) -> String {
		match self {
			Self::None       => "0".to_owned(),
			Self::Initial    => "initial".to_owned(),
			Self::Inherit    => "inherit".to_owned(),
			Self::Some(unit) => unit.to_string(),
		}
	}
}

#[macro_export]
#[doc(hidden)]
macro_rules! __dimension {
	($kind:ident, 0)           => {$crate::css::Property::$kind($crate::css::Dimension::None)};
	($kind:ident, auto)        => {$crate::css::Property::$kind($crate::css::Dimension::Auto)};
	($kind:ident, initial)     => {$crate::css::Property::$kind($crate::css::Dimension::Initial)};
	($kind:ident, inherit)     => {$crate::css::Property::$kind($crate::css::Dimension::Inherit)};
	($kind:ident, $($val:tt)+) => {$crate::css::Property::$kind($crate::css::Dimension::Some($crate::unit!($($val)+)))};
}

#[macro_export] macro_rules! width { ($($tt:tt)+) => {$crate::__dimension!(Width, $($tt)+)} }
#[macro_export] macro_rules! height { ($($tt:tt)+) => {$crate::__dimension!(Height, $($tt)+)} }

#[macro_export]
#[doc(hidden)]
macro_rules! __dimension_extremity {
	($kind:ident, 0)           => {$crate::css::Property::$kind($crate::css::DimensionExtremity::None)};
	($kind:ident, initial)     => {$crate::css::Property::$kind($crate::css::DimensionExtremity::Initial)};
	($kind:ident, inherit)     => {$crate::css::Property::$kind($crate::css::DimensionExtremity::Inherit)};
	($kind:ident, $($val:tt)+) => {$crate::css::Property::$kind($crate::css::DimensionExtremity::Some($crate::unit!($($val)+)))};
}

#[macro_export] macro_rules! min_width { ($($tt:tt)+) => { $crate::__dimension_extremity!(MinWidth, $($tt)+)} }
#[macro_export] macro_rules! max_width { ($($tt:tt)+) => { $crate::__dimension_extremity!(MaxWidth, $($tt)+)} }
#[macro_export] macro_rules! min_height { ($($tt:tt)+) => { $crate::__dimension_extremity!(MinHeight, $($tt)+)} }
#[macro_export] macro_rules! max_height { ($($tt:tt)+) => { $crate::__dimension_extremity!(MaxHeight, $($tt)+)} }
