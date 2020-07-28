use crate::prelude::*;
pub type F32 = ordered_float::NotNan<f32>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, SmartDefault)]
pub enum Unit {
	#[default]
	Zero,
	Px(F32),
	Em(F32),
	Rem(F32),
	Vw(F32),
	Vh(F32),
	Vmin(F32),
	Vmax(F32),
	Fr(F32),
	Percent(F32),
	// TODO: calc?
}

#[rustfmt::skip]
impl std::fmt::Display for Unit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Zero       => "0".fmt(f),
			Self::Px(x)      => write!(f, "{}px", x),
			Self::Em(x)      => write!(f, "{}em", x),
			Self::Rem(x)     => write!(f, "{}rem", x),
			Self::Vw(x)      => write!(f, "{}vw", x),
			Self::Vh(x)      => write!(f, "{}vh", x),
			Self::Vmin(x)    => write!(f, "{}vmin", x),
			Self::Vmax(x)    => write!(f, "{}vmax", x),
			Self::Fr(x)      => write!(f, "{}fr", x),
			Self::Percent(x) => write!(f, "{}%", x),
		}
	}
}

#[rustfmt::skip]
#[macro_export]
macro_rules! unit {
	(0)                                      => { $crate::Unit::Zero };
	(expr = ($($e:tt)+))                     => { $crate::Unit::Px(unsafe {      $crate::units::F32::unchecked_new(($($e)+) as _) }) };
	(expr = ($($e:tt)+) px)                  => { $crate::Unit::Px(unsafe {      $crate::units::F32::unchecked_new(($($e)+) as _) }) };
	(expr = ($($e:tt)+) em)                  => { $crate::Unit::Em(unsafe {      $crate::units::F32::unchecked_new(($($e)+) as _) }) };
	(expr = ($($e:tt)+) rem)                 => { $crate::Unit::Rem(unsafe {     $crate::units::F32::unchecked_new(($($e)+) as _) }) };
	(expr = ($($e:tt)+) vw)                  => { $crate::Unit::Vw(unsafe {      $crate::units::F32::unchecked_new(($($e)+) as _) }) };
	(expr = ($($e:tt)+) vh)                  => { $crate::Unit::Vh(unsafe {      $crate::units::F32::unchecked_new(($($e)+) as _) }) };
	(expr = ($($e:tt)+) vmin)                => { $crate::Unit::Vmin(unsafe {    $crate::units::F32::unchecked_new(($($e)+) as _) }) };
	(expr = ($($e:tt)+) vmax)                => { $crate::Unit::Vmax(unsafe {    $crate::units::F32::unchecked_new(($($e)+) as _) }) };
	(expr = ($($e:tt)+) fr)                  => { $crate::Unit::Fr(unsafe {      $crate::units::F32::unchecked_new(($($e)+) as _) }) };
	(expr = ($($e:tt)+) %)                   => { $crate::Unit::Percent(unsafe { $crate::units::F32::unchecked_new(($($e)+) as _) }) };
	(expr = ($($e:tt)*) $tt:tt $($rest:tt)*) => { $crate::unit!(expr = ($($e)* $tt) $($rest)*) };
	($head:tt $($rest:tt)*)                  => { $crate::unit!(expr = ($head) $($rest)*) };
}
