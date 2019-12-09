css_macros::easy_enum!{position static absolute fixed relative sticky}

#[macro_export] macro_rules! top { ($($tt:tt)+) => {$crate::__dimension!(Top, $($tt)+)} }
#[macro_export] macro_rules! right { ($($tt:tt)+) => {$crate::__dimension!(Right, $($tt)+)} }
#[macro_export] macro_rules! bottom { ($($tt:tt)+) => {$crate::__dimension!(Bottom, $($tt)+)} }
#[macro_export] macro_rules! left { ($($tt:tt)+) => {$crate::__dimension!(Left, $($tt)+)} }

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
