use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, PartialOrd, Ord)]
pub enum Margin {
	#[default]
	None,
	Auto,
	Initial,
	Inherit,
	Unset,
	Some(Unit),
}

#[rustfmt::skip]
impl std::fmt::Display for Margin {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::None       => "0".fmt(f),
			Self::Auto       => "auto".fmt(f),
			Self::Initial    => "initial".fmt(f),
			Self::Inherit    => "inherit".fmt(f),
			Self::Unset      => "unset".fmt(f),
			Self::Some(unit) => unit.fmt(f),
		}
	}
}

#[rustfmt::skip]
#[macro_export]
#[doc(hidden)]
macro_rules! __margin {
	($side:ident, 0)           => {$crate::paste::item!{ $crate::Property::[<Margin $side>]($crate::Margin::None) }};
	($side:ident, auto)        => {$crate::paste::item!{ $crate::Property::[<Margin $side>]($crate::Margin::Auto) }};
	($side:ident, initial)     => {$crate::paste::item!{ $crate::Property::[<Margin $side>]($crate::Margin::Initial) }};
	($side:ident, inherit)     => {$crate::paste::item!{ $crate::Property::[<Margin $side>]($crate::Margin::Inherit) }};
	($side:ident, unset)       => {$crate::paste::item!{ $crate::Property::[<Margin $side>]($crate::Margin::Unset) }};
	($side:ident, $($val:tt)+) => {$crate::paste::item!{ $crate::Property::[<Margin $side>]($crate::Margin::Some($crate::unit!($($val)+))) }};
}

#[macro_export] macro_rules! margin_left { ($($tt:tt)+) => { $crate::__margin!(Left, $($tt)+)} }
#[macro_export] macro_rules! margin_right { ($($tt:tt)+) => { $crate::__margin!(Right, $($tt)+)} }
#[macro_export] macro_rules! margin_top { ($($tt:tt)+) => { $crate::__margin!(Top, $($tt)+)} }
#[macro_export] macro_rules! margin_bottom { ($($tt:tt)+) => { $crate::__margin!(Bottom, $($tt)+)} }
