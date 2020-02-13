use crate::BasicShape;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, strum::Display)]
pub enum GeometryBox {
	#[strum(to_string = "margin-box")] MarginBox,
	#[strum(to_string = "border-box")] BorderBox,
	#[strum(to_string = "padding-box")] PaddingBox,
	#[strum(to_string = "content-box")] ContentBox,
	#[strum(to_string = "fill-box")] FillBox,
	#[strum(to_string = "stroke-box")] StrokeBox,
	#[strum(to_string = "view-box")] ViewBox,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ClipPathShape {
	BasicShape(BasicShape),
	GeometryBox(GeometryBox),
}

impl ToString for ClipPathShape {
	fn to_string(&self) -> String {
		match self {
			Self::BasicShape(x) => x.to_string(),
			Self::GeometryBox(x) => x.to_string(),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ClipPath {
	None,
	Initial,
	Inherit,
	Unset,
	Revert,
	Url(String),
	Shape(Vec<ClipPathShape>),
}

impl ToString for ClipPath {
	fn to_string(&self) -> String {
		match self {
			Self::None => "clip-path:none;".to_owned(),
			Self::Initial => "clip-path:initial;".to_owned(),
			Self::Inherit => "clip-path:inherit;".to_owned(),
			Self::Unset => "clip-path:unset;".to_owned(),
			Self::Revert => "clip-path:revert;".to_owned(),
			Self::Url(x) => format!(r#"clip-path:url("{}");"#, x),
			Self::Shape(x) => format!("clip-path:{};", x.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(" ")),
		}
	}
}
