use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone, Copy)]
pub enum Margin {
	#[default]
	None,
	Auto,
	Initial,
	Inherit,
	Some(Unit),
}

#[rustfmt::skip]
impl ToString for Margin {
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

#[rustfmt::skip]
#[macro_export]
#[doc(hidden)]
macro_rules! __margin {
	($side:ident, 0)           => {$crate::paste::expr!{ $crate::Property::[<Margin $side>]($crate::Margin::None) }};
	($side:ident, auto)        => {$crate::paste::expr!{ $crate::Property::[<Margin $side>]($crate::Margin::Auto) }};
	($side:ident, initial)     => {$crate::paste::expr!{ $crate::Property::[<Margin $side>]($crate::Margin::Initial) }};
	($side:ident, inherit)     => {$crate::paste::expr!{ $crate::Property::[<Margin $side>]($crate::Margin::Inherit) }};
	($side:ident, $($val:tt)+) => {$crate::paste::expr!{ $crate::Property::[<Margin $side>]($crate::Margin::Some($crate::unit!($($val)+))) }};
}

#[macro_export] macro_rules! margin_left { ($($tt:tt)+) => { $crate::__margin!(Left, $($tt)+)} }
#[macro_export] macro_rules! margin_right { ($($tt:tt)+) => { $crate::__margin!(Right, $($tt)+)} }
#[macro_export] macro_rules! margin_top { ($($tt:tt)+) => { $crate::__margin!(Top, $($tt)+)} }
#[macro_export] macro_rules! margin_bottom { ($($tt:tt)+) => { $crate::__margin!(Bottom, $($tt)+)} }
#[macro_export] macro_rules! margin { ($($tt:tt)+) => {
	vec![
		$crate::__margin!(Left, $($tt)+),
		$crate::__margin!(Right, $($tt)+),
		$crate::__margin!(Top, $($tt)+),
		$crate::__margin!(Bottom, $($tt)+),
	]
} }
