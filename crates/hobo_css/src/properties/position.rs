crate::macros::easy_enum! {position static absolute fixed relative sticky}
crate::macros::easy_enum! {z-index auto [number]}

#[macro_export] macro_rules! top { ($($tt:tt)+) => {$crate::__dimension!(Top, $($tt)+)} }
#[macro_export] macro_rules! right { ($($tt:tt)+) => {$crate::__dimension!(Right, $($tt)+)} }
#[macro_export] macro_rules! bottom { ($($tt:tt)+) => {$crate::__dimension!(Bottom, $($tt)+)} }
#[macro_export] macro_rules! left { ($($tt:tt)+) => {$crate::__dimension!(Left, $($tt)+)} }
