#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, strum_macros::Display)]
pub enum Overflow {
	#[strum(to_string = "overflow:visible;")] Visible,
	#[strum(to_string = "overflow:hidden;")] Hidden,
	#[strum(to_string = "overflow:scroll;")] Scroll,
	#[strum(to_string = "overflow:auto;")] Auto,
	#[strum(to_string = "overflow:initial;")] Initial,
	#[strum(to_string = "overflow:inherit;")] Inherit,
}

#[macro_export]
macro_rules! overflow {
	(visible) => { $crate::Property::Overflow($crate::Overflow::Visible) };
	(hidden)  => { $crate::Property::Overflow($crate::Overflow::Hidden) };
	(scroll)  => { $crate::Property::Overflow($crate::Overflow::Scroll) };
	(auto)    => { $crate::Property::Overflow($crate::Overflow::Auto) };
	(initial) => { $crate::Property::Overflow($crate::Overflow::Initial) };
	(inherit) => { $crate::Property::Overflow($crate::Overflow::Inherit) };
}
