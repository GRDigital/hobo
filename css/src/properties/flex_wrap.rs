// #[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone, Copy, strum_macros::Display)]
// pub enum FlexWrap {
//     #[default]
//     #[strum(to_string = "flex-wrap:nowrap;")] Nowrap,
//     #[strum(to_string = "flex-wrap:wrap;")] Wrap,
//     #[strum(to_string = "flex-wrap:wrap-reverse;")] WrapReverse,
//     #[strum(to_string = "flex-wrap:initial;")] Initial,
//     #[strum(to_string = "flex-wrap:inherit;")] Inherit,
// }

// #[macro_export]
// macro_rules! flex_wrap {
//     (nowrap)       => { $crate::Property::FlexWrap($crate::FlexWrap::Nowrap) };
//     (wrap)         => { $crate::Property::FlexWrap($crate::FlexWrap::Wrap) };
//     (wrap-reverse) => { $crate::Property::FlexWrap($crate::FlexWrap::WrapReverse) };
//     (initial)      => { $crate::Property::FlexWrap($crate::FlexWrap::Initial) };
//     (inherit)      => { $crate::Property::FlexWrap($crate::FlexWrap::Inherit) };
// }
