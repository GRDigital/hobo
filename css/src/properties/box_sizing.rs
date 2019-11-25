#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, strum_macros::Display)]
pub enum BoxSizing {
	#[strum(to_string = "box-sizing:content-box;")] ContentBox,
	#[strum(to_string = "box-sizing:border-box;")] BorderBox,
	#[strum(to_string = "box-sizing:initial;")] Initial,
	#[strum(to_string = "box-sizing:inherit;")] Inherit,
}

#[macro_export]
macro_rules! box_sizing {
	(content-box) => { $crate::Property::BoxSizing($crate::BoxSizing::ContentBox) };
	(border-box)  => { $crate::Property::BoxSizing($crate::BoxSizing::BorderBox) };
	(initial)     => { $crate::Property::BoxSizing($crate::BoxSizing::Initial) };
	(inherit)     => { $crate::Property::BoxSizing($crate::BoxSizing::Inherit) };
}
