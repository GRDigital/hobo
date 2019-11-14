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
	(nowrap)       => { $crate::css::Property::FlexWrap($crate::css::FlexWrap::Nowrap) };
	(wrap)         => { $crate::css::Property::FlexWrap($crate::css::FlexWrap::Wrap) };
	(wrap-reverse) => { $crate::css::Property::FlexWrap($crate::css::FlexWrap::WrapReverse) };
	(initial)      => { $crate::css::Property::FlexWrap($crate::css::FlexWrap::Initial) };
	(inherit)      => { $crate::css::Property::FlexWrap($crate::css::FlexWrap::Inherit) };
}
