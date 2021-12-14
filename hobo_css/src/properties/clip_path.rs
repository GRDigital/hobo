use crate::BasicShape;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, strum::Display, PartialOrd, Ord)]
pub enum GeometryBox {
	#[strum(to_string = "margin-box")] MarginBox,
	#[strum(to_string = "border-box")] BorderBox,
	#[strum(to_string = "padding-box")] PaddingBox,
	#[strum(to_string = "content-box")] ContentBox,
	#[strum(to_string = "fill-box")] FillBox,
	#[strum(to_string = "stroke-box")] StrokeBox,
	#[strum(to_string = "view-box")] ViewBox,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum ClipPathShape {
	BasicShape(BasicShape),
	GeometryBox(GeometryBox),
}

impl std::fmt::Display for ClipPathShape {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::BasicShape(x) => x.fmt(f),
			Self::GeometryBox(x) => x.fmt(f),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum ClipPath {
	None,
	Initial,
	Inherit,
	Unset,
	Url(String),
	Shape(Vec<ClipPathShape>),
}

impl std::fmt::Display for ClipPath {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::None => "-webkit-clip-path:none;clip-path:none;".fmt(f),
			Self::Initial => "-webkit-clip-path:initial;clip-path:initial;".fmt(f),
			Self::Inherit => "-webkit-clip-path:inherit;clip-path:inherit;".fmt(f),
			Self::Unset => "-webkit-clip-path:unset;clip-path:unset;".fmt(f),
			Self::Url(x) => write!(f, r#"-webkit-clip-path:url("{0}");clip-path:url("{0}");"#, x),
			Self::Shape(shapes) => {
				if let Some((first, rest)) = shapes.split_first() {
					"-webkit-clip-path:".fmt(f)?;
					first.fmt(f)?;
					for shape in rest {
						write!(f, " {}", shape)?;
					}
					";clip-path:".fmt(f)?;
					first.fmt(f)?;
					for shape in rest {
						write!(f, " {}", shape)?;
					}
					";".fmt(f)?;
				}
				Ok(())
			},
		}
	}
}
