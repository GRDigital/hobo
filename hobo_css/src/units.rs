use num_traits::cast::AsPrimitive;

pub type F32 = ordered_float::NotNan<f32>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, strum::Display, PartialOrd, Ord)]
pub enum Operator {
	#[strum(to_string = "+")] Plus,
	#[strum(to_string = "-")] Minus,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default, PartialOrd, Ord)]
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
	// probably a hack
	Duration(F32), // TODO: chrono duration?
	Calc(Box<Unit>, Operator, Box<Unit>),
}

impl Unit {
	#[inline] pub fn calc(left: Self, op: Operator, right: Self) -> Self { Self::Calc(Box::new(left), op, Box::new(right)) }
	#[inline] pub fn px<T: AsPrimitive<f32>>(x: T)   -> Self { Self::Px(      F32::new(x.as_()).unwrap()) }
	#[inline] pub fn em<T: AsPrimitive<f32>>(x: T)   -> Self { Self::Em(      F32::new(x.as_()).unwrap()) }
	#[inline] pub fn rem<T: AsPrimitive<f32>>(x: T)  -> Self { Self::Rem(     F32::new(x.as_()).unwrap()) }
	#[inline] pub fn vw<T: AsPrimitive<f32>>(x: T)   -> Self { Self::Vw(      F32::new(x.as_()).unwrap()) }
	#[inline] pub fn vh<T: AsPrimitive<f32>>(x: T)   -> Self { Self::Vh(      F32::new(x.as_()).unwrap()) }
	#[inline] pub fn vmin<T: AsPrimitive<f32>>(x: T) -> Self { Self::Vmin(    F32::new(x.as_()).unwrap()) }
	#[inline] pub fn vmax<T: AsPrimitive<f32>>(x: T) -> Self { Self::Vmax(    F32::new(x.as_()).unwrap()) }
	#[inline] pub fn fr<T: AsPrimitive<f32>>(x: T)   -> Self { Self::Fr(      F32::new(x.as_()).unwrap()) }
	#[inline] pub fn pct<T: AsPrimitive<f32>>(x: T)  -> Self { Self::Percent( F32::new(x.as_()).unwrap()) }
	#[inline] pub fn dur<T: AsPrimitive<f32>>(x: T)  -> Self { Self::Duration(F32::new(x.as_()).unwrap()) }
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
			Self::Duration(x)           => write!(f, "{}ms", x),
			Self::Calc(left, op, right) => write!(f, "calc({} {} {})", left, op, right),
		}
	}
}

impl std::ops::Add for Unit {
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		match (self, rhs) {
			(Self::Zero, Self::Zero) => Self::Zero,
			(Self::Px(a), Self::Px(b)) => Self::Px(a + b),
			(Self::Em(a), Self::Em(b)) => Self::Em(a + b),
			(Self::Rem(a), Self::Rem(b)) => Self::Rem(a + b),
			(Self::Vw(a), Self::Vw(b)) => Self::Vw(a + b),
			(Self::Vh(a), Self::Vh(b)) => Self::Vh(a + b),
			(Self::Vmin(a), Self::Vmin(b)) => Self::Vmin(a + b),
			(Self::Vmax(a), Self::Vmax(b)) => Self::Vmax(a + b),
			(Self::Fr(a), Self::Fr(b)) => Self::Fr(a + b),
			(Self::Percent(a), Self::Percent(b)) => Self::Percent(a + b),
			(Self::Duration(a), Self::Duration(b)) => Self::Duration(a + b),
			(a, b) => Self::Calc(Box::new(a), Operator::Plus, Box::new(b)),
		}
	}
}

impl std::ops::Sub for Unit {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self {
		match (self, rhs) {
			(Self::Zero, Self::Zero) => Self::Zero,
			(Self::Px(a), Self::Px(b)) => Self::Px(a - b),
			(Self::Em(a), Self::Em(b)) => Self::Em(a - b),
			(Self::Rem(a), Self::Rem(b)) => Self::Rem(a - b),
			(Self::Vw(a), Self::Vw(b)) => Self::Vw(a - b),
			(Self::Vh(a), Self::Vh(b)) => Self::Vh(a - b),
			(Self::Vmin(a), Self::Vmin(b)) => Self::Vmin(a - b),
			(Self::Vmax(a), Self::Vmax(b)) => Self::Vmax(a - b),
			(Self::Fr(a), Self::Fr(b)) => Self::Fr(a - b),
			(Self::Percent(a), Self::Percent(b)) => Self::Percent(a - b),
			(Self::Duration(a), Self::Duration(b)) => Self::Duration(a - b),
			(a, b) => Self::Calc(Box::new(a), Operator::Minus, Box::new(b)),
		}
	}
}

/// As pixels is the most common unit type, specifying `px` is optional
#[rustfmt::skip]
#[macro_export]
macro_rules! unit {
	(0)                       => { $crate::units::Unit::Zero                        };

	($e:literal $(px)?)       => { $crate::units::Unit::px($e)                      };
	($e:literal ms)           => { $crate::units::Unit::dur($e)                     };
	($e:literal $frag:ident)  => { $crate::units::Unit::$frag($e)                   };
	($e:literal %)            => { $crate::units::Unit::pct($e)       };

	($e:ident $(px)?)         => { $crate::units::Unit::px($e)                      };
	($e:ident ms)             => { $crate::units::Unit::dur($e)                     };
	($e:ident $frag:ident)    => { $crate::units::Unit::$frag($e)                   };
	($e:ident %)              => { $crate::units::Unit::pct($e)       };

	// this so you can use a more complex expression by wrapping it in parens
	(($($e:tt)+) $(px)?)      => { $crate::units::Unit::px($($e)+)                  };
	(($($e:tt)+) ms)          => { $crate::units::Unit::dur($($e)+)                 };
	(($($e:tt)+) $frag:ident) => { $crate::units::Unit::$frag($($e)+)               };
	(($($e:tt)+) %)           => { $crate::units::Unit::pct($($e)+) };

	// base case
	(($($e1:tt)+) $frag1:tt + ($($e2:tt)+) $frag2:tt) => { $crate::units::Unit::calc($crate::unit!(($($e1)+) $frag1), $crate::units::Operator::Plus, $crate::unit!(($($e2)+) $frag2)) };
	(($($e1:tt)+) $frag1:tt - ($($e2:tt)+) $frag2:tt) => { $crate::units::Unit::calc($crate::unit!(($($e1)+) $frag1), $crate::units::Operator::Minus, $crate::unit!(($($e2)+) $frag2)) };

	// convert from parens-less to base case
	(($($e1:tt)+) $frag1:tt + $e2:tt $frag2:tt) => { $crate::unit!(($($e1)+) $frag1 + ($e2) $frag2) };
	($e1:tt $frag1:tt + ($($e2:tt)+) $frag2:tt) => { $crate::unit!(($e1) $frag1 + ($($e2)+) $frag2) };
	($e1:tt $frag1:tt + $e2:tt $frag2:tt)       => { $crate::unit!(($e1) $frag1 + ($e2) $frag2) };

	(($($e1:tt)+) $frag1:tt - $e2:tt $frag2:tt) => { $crate::unit!(($($e1)+) $frag1 - ($e2) $frag2) };
	($e1:tt $frag1:tt - ($($e2:tt)+) $frag2:tt) => { $crate::unit!(($e1) $frag1 - ($($e2)+) $frag2) };
	($e1:tt $frag1:tt - $e2:tt $frag2:tt)       => { $crate::unit!(($e1) $frag1 - ($e2) $frag2) };
}
