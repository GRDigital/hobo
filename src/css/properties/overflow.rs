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
	(visible) => { $crate::css::Property::Overflow($crate::css::Overflow::Visible) };
	(hidden)  => { $crate::css::Property::Overflow($crate::css::Overflow::Hidden) };
	(scroll)  => { $crate::css::Property::Overflow($crate::css::Overflow::Scroll) };
	(auto)    => { $crate::css::Property::Overflow($crate::css::Overflow::Auto) };
	(initial) => { $crate::css::Property::Overflow($crate::css::Overflow::Initial) };
	(inherit) => { $crate::css::Property::Overflow($crate::css::Overflow::Inherit) };
}
