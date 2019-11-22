#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, strum_macros::Display)]
pub enum Visibility {
	#[strum(to_string = "visibility:visible;")] Visible,
	#[strum(to_string = "visibility:hidden;")] Hidden,
	#[strum(to_string = "visibility:collapse;")] Collapse,
	#[strum(to_string = "visibility:initial;")] Initial,
	#[strum(to_string = "visibility:inherit;")] Inherit,
}

#[macro_export]
macro_rules! visibility {
	(visible)  => { $crate::css::Property::Visibility($crate::css::Visibility::Visible) };
	(hidden)   => { $crate::css::Property::Visibility($crate::css::Visibility::Hidden) };
	(collapse) => { $crate::css::Property::Visibility($crate::css::Visibility::Collapse) };
	(initial)  => { $crate::css::Property::Visibility($crate::css::Visibility::Initial) };
	(inherit)  => { $crate::css::Property::Visibility($crate::css::Visibility::Inherit) };
}
