use crate::prelude::*;
use num_traits::cast::AsPrimitive;

pub type F32 = ordered_float::NotNan<f32>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, strum::Display)]
pub enum Operator {
	#[strum(to_string = "+")] Plus,
	#[strum(to_string = "-")] Minus,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, SmartDefault)]
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
	Calc(Box<Unit>, Operator, Box<Unit>),
}

#[rustfmt::skip]
impl std::fmt::Display for Unit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Zero                  => "0".fmt(f),
			Self::Px(x)                 => write!(f, "{}px", x),
			Self::Em(x)                 => write!(f, "{}em", x),
			Self::Rem(x)                => write!(f, "{}rem", x),
			Self::Vw(x)                 => write!(f, "{}vw", x),
			Self::Vh(x)                 => write!(f, "{}vh", x),
			Self::Vmin(x)               => write!(f, "{}vmin", x),
			Self::Vmax(x)               => write!(f, "{}vmax", x),
			Self::Fr(x)                 => write!(f, "{}fr", x),
			Self::Percent(x)            => write!(f, "{}%", x),
			Self::Calc(left, op, right) => write!(f, "calc({} {} {})", left, op, right),
		}
	}
}

/// As pixels is the most common unit type, specifying `px` is optional
#[rustfmt::skip]
#[macro_export]
macro_rules! unit {
	(0)                                             => { $crate::units::Unit::Zero };

	($e:literal $(px)?)                             => { $crate::units::Unit::Px(                                  unsafe { $crate::units::F32::new_unchecked($e as _) }) };
	($e:literal $frag:ident)                        => { $crate::paste::item!{$crate::units::Unit::[<$frag:camel>](unsafe { $crate::units::F32::new_unchecked($e as _) })} };
	($e:literal %)                                  => { $crate::units::Unit::Percent(                             unsafe { $crate::units::F32::new_unchecked($e as _) }) };

	($e:ident $(px)?)                               => { $crate::units::px($e) };
	($e:ident $frag:ident)                          => { $crate::units::$frag($e) };
	($e:ident %)                                    => { $crate::units::pct($e) };

	(($($e:tt)+) $(px)?)                            => { $crate::units::px($($e)+) };
	(($($e:tt)+) $frag:ident)                       => { $crate::units::$frag($($e)+) };
	(($($e:tt)+) %)                                 => { $crate::units::pct($($e)+) };

	($e1:literal $frag1:tt + $e2:literal $frag2:tt) => { $crate::units::Unit::Calc(Box::new($crate::unit!($e1 $frag1)), $crate::units::Operator::Plus,  Box::new($crate::unit!($e2 $frag2))) };
	($e1:ident   $frag1:tt + $e2:literal $frag2:tt) => { $crate::units::Unit::Calc(Box::new($crate::unit!($e1 $frag1)), $crate::units::Operator::Plus,  Box::new($crate::unit!($e2 $frag2))) };
	($e1:literal $frag1:tt + $e2:ident   $frag2:tt) => { $crate::units::Unit::Calc(Box::new($crate::unit!($e1 $frag1)), $crate::units::Operator::Plus,  Box::new($crate::unit!($e2 $frag2))) };
	($e1:ident   $frag1:tt + $e2:ident   $frag2:tt) => { $crate::units::Unit::Calc(Box::new($crate::unit!($e1 $frag1)), $crate::units::Operator::Plus,  Box::new($crate::unit!($e2 $frag2))) };

	($e1:literal $frag1:tt - $e2:literal $frag2:tt) => { $crate::units::Unit::Calc(Box::new($crate::unit!($e1 $frag1)), $crate::units::Operator::Minus, Box::new($crate::unit!($e2 $frag2))) };
	($e1:ident   $frag1:tt - $e2:literal $frag2:tt) => { $crate::units::Unit::Calc(Box::new($crate::unit!($e1 $frag1)), $crate::units::Operator::Minus, Box::new($crate::unit!($e2 $frag2))) };
	($e1:literal $frag1:tt - $e2:ident   $frag2:tt) => { $crate::units::Unit::Calc(Box::new($crate::unit!($e1 $frag1)), $crate::units::Operator::Minus, Box::new($crate::unit!($e2 $frag2))) };
	($e1:ident   $frag1:tt - $e2:ident   $frag2:tt) => { $crate::units::Unit::Calc(Box::new($crate::unit!($e1 $frag1)), $crate::units::Operator::Minus, Box::new($crate::unit!($e2 $frag2))) };
}

pub fn px<T: AsPrimitive<f32>>(x: T)   -> Unit { Unit::Px(     F32::new(x.as_()).unwrap()) }
pub fn em<T: AsPrimitive<f32>>(x: T)   -> Unit { Unit::Em(     F32::new(x.as_()).unwrap()) }
pub fn rem<T: AsPrimitive<f32>>(x: T)  -> Unit { Unit::Rem(    F32::new(x.as_()).unwrap()) }
pub fn vw<T: AsPrimitive<f32>>(x: T)   -> Unit { Unit::Vw(     F32::new(x.as_()).unwrap()) }
pub fn vh<T: AsPrimitive<f32>>(x: T)   -> Unit { Unit::Vh(     F32::new(x.as_()).unwrap()) }
pub fn vmin<T: AsPrimitive<f32>>(x: T) -> Unit { Unit::Vmin(   F32::new(x.as_()).unwrap()) }
pub fn vmax<T: AsPrimitive<f32>>(x: T) -> Unit { Unit::Vmax(   F32::new(x.as_()).unwrap()) }
pub fn fr<T: AsPrimitive<f32>>(x: T)   -> Unit { Unit::Fr(     F32::new(x.as_()).unwrap()) }
pub fn pct<T: AsPrimitive<f32>>(x: T)  -> Unit { Unit::Percent(F32::new(x.as_()).unwrap()) }
