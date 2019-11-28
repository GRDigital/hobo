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

// wrap nowrap?
// border?
#[macro_export]
macro_rules! __flexbox_line {
	($acc:expr, row) => {
		$acc.push($crate::Property::FlexDirection($crate::FlexDirection::Row));
	};
	($acc:expr, column) => {
		$acc.push($crate::Property::FlexDirection($crate::FlexDirection::Column));
	};
	($acc:expr, width ($($value:tt)*)) => {
		$acc.push($crate::width!($($value)*))
	};
	($acc:expr, width ($($min:tt)*) .. ($($max:tt)*)) => {
		$acc.push($crate::min_width!($($min)*));
		$acc.push($crate::max_width!($($max)*));
	};
	($acc:expr, height ($($value:tt)*)) => {
		$acc.push($crate::height!($($value)*))
	};
	($acc:expr, height ($($min:tt)*) .. ($($max:tt)*)) => {
		$acc.push($crate::min_height!($($min)*));
		$acc.push($crate::max_height!($($max)*));
	};
	($acc:expr, top ($($margin:tt)*) | ($($padding:tt)*)) => {
		$acc.push($crate::margin_top!($($margin)*));
		$acc.push($crate::padding_top!($($padding)*));
	};
	($acc:expr, right ($($margin:tt)*) | ($($padding:tt)*)) => {
		$acc.push($crate::margin_right!($($margin)*));
		$acc.push($crate::padding_right!($($padding)*));
	};
	($acc:expr, bottom ($($margin:tt)*) | ($($padding:tt)*)) => {
		$acc.push($crate::margin_bottom!($($margin)*));
		$acc.push($crate::padding_bottom!($($padding)*));
	};
	($acc:expr, left ($($margin:tt)*) | ($($padding:tt)*)) => {
		$acc.push($crate::margin_left!($($margin)*));
		$acc.push($crate::padding_left!($($padding)*));
	};
	($acc:expr, size $($rest:tt)*) => {
		$crate::__flexbox_line!($acc, width $($rest)*);
		$crate::__flexbox_line!($acc, height $($rest)*);
	};
	($acc:expr, horizontal $($rest:tt)*) => {
		$crate::__flexbox_line!($acc, left $($rest)*);
		$crate::__flexbox_line!($acc, right $($rest)*);
	};
	($acc:expr, vertical $($rest:tt)*) => {
		$crate::__flexbox_line!($acc, top $($rest)*);
		$crate::__flexbox_line!($acc, bottom $($rest)*);
	};
	($acc:expr, around $($rest:tt)*) => {
		$crate::__flexbox_line!($acc, horizontal $($rest)*);
		$crate::__flexbox_line!($acc, vertical $($rest)*);
	};
	($acc:expr, $side:ident $margin:tt) => {
		$crate::__flexbox_line!($acc, $side $margin | (0));
	};
	($acc:expr, [$($inner:tt)*]) => { $crate::__flexbox_line!($acc, $($inner)*) };
}

#[macro_export]
macro_rules! __flexbox_inner {
	(
		lines = ($($line:tt)*),
		current_line = (),
		rest = (),
	) => {{
		let mut acc = vec![
			$crate::Property::BoxSizing($crate::BoxSizing::BorderBox),
			$crate::Property::FlexShrink($crate::FlexShrink::Zero),
			$crate::Property::Display($crate::Display::Flex),
		];
		$($crate::__flexbox_line!(acc, $line);)*
		acc
	}};
	(
		lines = ($($lines:tt)*),
		current_line = ($($current_line:tt)*),
		rest = (, $($rest:tt)*),
	) => {
		$crate::__flexbox_inner!{
			lines = ($($lines)* [$($current_line)*]),
			current_line = (),
			rest = ($($rest)*),
		}
	};
	(
		lines = ($($lines:tt)*),
		current_line = ($($current_line:tt)*),
		rest = ($current:tt $($rest:tt)*),
	) => {
		$crate::__flexbox_inner!{
			lines = ($($lines)*),
			current_line = ($($current_line)* $current),
			rest = ($($rest)*),
		}
	};
}

#[macro_export]
macro_rules! flexbox {
	($($tt:tt)*) => {
		$crate::__flexbox_inner!{
			lines = (),
			current_line = (),
			rest = ($($tt)*),
		}
	};
}

#[test]
fn flexbox_macro_test() {
	assert_eq!(
		crate::declarations!(
			crate::flexbox!(
				width (100 px) .. (200 px),
				height (200 px),
				top (100 px) | (50 px),
				horizontal (15 px),
				column,
			),
		),
		crate::declarations!(
			crate::Property::BoxSizing(crate::BoxSizing::BorderBox),
			crate::Property::FlexShrink(crate::FlexShrink::Zero),
			crate::Property::Display(crate::Display::Flex),
			crate::Property::MinWidth(crate::DimensionExtremity::Some(crate::units::Unit::Px(unsafe { crate::units::F32::unchecked_new(100.) }))),
			crate::Property::MaxWidth(crate::DimensionExtremity::Some(crate::units::Unit::Px(unsafe { crate::units::F32::unchecked_new(200.) }))),
			crate::Property::Height(crate::Dimension::Some(crate::units::Unit::Px(unsafe { crate::units::F32::unchecked_new(200.) }))),
			crate::Property::MarginTop(crate::Margin::Some(crate::units::Unit::Px(unsafe { crate::units::F32::unchecked_new(100.) }))),
			crate::Property::PaddingTop(crate::Padding::Some(crate::units::Unit::Px(unsafe { crate::units::F32::unchecked_new(50.) }))),
			crate::Property::MarginLeft(crate::Margin::Some(crate::units::Unit::Px(unsafe { crate::units::F32::unchecked_new(15.) }))),
			crate::Property::PaddingLeft(crate::Padding::None),
			crate::Property::MarginRight(crate::Margin::Some(crate::units::Unit::Px(unsafe { crate::units::F32::unchecked_new(15.) }))),
			crate::Property::PaddingRight(crate::Padding::None),
			crate::Property::FlexDirection(crate::FlexDirection::Column),
		),
	);
}

/*
should define some kind of a Box struct that impls AppendProperty
this construct will completely handle the element's position and size (i.e. the bits that influence position of other elements on the page)
split line by line until comma and run through some kind of __box_line! macro
box!(
	width (250 px) .. (500 px), // syntax is either <min .. max> or just <value>
	height (100 px),
	absolute,
	top (25 px) .. (10 px) | (15 px), // top .. margin-top | padding-top
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
