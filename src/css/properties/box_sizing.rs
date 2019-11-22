#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, strum_macros::Display)]
pub enum BoxSizing {
	#[strum(to_string = "box-sizing:content-box;")] ContentBox,
	#[strum(to_string = "box-sizing:border-box;")] BorderBox,
	#[strum(to_string = "box-sizing:initial;")] Initial,
	#[strum(to_string = "box-sizing:inherit;")] Inherit,
}

#[macro_export]
macro_rules! box_sizing {
	(content-box) => { $crate::css::Property::BoxSizing($crate::css::BoxSizing::ContentBox) };
	(border-box)  => { $crate::css::Property::BoxSizing($crate::css::BoxSizing::BorderBox) };
	(initial)     => { $crate::css::Property::BoxSizing($crate::css::BoxSizing::Initial) };
	(inherit)     => { $crate::css::Property::BoxSizing($crate::css::BoxSizing::Inherit) };
}
