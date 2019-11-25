#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone, Copy, strum_macros::Display)]
pub enum FlexWrap {
	#[default]
	#[strum(to_string = "flex-wrap:nowrap;")] Nowrap,
	#[strum(to_string = "flex-wrap:wrap;")] Wrap,
	#[strum(to_string = "flex-wrap:wrap-reverse;")] WrapReverse,
	#[strum(to_string = "flex-wrap:initial;")] Initial,
	#[strum(to_string = "flex-wrap:inherit;")] Inherit,
}

#[macro_export]
macro_rules! flex_wrap {
	(nowrap)	   => { $crate::Property::FlexWrap($crate::FlexWrap::Nowrap) };
	(wrap)		   => { $crate::Property::FlexWrap($crate::FlexWrap::Wrap) };
	(wrap-reverse) => { $crate::Property::FlexWrap($crate::FlexWrap::WrapReverse) };
	(initial)	   => { $crate::Property::FlexWrap($crate::FlexWrap::Initial) };
	(inherit)	   => { $crate::Property::FlexWrap($crate::FlexWrap::Inherit) };
}

#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone, Copy, strum_macros::Display)]
pub enum FlexDirection {
	#[default]
	#[strum(to_string = "flex-direction:row;")] Row,
	#[strum(to_string = "flex-direction:row-reverse;")] RowReverse,
	#[strum(to_string = "flex-direction:column;")] Column,
	#[strum(to_string = "flex-direction:column-reverse;")] ColumnReverse,
	#[strum(to_string = "flex-direction:initial;")] Initial,
	#[strum(to_string = "flex-direction:inherit;")] Inherit,
}

#[macro_export]
macro_rules! flex_direction {
	(row)			 => { $crate::Property::FlexDirection($crate::FlexDirection::Row) };
	(row-reverse)	 => { $crate::Property::FlexDirection($crate::FlexDirection::RowReverse) };
	(column)		 => { $crate::Property::FlexDirection($crate::FlexDirection::Column) };
	(column-reverse) => { $crate::Property::FlexDirection($crate::FlexDirection::ColumnReverse) };
	(initial)		 => { $crate::Property::FlexDirection($crate::FlexDirection::Initial) };
	(inherit)		 => { $crate::Property::FlexDirection($crate::FlexDirection::Inherit) };
}

#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone, Copy, strum_macros::Display)]
pub enum JustifyContent {
	#[default]
	#[strum(to_string = "justify-content:flex-start;")] FlexStart,
	#[strum(to_string = "justify-content:flex-end;")] FlexEnd,
	#[strum(to_string = "justify-content:center;")] Center,
	#[strum(to_string = "justify-content:space-between;")] SpaceBetween,
	#[strum(to_string = "justify-content:space-around;")] SpaceAround,
	#[strum(to_string = "justify-content:initial;")] Initial,
	#[strum(to_string = "justify-content:inherit;")] Inherit,
}

#[macro_export]
macro_rules! justify_content {
	(flex-start)	=> { $crate::Property::JustifyContent($crate::JustifyContent::FlexStart) };
	(flex-end)		=> { $crate::Property::JustifyContent($crate::JustifyContent::FlexEnd) };
	(center)		=> { $crate::Property::JustifyContent($crate::JustifyContent::Center) };
	(space-between) => { $crate::Property::JustifyContent($crate::JustifyContent::SpaceBetween) };
	(space-around)	=> { $crate::Property::JustifyContent($crate::JustifyContent::SpaceAround) };
	(initial)		=> { $crate::Property::JustifyContent($crate::JustifyContent::Initial) };
	(inherit)		=> { $crate::Property::JustifyContent($crate::JustifyContent::Inherit) };
}

#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone, Copy, strum_macros::Display)]
pub enum AlignItems {
	#[default]
	#[strum(to_string = "align-items:stretch;")] Stretch,
	#[strum(to_string = "align-items:center;")] Center,
	#[strum(to_string = "align-items:flex-start;")] FlexStart,
	#[strum(to_string = "align-items:flex-end;")] FlexEnd,
	#[strum(to_string = "align-items:baseline;")] Baseline,
	#[strum(to_string = "align-items:initial;")] Initial,
	#[strum(to_string = "align-items:inherit;")] Inherit,
}

#[macro_export]
macro_rules! align_items {
	(stretch)	 => { $crate::Property::AlignItems($crate::AlignItems::Stretch) };
	(center)	 => { $crate::Property::AlignItems($crate::AlignItems::Center) };
	(flex-start) => { $crate::Property::AlignItems($crate::AlignItems::FlexStart) };
	(flex-end)	 => { $crate::Property::AlignItems($crate::AlignItems::FlexEnd) };
	(baseline)	 => { $crate::Property::AlignItems($crate::AlignItems::Baseline) };
	(initial)	 => { $crate::Property::AlignItems($crate::AlignItems::Initial) };
	(inherit)	 => { $crate::Property::AlignItems($crate::AlignItems::Inherit) };
}

#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone, Copy, strum_macros::Display)]
pub enum AlignContent {
	#[default]
	#[strum(to_string = "align-content:stretch;")] Stretch,
	#[strum(to_string = "align-content:center;")] Center,
	#[strum(to_string = "align-content:flex-start;")] FlexStart,
	#[strum(to_string = "align-content:flex-end;")] FlexEnd,
	#[strum(to_string = "align-content:space-between;")] SpaceBetween,
	#[strum(to_string = "align-content:space-around;")] SpaceAround,
	#[strum(to_string = "align-content:initial;")] Initial,
	#[strum(to_string = "align-content:inherit;")] Inherit,
}

#[macro_export]
macro_rules! align_content {
	(stretch)		=> { $crate::Property::AlignContent($crate::AlignContent::Stretch) };
	(center)		=> { $crate::Property::AlignContent($crate::AlignContent::Center) };
	(flex-start)	=> { $crate::Property::AlignContent($crate::AlignContent::FlexStart) };
	(flex-end)		=> { $crate::Property::AlignContent($crate::AlignContent::FlexEnd) };
	(space-between) => { $crate::Property::AlignContent($crate::AlignContent::SpaceBetween) };
	(space-around)	=> { $crate::Property::AlignContent($crate::AlignContent::SpaceAround) };
	(initial)		=> { $crate::Property::AlignContent($crate::AlignContent::Initial) };
	(inherit)		=> { $crate::Property::AlignContent($crate::AlignContent::Inherit) };
}

#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone, Copy, strum_macros::Display)]
pub enum AlignSelf {
	#[default]
	#[strum(to_string = "align-self:auto;")] Auto,
	#[strum(to_string = "align-self:stretch;")] Stretch,
	#[strum(to_string = "align-self:center;")] Center,
	#[strum(to_string = "align-self:flex-start;")] FlexStart,
	#[strum(to_string = "align-self:flex-end;")] FlexEnd,
	#[strum(to_string = "align-self:baseline;")] Baseline,
	#[strum(to_string = "align-self:initial;")] Initial,
	#[strum(to_string = "align-self:inherit;")] Inherit,
}

#[macro_export]
macro_rules! align_self {
	(auto)		 => { $crate::Property::AlignSelf($crate::AlignSelf::Auto) };
	(stretch)	 => { $crate::Property::AlignSelf($crate::AlignSelf::Stretch) };
	(center)	 => { $crate::Property::AlignSelf($crate::AlignSelf::Center) };
	(flex-start) => { $crate::Property::AlignSelf($crate::AlignSelf::FlexStart) };
	(flex-end)	 => { $crate::Property::AlignSelf($crate::AlignSelf::FlexEnd) };
	(baseline)	 => { $crate::Property::AlignSelf($crate::AlignSelf::Baseline) };
	(initial)	 => { $crate::Property::AlignSelf($crate::AlignSelf::Initial) };
	(inherit)	 => { $crate::Property::AlignSelf($crate::AlignSelf::Inherit) };
}

#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone, Copy)]
pub enum FlexBasis {
	#[default]
	Some(#[default = 1] i32),
	Initial,
	Inherit,
}

impl ToString for FlexBasis {
	fn to_string(&self) -> String {
		match self {
			Self::Initial    => "flex-basis:initial;".to_owned(),
			Self::Inherit    => "flex-basis:inherit;".to_owned(),
			Self::Some(x)    => format!("flex-basis:{};", x),
		}
	}
}

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

impl ToString for FlexGrow {
	fn to_string(&self) -> String {
		match self {
			Self::Initial    => "flex-grow:initial;".to_owned(),
			Self::Inherit    => "flex-grow:inherit;".to_owned(),
			Self::Some(x)    => format!("flex-grow:{};", x),
		}
	}
}

#[macro_export]
macro_rules! flex_grow {
	($some:expr) => { $crate::Property::FlexGrow($crate::FlexGrow::Some($some)) };
	(initial)    => { $crate::Property::FlexGrow($crate::FlexGrow::Initial) };
	(inherit)    => { $crate::Property::FlexGrow($crate::FlexGrow::Inherit) };
}

#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone, Copy)]
pub enum FlexShrink {
	#[default]
	Some(#[default = 1] i32),
	Initial,
	Inherit,
}

impl ToString for FlexShrink {
	fn to_string(&self) -> String {
		match self {
			Self::Initial    => "flex-shrink:initial;".to_owned(),
			Self::Inherit    => "flex-shrink:inherit;".to_owned(),
			Self::Some(x)    => format!("flex-shrink:{};", x),
		}
	}
}

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

impl ToString for Order {
	fn to_string(&self) -> String {
		match self {
			Self::Initial    => "order:initial;".to_owned(),
			Self::Inherit    => "order:inherit;".to_owned(),
			Self::Some(x)    => format!("order:{};", x),
		}
	}
}

#[macro_export]
macro_rules! order {
	($some:expr) => { $crate::Property::Order($crate::Order::Some($some)) };
	(initial)    => { $crate::Property::Order($crate::Order::Initial) };
	(inherit)    => { $crate::Property::Order($crate::Order::Inherit) };
}
