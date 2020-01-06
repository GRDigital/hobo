css_macros::easy_enum! {flex-wrap nowrap wrap wrap-reverse}
css_macros::easy_enum! {flex-direction row row-reverse column column-reverse}
css_macros::easy_enum! {justify-content flex-start flex-end center space-between space-around}
css_macros::easy_enum! {align-items stretch center flex-start flex-end baseline}
css_macros::easy_enum! {align-content stretch center flex-start flex-end space-between space-around}
css_macros::easy_enum! {align-self auto stretch center flex-start flex-end baseline}

#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone, Copy)]
pub enum FlexBasis {
	#[default]
	Some(#[default = 1] i32),
	Initial,
	Inherit,
}

#[rustfmt::skip]
impl ToString for FlexBasis {
	fn to_string(&self) -> String {
		match self {
			Self::Initial    => "flex-basis:initial;".to_owned(),
			Self::Inherit    => "flex-basis:inherit;".to_owned(),
			Self::Some(x)    => format!("flex-basis:{};", x),
		}
	}
}

#[rustfmt::skip]
#[macro_export]
macro_rules! flex_basis {
	($some:expr) => { $crate::Property::FlexBasis($crate::FlexBasis::Some($some)) };
	(initial)    => { $crate::Property::FlexBasis($crate::FlexBasis::Initial) };
	(inherit)    => { $crate::Property::FlexBasis($crate::FlexBasis::Inherit) };
}

#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone, Copy)]
pub enum FlexGrow {
	#[default]
	Some(i32),
	Initial,
	Inherit,
}

#[rustfmt::skip]
impl ToString for FlexGrow {
	fn to_string(&self) -> String {
		match self {
			Self::Initial    => "flex-grow:initial;".to_owned(),
			Self::Inherit    => "flex-grow:inherit;".to_owned(),
			Self::Some(x)    => format!("flex-grow:{};", x),
		}
	}
}

#[rustfmt::skip]
#[macro_export]
macro_rules! flex_grow {
	($some:expr) => { $crate::Property::FlexGrow($crate::FlexGrow::Some($some)) };
	(initial)    => { $crate::Property::FlexGrow($crate::FlexGrow::Initial) };
	(inherit)    => { $crate::Property::FlexGrow($crate::FlexGrow::Inherit) };
}

#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone, Copy)]
pub enum FlexShrink {
	Zero,
	#[default]
	Some(#[default = 1] i32),
	Initial,
	Inherit,
}

#[rustfmt::skip]
impl ToString for FlexShrink {
	fn to_string(&self) -> String {
		match self {
			Self::Initial    => "flex-shrink:initial;".to_owned(),
			Self::Inherit    => "flex-shrink:inherit;".to_owned(),
			Self::Some(x)    => format!("flex-shrink:{};", x),
			Self::Zero       => "flex-shrink:0;".to_owned(),
		}
	}
}

#[rustfmt::skip]
#[macro_export]
macro_rules! flex_shrink {
	($some:expr) => { $crate::Property::FlexShrink($crate::FlexShrink::Some($some)) };
	(initial)    => { $crate::Property::FlexShrink($crate::FlexShrink::Initial) };
	(inherit)    => { $crate::Property::FlexShrink($crate::FlexShrink::Inherit) };
}

#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone, Copy)]
pub enum Order {
	#[default]
	Some(i32),
	Initial,
	Inherit,
}

#[rustfmt::skip]
impl ToString for Order {
	fn to_string(&self) -> String {
		match self {
			Self::Initial    => "order:initial;".to_owned(),
			Self::Inherit    => "order:inherit;".to_owned(),
			Self::Some(x)    => format!("order:{};", x),
		}
	}
}

#[rustfmt::skip]
#[macro_export]
macro_rules! order {
	($some:expr) => { $crate::Property::Order($crate::Order::Some($some)) };
	(initial)    => { $crate::Property::Order($crate::Order::Initial) };
	(inherit)    => { $crate::Property::Order($crate::Order::Inherit) };
}
