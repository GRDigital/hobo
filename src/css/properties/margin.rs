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

/*
should define some kind of a Box struct that impls AppendProperty
this construct will completely handle the element's position and size (i.e. the bits that influence position of other elements on the page)
split line by line until comma and run through some kind of __box_line! macro
box!(
	width (250 px .. 500 px), // syntax is either <min .. max> or just <value>
	height (100 px),
	absolute,
	top (25 px .. 10 px | 15 px), // top .. margin-top | padding-top
	right (0 .. 5px |1 px solid black| 10 px), // right .. margin-right | border-right-width border-right-style border-right-color | padding-right
)

so i.e., the above structure would be equaivalent to:
{
	// these are appended automatically as it's what you'd almost certainly want
	box-sizing: border-box;
	flex-shrink: 0;

	min-width: 250px;
	max-width: 500px;
	height: 100px;
	position: absolute;
	top: 25px;
	margin-top: 10px;
	padding-top: 15px;
	right: 0;
	margin-right: 5px;
	border-right-width: 1px;
	border-right-style: solid;
	border-right-color: black;
	padding-right: 10px;
}
*/

// update @ 22 november - dumb as shit, definitely no the way to go
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
