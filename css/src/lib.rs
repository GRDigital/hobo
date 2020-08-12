pub mod prelude;
#[macro_use] pub mod properties;
#[doc(hidden)]
#[macro_use] pub mod shortcuts;
#[macro_use] pub mod units;
#[macro_use] pub mod selector;
pub mod color;
pub mod font_face;
pub mod media;
pub mod append_property;

#[doc(hidden)]
pub use paste;
pub use properties::*;
use std::borrow::Cow;
pub use units::Unit;
pub use hobo_css_macros as css_macros;
pub use hobo_css_macros_decl as css_macros_decl;
pub use color::Color;
pub use units::F32;
pub use append_property::AppendProperty;

#[extend::ext(pub)]
impl F32 {
	fn new_unwrap(x: f32) -> Self {
		F32::new(x).unwrap()
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Rule {
	Style(StyleRule),
	Media(media::MediaSelector, Style),
	// Keyframes,
	FontFace(font_face::FontFace),
}

impl std::fmt::Display for Rule {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Style(x) => x.fmt(f),
			Self::Media(selector, style) => write!(f, "@media {}{{{}}}", selector, style),
			Self::FontFace(x) => x.fmt(f),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct StyleRule(pub selector::Selector, pub Vec<Property>);

impl std::fmt::Display for StyleRule {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)?;
		"{".fmt(f)?;
		for property in &self.1 {
			property.fmt(f)?;
		}
		"}".fmt(f)
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Style(pub Vec<Rule>);

impl std::fmt::Display for Style {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for rule in &self.0 {
			rule.fmt(f)?;
		}
		Ok(())
	}
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

#[macro_export]
macro_rules! properties {
	($($e:expr),*$(,)?) => {{
		let mut v = Vec::new();
		$($crate::AppendProperty::append_property($e, &mut v);)*
		v
	}};
}

#[macro_export]
macro_rules! class {
	($($rules:tt)*) => {$crate::style!(.& { $($rules)* })};
}

// TODO: procmacroify?
#[macro_export]
macro_rules! rule {
	(@font-face { $($prop:ident : $value:expr),*$(,)? }) => {{
		use $crate::font_face::*;

		$crate::Rule::FontFace($crate::font_face::FontFace {
			$($prop: $value),*,
			..$crate::font_face::FontFace::default()
		})
	}};

	// finished @media
	((@media $($selector:tt)+) { $($style:tt)* }) => {
		$crate::Rule::Media(
			$crate::css_macros_decl::media_selector!($($selector)+),
			$crate::style!($($style)*),
		)
	};

	// finished
	(($($selector:tt)+) { $($rules:tt)* }) => {
		$crate::Rule::Style($crate::StyleRule(
			$crate::css_macros_decl::selector!($($selector)+),
			$crate::properties!($($rules)*),
		))
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

// TODO: procmacroify
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

// TODO: procmacroify
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
