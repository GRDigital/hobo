#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, strum_macros::Display)]
pub enum Position {
	#[strum(to_string = "position:static;")] Static,
	#[strum(to_string = "position:absolute;")] Absolute,
	#[strum(to_string = "position:fixed;")] Fixed,
	#[strum(to_string = "position:relative;")] Relative,
	#[strum(to_string = "position:sticky;")] Sticky,
	#[strum(to_string = "position:initial;")] Initial,
	#[strum(to_string = "position:inherit;")] Inherit,
}

#[macro_export]
macro_rules! position {
	(static)   => { $crate::css::Property::Position($crate::css::Position::Static) };
	(absolute) => { $crate::css::Property::Position($crate::css::Position::Absolute) };
	(fixed)    => { $crate::css::Property::Position($crate::css::Position::Fixed) };
	(relative) => { $crate::css::Property::Position($crate::css::Position::Relative) };
	(sticky)   => { $crate::css::Property::Position($crate::css::Position::Sticky) };
	(initial)  => { $crate::css::Property::Position($crate::css::Position::Initial) };
	(inherit)  => { $crate::css::Property::Position($crate::css::Position::Inherit) };
}

#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone, Copy)]
pub enum ZIndex {
	#[default]
	Auto,
	Initial,
	Inherit,
	Some(i32),
}

impl ToString for ZIndex {
	fn to_string(&self) -> String {
		match self {
			Self::Auto       => "z-index:auto;".to_owned(),
			Self::Initial    => "z-index:initial;".to_owned(),
			Self::Inherit    => "z-index:inherit;".to_owned(),
			Self::Some(x)    => format!("z-index:{};", x),
		}
	}
}

#[macro_export]
macro_rules! z_index {
	(auto)       => { $crate::css::Property::Position($crate::css::Position::Auto) };
	($some:expr) => { $crate::css::Property::Position($crate::css::Position::Some($some)) };
	(initial)    => { $crate::css::Property::Position($crate::css::Position::Initial) };
	(inherit)    => { $crate::css::Property::Position($crate::css::Position::Inherit) };
}
