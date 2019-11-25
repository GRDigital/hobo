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
	(visible)  => { $crate::Property::Visibility($crate::Visibility::Visible) };
	(hidden)   => { $crate::Property::Visibility($crate::Visibility::Hidden) };
	(collapse) => { $crate::Property::Visibility($crate::Visibility::Collapse) };
	(initial)  => { $crate::Property::Visibility($crate::Visibility::Initial) };
	(inherit)  => { $crate::Property::Visibility($crate::Visibility::Inherit) };
}
