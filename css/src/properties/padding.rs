use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone, Copy)]
pub enum Padding {
	#[default]
	None,
	Initial,
	Inherit,
	Some(Unit),
}

#[rustfmt::skip]
impl ToString for Padding {
	fn to_string(&self) -> String {
		match self {
			Self::None       => "0".to_owned(),
			Self::Initial    => "initial".to_owned(),
			Self::Inherit    => "inherit".to_owned(),
			Self::Some(unit) => unit.to_string(),
		}
	}
}

#[rustfmt::skip]
#[macro_export]
#[doc(hidden)]
macro_rules! __padding {
	($side:ident, 0)           => {$crate::paste::expr!{ $crate::Property::[<Padding $side>]($crate::Padding::None) }};
	($side:ident, initial)     => {$crate::paste::expr!{ $crate::Property::[<Padding $side>]($crate::Padding::Initial) }};
	($side:ident, inherit)     => {$crate::paste::expr!{ $crate::Property::[<Padding $side>]($crate::Padding::Inherit) }};
	($side:ident, $($val:tt)+) => {$crate::paste::expr!{ $crate::Property::[<Padding $side>]($crate::Padding::Some($crate::unit!($($val)+))) }};
}

#[macro_export] macro_rules! padding_left { ($($tt:tt)+) => { $crate::__padding!(Left, $($tt)+)} }
#[macro_export] macro_rules! padding_right { ($($tt:tt)+) => { $crate::__padding!(Right, $($tt)+)} }
#[macro_export] macro_rules! padding_top { ($($tt:tt)+) => { $crate::__padding!(Top, $($tt)+)} }
#[macro_export] macro_rules! padding_bottom { ($($tt:tt)+) => { $crate::__padding!(Bottom, $($tt)+)} }
#[macro_export] macro_rules! padding { ($($tt:tt)+) => {
	vec![
		$crate::__padding!(Left, $($tt)+),
		$crate::__padding!(Right, $($tt)+),
		$crate::__padding!(Top, $($tt)+),
		$crate::__padding!(Bottom, $($tt)+),
	]
} }
