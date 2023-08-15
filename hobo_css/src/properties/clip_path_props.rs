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
pub enum clip_path {
	none,
	initial,
	inherit,
	unset,
	url(String),
	shapes(Vec<ClipPathShape>),
}

impl clip_path {
	pub fn url(x: impl Into<String>) -> Self { Self::url(x.into()) }
	pub fn shape(x: ClipPathShape) -> Self { Self::shapes(vec![x]) }
}

impl std::fmt::Display for clip_path {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::none => "-webkit-clip-path:none;clip-path:none;".fmt(f),
			Self::initial => "-webkit-clip-path:initial;clip-path:initial;".fmt(f),
			Self::inherit => "-webkit-clip-path:inherit;clip-path:inherit;".fmt(f),
			Self::unset => "-webkit-clip-path:unset;clip-path:unset;".fmt(f),
			Self::url(x) => write!(f, r#"-webkit-clip-path:url("{x}");clip-path:url("{x}");"#),
			Self::shapes(shapes) => {
				if let Some((first, rest)) = shapes.split_first() {
					"-webkit-clip-path:".fmt(f)?;
					first.fmt(f)?;
					for shape in rest {
						write!(f, " {shape}")?;
					}
					";clip-path:".fmt(f)?;
					first.fmt(f)?;
					for shape in rest {
						write!(f, " {shape}")?;
					}
					";".fmt(f)?;
				}
				Ok(())
			},
		}
	}
}
