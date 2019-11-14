use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone, Copy)]
pub enum Margin {
	#[default]
	None,
	Auto,
	Initial,
	Inherit,
	Some(css::Unit),
}

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

#[macro_export]
#[doc(hidden)]
macro_rules! __margin {
	($side:ident, 0)           => {$crate::paste::expr!{ $crate::css::Property::[<Margin $side>]($crate::css::Margin::None) }};
	($side:ident, auto)        => {$crate::paste::expr!{ $crate::css::Property::[<Margin $side>]($crate::css::Margin::Auto) }};
	($side:ident, initial)     => {$crate::paste::expr!{ $crate::css::Property::[<Margin $side>]($crate::css::Margin::Initial) }};
	($side:ident, inherit)     => {$crate::paste::expr!{ $crate::css::Property::[<Margin $side>]($crate::css::Margin::Inherit) }};
	($side:ident, $($val:tt)+) => {$crate::paste::expr!{ $crate::css::Property::[<Margin $side>]($crate::css::Margin::Some($crate::unit!($($val)+))) }};
}

#[macro_export] macro_rules! margin_left { ($($tt:tt)+) => { $crate::__margin!(Left, $($tt)+)} }
#[macro_export] macro_rules! margin_right { ($($tt:tt)+) => { $crate::__margin!(Right, $($tt)+)} }
#[macro_export] macro_rules! margin_top { ($($tt:tt)+) => { $crate::__margin!(Top, $($tt)+)} }
#[macro_export] macro_rules! margin_bottom { ($($tt:tt)+) => { $crate::__margin!(Bottom, $($tt)+)} }

// idk if this is not dumb, looks pretty dumb to me
// will also need to account for 1 or 2 elements, should be ez to amend tts
// #[macro_export]
// macro_rules! margin {
//     (@@@@([$($top:tt)+] [$($right:tt)+] [$($bottom:tt)+] [$($left:tt)+])) => {
//         vec![
//             $crate::margin_top!($($top)+),
//             $crate::margin_right!($($right)+),
//             $crate::margin_bottom!($($bottom)+),
//             $crate::margin_left!($($left)+),
//         ]
//     };

//     (@@@($($head:tt)+) ($($cur_head:tt)+) ,) => {
//         $crate::margin!(@@@@($($head)+ [$($cur_head)+]))
//     };
//     (@@@($($head:tt)+) ($($cur_head:tt)*) $cur:tt) => {
//         $crate::margin!(@@@@($($head)+ [$($cur_head)* $cur]))
//     };
//     (@@@($($head:tt)+) ($($cur_head:tt)*) $cur:tt $($rest:tt)+) => {
//         $crate::margin!(@@@($($head)+) ($($cur_head)* $cur) $($rest)+)
//     };

//     (@@($($head:tt)+) ($($cur_head:tt)*) , $($rest:tt)+) => {
//         $crate::margin!(@@@($($head)+ [$($cur_head)+]) () $($rest)+)
//     };
//     (@@($($head:tt)+) ($($cur_head:tt)*) $cur:tt $($rest:tt)+) => {
//         $crate::margin!(@@($($head)+) ($($cur_head)* $cur) $($rest)+)
//     };

//     (@($($head:tt)+) ($($cur_head:tt)*) , $($rest:tt)+) => {
//         $crate::margin!(@@($($head)+ [$($cur_head)*]) () $($rest)+)
//     };
//     (@($($head:tt)+) ($($cur_head:tt)*) $cur:tt $($rest:tt)+) => {
//         $crate::margin!(@($($head)+) ($($cur_head)* $cur) $($rest)+)
//     };

//     (($($cur_head:tt)+) , $($rest:tt)+) => {
//         $crate::margin!(@([$($cur_head)+]) () $($rest)+)
//     };
//     (($($cur_head:tt)+) $cur:tt $($rest:tt)+) => {
//         $crate::margin!(($($cur_head)+ $cur) $($rest)+)
//     };

//     ($cur:tt $($rest:tt)+) => {
//         $crate::margin!(($cur) $($rest)+)
//     };
// }
