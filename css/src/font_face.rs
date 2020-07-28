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

#[derive(Debug, PartialEq, Eq, Hash, Clone, SmartDefault)]
pub struct FontFace {
	pub font_family: String,
	pub src: Vec<(String, Option<Format>)>,
	pub font_display: Display,
	pub font_stretch: (Stretch, Option<Stretch>),
	pub font_style: Style,
	pub font_weight: (Weight, Option<Weight>),
	// font_variant:
	// font-feature-settings
	// font-variation-settings:
	// unicode_range: Vec<(u32, u32)>,
}

impl std::fmt::Display for FontFace {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, r#"@font-face{{font-family:"{}";"#, self.font_family)?;
		if let Some((first, rest)) = self.src.split_first() {
			"src:".fmt(f)?;
			write!(f, r#"url("{}")"#, first.0)?;
			if let Some(fmt) = first.1 {
				write!(f, r#" format("{}")"#, fmt)?;
			}
			for (url, fmt) in rest {
				write!(f, r#", url("{}")"#, url)?;
				if let Some(fmt) = fmt {
					write!(f, r#" format("{}")"#, fmt)?;
				}
			}
			";".fmt(f)?;
		}
		write!(f,
			"font-display:{};font-stretch:{} {};font-style:{};font-weight:{} {};}}",
			self.font_display,
			&self.font_stretch.0, &if let Some(x) = self.font_stretch.1 { x } else { self.font_stretch.0 },
			&self.font_style,
			&self.font_weight.0, &if let Some(x) = self.font_weight.1 { x } else { self.font_weight.0 },
		)
	}
}

#[test]
fn font_face() {
	assert_eq!(
		FontFace {
			src: vec![("https://fonts.gstatic.com/s/montserrat/v14/JTUSjIg1_i6t8kCHKm459Wlhyw.woff2".into(), Some(Format::Woff2))],
			font_family: "Montserrat".into(),
			font_weight: (Weight::Normal, Some(Weight::Normal)),
			..Default::default()
		}.to_string(),
		r#"@font-face{font-family:"Montserrat";src:url("https://fonts.gstatic.com/s/montserrat/v14/JTUSjIg1_i6t8kCHKm459Wlhyw.woff2") format("woff2");font-display:auto;font-stretch:normal normal;font-style:normal;font-weight:normal normal;}"#,
	);
}
