use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone)]
pub enum Transform {
	#[default]
	None,
	Initial,
	Inherit,
	Some(Vec<TransformFunction>),
}

#[rustfmt::skip]
impl ToString for Transform {
	fn to_string(&self) -> String {
		match self {
			Self::None       => "transform:none;".to_owned(),
			Self::Initial    => "transform:initial;".to_owned(),
			Self::Inherit    => "transform:inherit;".to_owned(),
			Self::Some(fns)  => format!("transform:{};", fns.iter().map(ToString::to_string).collect::<Vec<_>>().join(" ")),
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

impl ToString for TransformFunction {
	fn to_string(&self) -> String {
		match self {
			Self::Matrix(a1, b1, a2, b2, a3, b3) => format!("matrix({}, {}, {}, {}, {}, {})", a1, b1, a2, b2, a3, b3),
			Self::Matrix3d(a1, b1, c1, d1, a2, b2, c2, d2, a3, b3, c3, d3, a4, b4, c4, d4) => format!("matrix3d({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})", a1, b1, c1, d1, a2, b2, c2, d2, a3, b3, c3, d3, a4, b4, c4, d4),
			Self::TranslateX(x) => format!("translateX({})", x.to_string()),
			Self::TranslateY(x) => format!("translateY({})", x.to_string()),
			Self::TranslateZ(x) => format!("translateZ({})", x.to_string()),
			Self::ScaleX(x) => format!("scaleX({})", x),
			Self::ScaleY(x) => format!("scaleY({})", x),
			Self::ScaleZ(x) => format!("scaleZ({})", x),
			Self::RotateX(x) => format!("rotateX({}deg)", x),
			Self::RotateY(x) => format!("rotateY({}deg)", x),
			Self::RotateZ(x) => format!("rotateZ({}deg)", x),
			Self::SkewX(x) => format!("skewX({})", x),
			Self::SkewY(x) => format!("skewY({})", x),
			Self::SkewZ(x) => format!("skewZ({})", x),
			Self::Perspective(x) => format!("perspective({})", x.to_string()),
		}
	}
}
