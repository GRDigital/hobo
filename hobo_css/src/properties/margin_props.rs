use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum Margin {
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
			Self::Auto       => "auto".fmt(f),
			Self::Initial    => "initial".fmt(f),
			Self::Inherit    => "inherit".fmt(f),
			Self::Unset      => "unset".fmt(f),
			Self::Some(unit) => unit.fmt(f),
		}
	}
}

macro_rules! decl_margins {
	($($sides:ident),*) => {paste::paste!{$(
		pub struct [<margin_ $sides>];
		impl [<margin_ $sides>] {
			insert_enumlike![crate::Property::[<Margin $sides:camel>],
				(initial, Margin::Initial),
				(inherit, Margin::Inherit),
				(unset, Margin::Unset),
				(auto, Margin::Auto),
			];
			insert_unitlike!(crate::Property::[<Margin $sides:camel>], Margin::Some);
		}
	)*}};
}
decl_margins![left, right, top, bottom];

crate::macros::easy_join!(margin_horizontal, (margin_left, margin_right), (auto, [unit]));
crate::macros::easy_join!(margin_vertical, (margin_top, margin_bottom), (auto, [unit]));
crate::macros::easy_join!(margin, (margin_horizontal, margin_vertical), (auto, [unit]));

#[rustfmt::skip]
#[macro_export]
#[doc(hidden)]
macro_rules! __margin {
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
