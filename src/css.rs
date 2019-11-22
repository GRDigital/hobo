#[macro_use] pub mod properties;
#[macro_use] pub mod units;
#[macro_use] pub mod selector;

pub use properties::*;
use std::string::ToString;
pub use units::Unit;
#[doc(inline)]
pub use crate::{
	style,
	background_color,
	display,
	flex_wrap,
	height,
	margin_bottom,
	margin_left,
	margin_right,
	margin_top,
	max_height,
	max_width,
	min_height,
	min_width,
	padding_bottom,
	padding_left,
	padding_right,
	padding_top,
	width,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Rule(pub selector::Selector, pub Vec<Property>);

impl ToString for Rule {
	fn to_string(&self) -> String {
		format!("{}{{{}}}", self.0.to_string(), self.1.iter().map(ToString::to_string).collect::<String>())
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Style(pub Vec<Rule>);

impl ToString for Style {
	fn to_string(&self) -> String {
		self.0.iter().map(ToString::to_string).collect::<String>()
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
	fn append_property(mut self, decls: &mut Vec<Property>) {
		decls.append(&mut self);
	}
}

impl AppendProperty for Property {
	fn append_property(self, decls: &mut Vec<Property>) {
		decls.push(self);
	}
}

#[macro_export]
macro_rules! declarations {
	($($e:expr),*$(,)*) => {{
		let mut v = Vec::new();
		$($crate::css::AppendProperty::append_property($e, &mut v);)*
		v
	}};
}

#[macro_export]
macro_rules! rule {
	// finished
	(($($selector:tt)+) { $($rules:tt)+ }) => {
		$crate::css::Rule(
			$crate::selector!($($selector)+),
			$crate::declarations!($($rules)+),
		)
	};

	// middle
	(($($head:tt)+) $cur:tt $($tail:tt)+) => {
		$crate::rule!(($($head)+ $cur) $($tail)+)
	};

	// start
	($head:tt $($tail:tt)+) => {
		$crate::rule!(($head) $($tail)+)
	};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __accumulate_style {
	(
		acc = $acc:expr,
		rules = ()
	) => {{
		$crate::css::Style($acc)
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
	}
}

#[test]
fn macros() {
	assert_eq!(format!("#{:x}{:x}{:x}{:x}", 0xf1, 0xf2, 0xf3, 0xff), "#f1f2f3ff");
	assert_eq!(format!("#{:x}{:x}{:x}{:x}", 0xf1, 0xf2, 0xf3, 0x44), "#f1f2f344");
	assert_eq!(format!("#{:x}{:x}{:x}{:x}", 255, 128, 255, 255), "#ff80ffff");
	assert_eq!(format!("#{:x}{:x}{:x}{:x}", 255, 128, 255, 128), "#ff80ff80");

	assert_eq!(
		style! {
			div#("id"):first_child > span >> div::after {
				margin_left!(10 px),
				display!(block),
			}

			div.("fsdg"):hover > span >> div::after {
				display!(block),
				margin_left!(10 px),
			}
		}.to_string(),
		"div#id:first-child>span div::after{margin-left:10px;display:block;}div.fsdg:hover>span div::after{display:block;margin-left:10px;}",
	);
	assert_eq!(
		style! {
			div#("id"):first_child > span >> div::after {
				margin_left!(10 px),
				display!(block),
			}
		}.to_string(),
		"div#id:first-child>span div::after{margin-left:10px;display:block;}",
	);
	assert_eq!(
		style! {
			div.&#("id"):first_child > span >> div::after {
				margin_left!(10 px),
				display!(block),
			}

			.&.("fsdg"):hover > span >> div::after {
				display!(block),
				margin_left!(10 px),
			}
		}.to_string(),
		"div.&#id:first-child>span div::after{margin-left:10px;display:block;}.&.fsdg:hover>span div::after{display:block;margin-left:10px;}",
	);

	assert_eq!(
		style! {
			div.&#("id"):first_child > span >> div::after {
				margin_left!(10 px),
				display!(block),
			}

			.&.("fsdg"):hover > span >> div::after {
				display!(block),
				margin_left!(10 px),
			}

			.&.("asdf"):hover > span >> div::after {
				display!(flex),
				margin_right!(10 px),
			}
		}.to_string(),
		"div.&#id:first-child>span div::after{margin-left:10px;display:block;}.&.fsdg:hover>span div::after{display:block;margin-left:10px;}.&.asdf:hover>span div::after{display:flex;margin-right:10px;}",
	);

	assert_eq!(selector!(div).to_string(), "div");
	assert_eq!(selector!(div:nth_child(5)).to_string(), "div:nth-child(5)");
	assert_eq!(selector!(.("fsdg")).to_string(), ".fsdg");
	assert_eq!(selector!(#("fsdg")).to_string(), "#fsdg");
	assert_eq!(selector!(:active).to_string(), ":active");
	assert_eq!(selector!(::after).to_string(), "::after");
	assert_eq!(selector!(div.("fsdg"):hover > span >> div::after).to_string(), "div.fsdg:hover>span div::after");
	assert_eq!(selector!(div#("id"):first_child > span >> div::after).to_string(), "div#id:first-child>span div::after");

	assert_eq!(
		rule! {
			div.("fsdg"):hover > span >> div::after {
				display!(block),
				margin_left!(10 px),
			}
		}.to_string(),
		"div.fsdg:hover>span div::after{display:block;margin-left:10px;}",
	);
}
