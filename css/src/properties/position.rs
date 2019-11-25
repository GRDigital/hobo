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
	(static)   => { $crate::Property::Position($crate::Position::Static) };
	(absolute) => { $crate::Property::Position($crate::Position::Absolute) };
	(fixed)    => { $crate::Property::Position($crate::Position::Fixed) };
	(relative) => { $crate::Property::Position($crate::Position::Relative) };
	(sticky)   => { $crate::Property::Position($crate::Position::Sticky) };
	(initial)  => { $crate::Property::Position($crate::Position::Initial) };
	(inherit)  => { $crate::Property::Position($crate::Position::Inherit) };
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
	(auto)       => { $crate::Property::ZIndex($crate::ZIndex::Auto) };
	($some:expr) => { $crate::Property::ZIndex($crate::ZIndex::Some($some)) };
	(initial)    => { $crate::Property::ZIndex($crate::ZIndex::Initial) };
	(inherit)    => { $crate::Property::ZIndex($crate::ZIndex::Inherit) };
}
