#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, strum_macros::Display)]
pub enum Display {
	#[strum(to_string = "display:none;")] None,
	#[strum(to_string = "display:inline;")] Inline,
	#[strum(to_string = "display:block;")] Block,
	#[strum(to_string = "display:inline-block;")] InlineBlock,
	#[strum(to_string = "display:flex;")] Flex,
	#[strum(to_string = "display:inline-flex;")] InlineFlex,
	#[strum(to_string = "display:initial;")] Initial,
	#[strum(to_string = "display:inherit;")] Inherit,
}

#[macro_export]
macro_rules! display {
	(none)         => { $crate::css::Property::Display($crate::css::Display::None) };
	(block)        => { $crate::css::Property::Display($crate::css::Display::Block) };
	(inline-block) => { $crate::css::Property::Display($crate::css::Display::InlineBlock) };
	(inherit)      => { $crate::css::Property::Display($crate::css::Display::Inherit) };
	(initial)      => { $crate::css::Property::Display($crate::css::Display::Initial) };
	(flex)         => { $crate::css::Property::Display($crate::css::Display::Flex) };
	(inline-flex)  => { $crate::css::Property::Display($crate::css::Display::InlineFlex) };
}
