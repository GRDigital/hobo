pub mod prelude;
#[macro_use] pub mod properties;
#[macro_use] pub mod shortcuts;
#[macro_use] pub mod units;
#[macro_use] pub mod selector;

pub use paste;
pub use properties::*;
use std::{borrow::Cow, string::ToString};
pub use units::Unit;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Rule(pub selector::Selector, pub Vec<Property>);

impl ToString for Rule {
	fn to_string(&self) -> String { format!("{}{{{}}}", self.0.to_string(), self.1.iter().map(ToString::to_string).collect::<String>()) }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Style(pub Vec<Rule>);

impl ToString for Style {
	fn to_string(&self) -> String { self.0.iter().map(ToString::to_string).collect::<String>() }
}

impl Style {
	pub fn append(&mut self, other: &mut Style) { self.0.append(&mut other.0); }
}

impl<'a> From<&'a Style> for Cow<'a, Style> {
	fn from(x: &'a Style) -> Cow<'a, Style> {
		Cow::Borrowed(x)
	}
}

impl<'a> From<Style> for Cow<'a, Style> {
	fn from(x: Style) -> Cow<'a, Style> {
		Cow::Owned(x)
	}
}

impl<'a> From<&'a Style> for Cow<'a, AtRules> {
	fn from(x: &Style) -> Cow<'a, AtRules> {
		Cow::Owned(AtRules(vec![AtRule { style: x.clone(), media: None }]))
	}
}

impl<'a> From<Style> for Cow<'a, AtRules> {
	fn from(x: Style) -> Cow<'a, AtRules> {
		Cow::Owned(AtRules(vec![AtRule { style: x, media: None }]))
	}
}

#[doc(hidden)]
pub trait AppendProperty {
	fn append_property(self, decls: &mut Vec<Property>);
}

impl AppendProperty for () {
	fn append_property(self, _: &mut Vec<Property>) {}
}

impl AppendProperty for Vec<Property> {
	fn append_property(mut self, decls: &mut Vec<Property>) { decls.append(&mut self); }
}

impl AppendProperty for Property {
	fn append_property(self, decls: &mut Vec<Property>) { decls.push(self); }
}

impl AppendProperty for String {
	fn append_property(self, decls: &mut Vec<Property>) { decls.push(Property::Raw(self)); }
}

impl AppendProperty for &'static str {
	fn append_property(self, decls: &mut Vec<Property>) { decls.push(Property::Raw(self.into())); }
}

impl<F: FnOnce(&mut Vec<Property>)> AppendProperty for F {
	fn append_property(self, decls: &mut Vec<Property>) { self(decls); }
}

#[macro_export]
macro_rules! properties {
	($($e:expr),*$(,)*) => {{
		let mut v = Vec::new();
		$($crate::AppendProperty::append_property($e, &mut v);)*
		v
	}};
}
#[macro_export]
macro_rules! class {
	($($rules:tt)*) => {
		$crate::Style(vec![
			$crate::Rule(
				$crate::selector::Selector::build().class_placeholder(),
				$crate::properties!($($rules)*),
			),
		])
	};
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct AtRule {
	pub media: Option<Media>,
	pub style: Style,
}

impl ToString for AtRule {
	fn to_string(&self) -> String {
		if let Some(media) = self.media.as_ref() {
			format!("{}{{{}}}", media.to_string(), self.style.to_string())
		} else {
			self.style.to_string()
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct AtRules(pub Vec<AtRule>);

impl ToString for AtRules {
	fn to_string(&self) -> String {
		self.0.iter().map(ToString::to_string).collect::<String>()
	}
}

impl<'a> From<AtRules> for Cow<'a, AtRules> {
	fn from(x: AtRules) -> Cow<'a, AtRules> {
		Cow::Owned(x)
	}
}

impl<'a> From<&'a AtRules> for Cow<'a, AtRules> {
	fn from(x: &'a AtRules) -> Cow<'a, AtRules> {
		Cow::Borrowed(x)
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Media {
	MinWidth(Unit),
	MaxWidth(Unit),
	MinAspectRatio(u32, u32),
	MaxAspectRatio(u32, u32),
}

impl ToString for Media {
	fn to_string(&self) -> String {
		match self {
			Self::MinWidth(unit) => format!("@media(min-width:{})", unit.to_string()),
			Self::MaxWidth(unit) => format!("@media(max-width:{})", unit.to_string()),
			Self::MinAspectRatio(width, height) => format!("@media(min-aspect-ratio:{}/{})", width, height),
			Self::MaxAspectRatio(width, height) => format!("@media(max-aspect-ratio:{}/{})", width, height),
		}
	}
}

// TODO: kind of hacky
// #[derive(Debug, PartialEq, Eq, Hash, Clone)]
// pub struct MediaMaxWidth(pub Unit);

// impl ToString for MediaMaxWidth {
//     fn to_string(&self) -> String {
//         format!("@media (max-width:{})", self.0.to_string())
//     }
// }

// TODO: replace @font-face selector with regular rust struct
/*
pub enum FontDisplay {
	Auto,
	Block,
	Swap,
	Fallback,
	Optional,
}

pub enum FontStretch {
	UltraCondensed,
	ExtraCondensed,
	Condensed,
	SemiCondensed,
	Normal,
	SemiExpanded,
	Expanded,
	ExtraExpanded,
	UltraExpanded,
	Percentage(f32),
}

pub enum FontStyle {
	Normal,
	Italic,
	Oblique,
	ObliqueAngle(f32),
	ObliqueAngleRange(f32, f32),
}

pub enum FontWeight {
	Normal,
	Bold,
	Number(u16),
}

pub struct FontFace {
	font_family: String,
	src: Vec<(String, String)>,
	font_display: FontDisplay,
	font_stretch: Vec<FontStretch>,
	font_style: FontStyle,
	font_weight: Vec<FontWeight>,
	// font_variant:
	// font-feature-settings
	// font-variation-settings:
	unicode_range: Vec<(u32, u32)>,
}
*/

#[macro_export]
macro_rules! rule {
	// ((@font-face) { $($rules:tt),*$(,)* }) => {
	//     $crate::Rule::FontFace {
	//         $($tt:tt),*,
	//         ...$crate::FontFace::default()
	//     }
	// };

	// finished
	(($($selector:tt)+) { $($rules:tt)* }) => {
		$crate::Rule(
			$crate::selector!($($selector)+),
			$crate::properties!($($rules)*),
		)
	};

	// middle
	(($($head:tt)+) $cur:tt $($tail:tt)*) => {
		$crate::rule!(($($head)+ $cur) $($tail)*)
	};

	// start
	($head:tt $($tail:tt)*) => {
		$crate::rule!(($head) $($tail)*)
	};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __accumulate_style {
	(
		acc = $acc:expr,
		rules = ()
	) => {{
		$crate::Style($acc)
	}};

	(
		acc = $acc:expr,
		rules = ([$($rule:tt)+] $($rest:tt)*)
	) => {{
		$acc.push($crate::rule!($($rule)+));
		$crate::__accumulate_style!(acc = $acc, rules = ($($rest)*))
	}};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __style {
	(
		rules = ($($rules:tt)*),
		new_rule = (),
		rest = (),
	) => {{
		let mut acc = Vec::new();
		$crate::__accumulate_style!(acc = acc, rules = ($($rules)*))
	}};

	(
		rules = ($($rules:tt)*),
		new_rule = ($($new_rule:tt)*),
		rest = ({ $($decls:tt)* }),
	) => {
		$crate::__style!{
			rules = ($($rules)* [$($new_rule)* { $($decls)* }]),
			new_rule = (),
			rest = (),
		}
	};

	(
		rules = ($($rules:tt)*),
		new_rule = ($($new_rule:tt)*),
		rest = ({ $($decls:tt)* } $($rest:tt)*),
	) => {
		$crate::__style!{
			rules = ($($rules)* [$($new_rule)* { $($decls)* }]),
			new_rule = (),
			rest = ($($rest)*),
		}
	};

	(
		rules = ($($rules:tt)*),
		new_rule = ($($new_rule:tt)*),
		rest = ($cur:tt $($rest:tt)*),
	) => {
		$crate::__style!{
			rules = ($($rules)*),
			new_rule = ($($new_rule)* $cur),
			rest = ($($rest)*),
		}
	};
}

#[macro_export]
macro_rules! style {
	($($tt:tt)+) => {
		$crate::__style! {
			rules = (),
			new_rule = (),
			rest = ($($tt)+),
		}
	};
}

// #[test]
// fn macros() {
//     assert_eq!(format!("#{:x}{:x}{:x}{:x}", 0xf1, 0xf2, 0xf3, 0xff), "#f1f2f3ff");
//     assert_eq!(format!("#{:x}{:x}{:x}{:x}", 0xf1, 0xf2, 0xf3, 0x44), "#f1f2f344");
//     assert_eq!(format!("#{:x}{:x}{:x}{:x}", 255, 128, 255, 255), "#ff80ffff");
//     assert_eq!(format!("#{:x}{:x}{:x}{:x}", 255, 128, 255, 128), "#ff80ff80");

//     assert_eq!(
//         style! {
//             div#("id"):first_child > span >> div::after {
//                 margin_left!(10 px),
//                 display!(block),
//             }

//             div.("fsdg"):hover > span >> div::after {
//                 display!(block),
//                 margin_left!(10 px),
//             }
//         }
//         .to_string(),
//         "div#id:first-child>span div::after{margin-left:10px;display:block;}div.fsdg:hover>span \
//         div::after{display:block;margin-left:10px;}",
//     );
//     assert_eq!(
//         style! {
//             div#("id"):first_child > span >> div::after {
//                 margin_left!(10 px),
//                 display!(block),
//             }
//         }
//         .to_string(),
//         "div#id:first-child>span div::after{margin-left:10px;display:block;}",
//     );
//     assert_eq!(
//         style! {
//             div.&#("id"):first_child > span >> div::after {
//                 margin_left!(10 px),
//                 display!(block),
//             }

//             .&.("fsdg"):hover > span >> div::after {
//                 display!(block),
//                 margin_left!(10 px),
//             }
//         }
//         .to_string(),
//         "div.&#id:first-child>span div::after{margin-left:10px;display:block;}.&.fsdg:hover>span \
//         div::after{display:block;margin-left:10px;}",
//     );

//     assert_eq!(
//         style! {
//             div.&#("id"):first_child > span >> div::after {
//                 margin_left!(10 px),
//                 display!(block),
//             }

//             .&.("fsdg"):hover > span >> div::after {
//                 display!(block),
//                 margin_left!(10 px),
//             }

//             .&.("asdf"):hover > span >> div::after {
//                 display!(flex),
//                 margin_right!(10 px),
//             }
//         }
//         .to_string(),
//         "div.&#id:first-child>span div::after{margin-left:10px;display:block;}.&.fsdg:hover>span \
//         div::after{display:block;margin-left:10px;}.&.asdf:hover>span div::after{display:flex;margin-right:10px;}",
//     );

//     assert_eq!(selector!(div).to_string(), "div");
//     assert_eq!(selector!(div:nth_child(5)).to_string(), "div:nth-child(5)");
//     assert_eq!(selector!(.("fsdg")).to_string(), ".fsdg");
//     assert_eq!(selector!(#("fsdg")).to_string(), "#fsdg");
//     assert_eq!(selector!(:active).to_string(), ":active");
//     assert_eq!(selector!(::after).to_string(), "::after");
//     assert_eq!(selector!(div.("fsdg"):hover > span >> div::after).to_string(), "div.fsdg:hover>span div::after");
//     assert_eq!(selector!(div#("id"):first_child > span >> div::after).to_string(), "div#id:first-child>span div::after");

//     assert_eq!(
//         rule! {
//             div.("fsdg"):hover > span >> div::after {
//                 display!(block),
//                 margin_left!(10 px),
//             }
//         }
//         .to_string(),
//         "div.fsdg:hover>span div::after{display:block;margin-left:10px;}",
//     );
// }
