use crate::prelude::*;

// TODO: replace @font-face selector with regular rust struct

#[derive(SmartDefault, strum::Display, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Display {
	#[default]
	#[strum(to_string = "auto")] Auto,
	#[strum(to_string = "block")] Block,
	#[strum(to_string = "swap")] Swap,
	#[strum(to_string = "fallback")] Fallback,
	#[strum(to_string = "optional")] Optional,
}

#[derive(SmartDefault, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Stretch {
	UltraCondensed,
	ExtraCondensed,
	Condensed,
	SemiCondensed,
	#[default]
	Normal,
	SemiExpanded,
	Expanded,
	ExtraExpanded,
	UltraExpanded,
	Percentage(F32),
}

impl std::fmt::Display for Stretch {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::UltraCondensed => write!(f, "ultra-condensed"),
			Self::ExtraCondensed => write!(f, "extra-condensed"),
			Self::Condensed => write!(f, "condensed"),
			Self::SemiCondensed => write!(f, "semi-condensed"),
			Self::Normal => write!(f, "normal"),
			Self::SemiExpanded => write!(f, "semi-expanded"),
			Self::Expanded => write!(f, "expanded"),
			Self::ExtraExpanded => write!(f, "extra-expanded"),
			Self::UltraExpanded => write!(f, "ultra-expanded"),
			Self::Percentage(x) => write!(f, "{}%", x),
		}
	}
}

#[derive(SmartDefault,  Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Style {
	#[default]
	Normal,
	Italic,
	Oblique,
	ObliqueAngle(F32),
	ObliqueAngleRange(F32, F32),
}

impl std::fmt::Display for Style {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Normal => write!(f, "normal"),
			Self::Italic => write!(f, "italic"),
			Self::Oblique => write!(f, "oblique"),
			Self::ObliqueAngle(x) => write!(f, "oblique {}deg", x),
			Self::ObliqueAngleRange(min, max) => write!(f, "oblique {}deg {}deg", min, max),
		}
	}
}

#[derive(SmartDefault, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Weight {
	#[default]
	Normal,
	Bold,
	Number(u16),
}

impl std::fmt::Display for Weight {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Normal => write!(f, "normal"),
			Self::Bold => write!(f, "bold"),
			Self::Number(x) => write!(f, "{}", x),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, strum::Display)]
pub enum Format {
	#[strum(to_string = "woff")] Woff,
	#[strum(to_string = "woff2")] Woff2,
	#[strum(to_string = "truetype")] TrueType,
	#[strum(to_string = "opentype")] OpenType,
	#[strum(to_string = "embedded-opentype")] EmbeddedOpenType,
	#[strum(to_string = "svg")] Svg,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Source {
	Local(String),
	Url(String, Option<Format>),
}

impl std::fmt::Display for Source {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Local(name) => write!(f, r#"local("{}")"#, name),
			Self::Url(name, format) => {
				write!(f, r#"url("{}")"#, name)?;
				if let Some(format) = format {
					write!(f, r#" format("{}")"#, format)?;
				}
				Ok(())
			},
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, SmartDefault)]
pub struct FontFace {
	pub family: String,
	pub src: Vec<Source>,
	pub display: Display,
	pub stretch: (Stretch, Option<Stretch>),
	pub style: Style,
	pub weight: (Weight, Option<Weight>),
	// font_variant:
	// font-feature-settings
	// font-variation-settings:
	pub unicode_range: Vec<(u32, Option<u32>)>,
}

impl std::fmt::Display for FontFace {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		"@font-face{".fmt(f)?;
			write!(f, r#"font-family:"{}";"#, self.family)?;
			if let Some((first, rest)) = self.src.split_first() {
				write!(f, "src:{}", first)?;
				for src in rest {
					write!(f, r#",{}"#, src)?;
				}
				";".fmt(f)?;
			}
			write!(f, "font-display:{};", self.display)?;
			write!(f, "font-stretch:{} {};", &self.stretch.0, &if let Some(x) = self.stretch.1 { x } else { self.stretch.0 })?;
			write!(f, "font-style:{};", &self.style)?;
			write!(f, "font-weight:{} {};", &self.weight.0, &if let Some(x) = self.weight.1 { x } else { self.weight.0 })?;
			if let Some(((min, max), rest)) = self.unicode_range.split_first() {
				write!(f, "unicode-range:U+{:X}", min)?;
				if let Some(max) = max {
					write!(f, "-{:X}", max)?;
				}
				for (min, max) in rest {
					write!(f, ",U+{:X}", min)?;
					if let Some(max) = max {
						write!(f, "-{:X}", max)?;
					}
				}
				";".fmt(f)?;
			}
		"}".fmt(f)
	}
}

#[test]
fn font_face() {
	assert_eq!(
		FontFace {
			src: vec![Source::Url("https://fonts.gstatic.com/s/montserrat/v14/JTUSjIg1_i6t8kCHKm459Wlhyw.woff2".into(), Some(Format::Woff2))],
			family: "Montserrat".into(),
			weight: (Weight::Normal, Some(Weight::Normal)),
			..Default::default()
		}.to_string(),
		r#"@font-face{font-family:"Montserrat";src:url("https://fonts.gstatic.com/s/montserrat/v14/JTUSjIg1_i6t8kCHKm459Wlhyw.woff2") format("woff2");font-display:auto;font-stretch:normal normal;font-style:normal;font-weight:normal normal;}"#,
	);
}
