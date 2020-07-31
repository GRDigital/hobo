use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, SmartDefault, Clone)]
pub enum TransformOrigin {
	#[default]
	None,
	Initial,
	Inherit,
	Unset,
	Some(Unit, Unit),
}

#[rustfmt::skip]
impl std::fmt::Display for TransformOrigin {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::None              => "transform-origin:none;".fmt(f),
			Self::Initial           => "transform-origin:initial;".fmt(f),
			Self::Inherit           => "transform-origin:inherit;".fmt(f),
			Self::Unset             => "transform-origin:unset;".fmt(f),
			Self::Some(top, bottom) => write!(f, "transform-origin:{} {};", top, bottom),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, SmartDefault, Clone)]
pub enum Transform {
	#[default]
	None,
	Initial,
	Inherit,
	Unset,
	Some(Vec<TransformFunction>),
}

#[rustfmt::skip]
impl std::fmt::Display for Transform {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::None       => "transform:none;".fmt(f),
			Self::Initial    => "transform:initial;".fmt(f),
			Self::Inherit    => "transform:inherit;".fmt(f),
			Self::Unset      => "transform:unset;".fmt(f),
			Self::Some(fns)  => {
				"transform:".fmt(f)?;
				if let Some((first, rest)) = fns.split_first() {
					write!(f, "{}", first)?;
					for func in rest {
						write!(f, " {}", func)?;
					}
				}
				";".fmt(f)
			},
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TransformFunction {
	Matrix(F32, F32, F32, F32, F32, F32),
	Matrix3d(F32, F32, F32, F32, F32, F32, F32, F32, F32, F32, F32, F32, F32, F32, F32, F32),
	TranslateX(Unit),
	TranslateY(Unit),
	TranslateZ(Unit),
	ScaleX(F32),
	ScaleY(F32),
	ScaleZ(F32),
	RotateX(F32),
	RotateY(F32),
	RotateZ(F32),
	SkewX(F32),
	SkewY(F32),
	SkewZ(F32),
	Perspective(Unit),
}

impl std::fmt::Display for TransformFunction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Matrix(a1, b1, a2, b2, a3, b3) => write!(f, "matrix({}, {}, {}, {}, {}, {})", a1, b1, a2, b2, a3, b3),
			Self::Matrix3d(a1, b1, c1, d1, a2, b2, c2, d2, a3, b3, c3, d3, a4, b4, c4, d4) => write!(f, "matrix3d({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})", a1, b1, c1, d1, a2, b2, c2, d2, a3, b3, c3, d3, a4, b4, c4, d4),
			Self::TranslateX(x) => write!(f, "translateX({})", x),
			Self::TranslateY(x) => write!(f, "translateY({})", x),
			Self::TranslateZ(x) => write!(f, "translateZ({})", x),
			Self::ScaleX(x) => write!(f, "scaleX({})", x),
			Self::ScaleY(x) => write!(f, "scaleY({})", x),
			Self::ScaleZ(x) => write!(f, "scaleZ({})", x),
			Self::RotateX(x) => write!(f, "rotateX({}deg)", x),
			Self::RotateY(x) => write!(f, "rotateY({}deg)", x),
			Self::RotateZ(x) => write!(f, "rotateZ({}deg)", x),
			Self::SkewX(x) => write!(f, "skewX({})", x),
			Self::SkewY(x) => write!(f, "skewY({})", x),
			Self::SkewZ(x) => write!(f, "skewZ({})", x),
			Self::Perspective(x) => write!(f, "perspective({})", x),
		}
	}
}
