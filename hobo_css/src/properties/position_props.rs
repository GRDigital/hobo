crate::macros::easy_enum! {position static absolute fixed relative sticky}
crate::macros::easy_enum! {z-index auto [number]}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum PositionOffset {
	Auto,
	Initial,
	Inherit,
	Unset,
	Some(crate::Unit),
}

#[rustfmt::skip]
impl std::fmt::Display for PositionOffset {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Auto       => "auto".fmt(f),
			Self::Initial    => "initial".fmt(f),
			Self::Inherit    => "inherit".fmt(f),
			Self::Unset      => "unset".fmt(f),
			Self::Some(unit) => unit.fmt(f),
		}
	}
}

#[rustfmt::skip]
#[macro_export]
#[doc(hidden)]
macro_rules! __position {
	($kind:ident, auto)        => {$crate::Property::$kind($crate::PositionOffset::Auto)};
	($kind:ident, initial)     => {$crate::Property::$kind($crate::PositionOffset::Initial)};
	($kind:ident, inherit)     => {$crate::Property::$kind($crate::PositionOffset::Inherit)};
	($kind:ident, unset)       => {$crate::Property::$kind($crate::PositionOffset::Unset)};
	($kind:ident, $($val:tt)+) => {$crate::Property::$kind($crate::PositionOffset::Some($crate::unit!($($val)+)))};
}

#[macro_export] macro_rules! top { ($($tt:tt)+) => {$crate::__position!(Top, $($tt)+)} }
#[macro_export] macro_rules! right { ($($tt:tt)+) => {$crate::__position!(Right, $($tt)+)} }
#[macro_export] macro_rules! bottom { ($($tt:tt)+) => {$crate::__position!(Bottom, $($tt)+)} }
#[macro_export] macro_rules! left { ($($tt:tt)+) => {$crate::__position!(Left, $($tt)+)} }
