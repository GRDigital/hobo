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
	(none)         => { $crate::Property::Display($crate::Display::None) };
	(block)        => { $crate::Property::Display($crate::Display::Block) };
	(inline-block) => { $crate::Property::Display($crate::Display::InlineBlock) };
	(inherit)      => { $crate::Property::Display($crate::Display::Inherit) };
	(initial)      => { $crate::Property::Display($crate::Display::Initial) };
	(flex)         => { $crate::Property::Display($crate::Display::Flex) };
	(inline-flex)  => { $crate::Property::Display($crate::Display::InlineFlex) };
}
