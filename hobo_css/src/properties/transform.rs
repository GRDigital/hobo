use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, SmartDefault, Clone, PartialOrd, Ord)]
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

#[derive(Debug, PartialEq, Eq, Hash, SmartDefault, Clone, PartialOrd, Ord)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
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

impl TransformFunction {
	pub fn matrix(a1: f32, b1: f32, a2: f32, b2: f32, a3: f32, b3: f32) -> Self { Self::Matrix(F32::new(a1).unwrap(), F32::new(b1).unwrap(), F32::new(a2).unwrap(), F32::new(b2).unwrap(), F32::new(a3).unwrap(), F32::new(b3).unwrap()) }
	pub fn matrix_3d(a1: f32, b1: f32, c1: f32, d1: f32, a2: f32, b2: f32, c2: f32, d2: f32, a3: f32, b3: f32, c3: f32, d3: f32, a4: f32, b4: f32, c4: f32, d4: f32) -> Self { Self::Matrix3d(F32::new(a1).unwrap(), F32::new(b1).unwrap(), F32::new(c1).unwrap(), F32::new(d1).unwrap(), F32::new(a2).unwrap(), F32::new(b2).unwrap(), F32::new(c2).unwrap(), F32::new(d2).unwrap(), F32::new(a3).unwrap(), F32::new(b3).unwrap(), F32::new(c3).unwrap(), F32::new(d3).unwrap(), F32::new(a4).unwrap(), F32::new(b4).unwrap(), F32::new(c4).unwrap(), F32::new(d4).unwrap()) }
	pub fn translate_x(x: Unit) -> Self { Self::TranslateX(x) }
	pub fn translate_y(x: Unit) -> Self { Self::TranslateY(x) }
	pub fn translate_z(x: Unit) -> Self { Self::TranslateZ(x) }
	pub fn scale_x(x: f32) -> Self { Self::ScaleX(F32::new(x).unwrap()) }
	pub fn scale_y(x: f32) -> Self { Self::ScaleY(F32::new(x).unwrap()) }
	pub fn scale_z(x: f32) -> Self { Self::ScaleZ(F32::new(x).unwrap()) }
	pub fn rotate_x(x: f32) -> Self { Self::RotateX(F32::new(x).unwrap()) }
	pub fn rotate_y(x: f32) -> Self { Self::RotateY(F32::new(x).unwrap()) }
	pub fn rotate_z(x: f32) -> Self { Self::RotateZ(F32::new(x).unwrap()) }
	pub fn skew_x(x: f32) -> Self { Self::SkewX(F32::new(x).unwrap()) }
	pub fn skew_y(x: f32) -> Self { Self::SkewY(F32::new(x).unwrap()) }
	pub fn skew_z(x: f32) -> Self { Self::SkewZ(F32::new(x).unwrap()) }
	pub fn perspective(x: Unit) -> Self { Self::Perspective(x) }
}

impl crate::AppendProperty for TransformFunction {
	fn append_property(self, properties: &mut Vec<crate::Property>) {
		Transform::Some(vec![self]).append_property(properties)
	}
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
