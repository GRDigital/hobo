#[rustfmt::skip]
#[macro_export]
#[doc(hidden)]
macro_rules! __padding {
	($side:ident, 0)           => {$crate::paste::item!{ $crate::Property::[<Padding $side>]($crate::UnitValue::Zero) }};
	($side:ident, initial)     => {$crate::paste::item!{ $crate::Property::[<Padding $side>]($crate::UnitValue::Initial) }};
	($side:ident, inherit)     => {$crate::paste::item!{ $crate::Property::[<Padding $side>]($crate::UnitValue::Inherit) }};
	($side:ident, unset)       => {$crate::paste::item!{ $crate::Property::[<Padding $side>]($crate::UnitValue::Unset) }};
	($side:ident, revert)      => {$crate::paste::item!{ $crate::Property::[<Padding $side>]($crate::UnitValue::Revert) }};
	($side:ident, $($val:tt)+) => {$crate::paste::item!{ $crate::Property::[<Padding $side>]($crate::UnitValue::Unit($crate::unit!($($val)+))) }};
}

#[macro_export] macro_rules! padding_left { ($($tt:tt)+) => { $crate::__padding!(Left, $($tt)+)} }
#[macro_export] macro_rules! padding_right { ($($tt:tt)+) => { $crate::__padding!(Right, $($tt)+)} }
#[macro_export] macro_rules! padding_top { ($($tt:tt)+) => { $crate::__padding!(Top, $($tt)+)} }
#[macro_export] macro_rules! padding_bottom { ($($tt:tt)+) => { $crate::__padding!(Bottom, $($tt)+)} }
