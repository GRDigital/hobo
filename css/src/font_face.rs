use crate::prelude::*;

// TODO: replace @font-face selector with regular rust struct

#[derive(SmartDefault, strum::Display)]
pub enum FontDisplay {
	#[default]
	#[strum(to_string = "auto")] Auto,
	#[strum(to_string = "block")] Block,
	#[strum(to_string = "swap")] Swap,
	#[strum(to_string = "fallback")] Fallback,
	#[strum(to_string = "optional")] Optional,
}

#[derive(SmartDefault)]
pub enum FontStretch {
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
	Percentage(f32),
}

impl std::fmt::Display for FontStretch {
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

#[derive(SmartDefault)]
pub enum FontStyle {
	#[default]
	Normal,
	Italic,
	Oblique,
	ObliqueAngle(f32),
	ObliqueAngleRange(f32, f32),
}

impl std::fmt::Display for FontStyle {
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

#[derive(SmartDefault)]
pub enum FontWeight {
	#[default]
	Normal,
	Bold,
	Number(u16),
}

impl std::fmt::Display for FontWeight {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Normal => write!(f, "normal"),
			Self::Bold => write!(f, "bold"),
			Self::Number(x) => write!(f, "{}", x),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, strum::Display)]
pub enum FontFormat {
	#[strum(to_string = "woff")] Woff,
	#[strum(to_string = "woff2")] Woff2,
	#[strum(to_string = "truetype")] TrueType,
	#[strum(to_string = "opentype")] OpenType,
	#[strum(to_string = "embedded-opentype")] EmbeddedOpenType,
	#[strum(to_string = "svg")] Svg,
}

#[derive(SmartDefault)]
pub struct FontFace {
	font_family: String,
	src: Vec<(String, Option<FontFormat>)>,
	font_display: FontDisplay,
	font_stretch: (FontStretch, FontStretch),
	font_style: FontStyle,
	font_weight: (FontWeight, FontWeight),
	// font_variant:
	// font-feature-settings
	// font-variation-settings:
	// unicode_range: Vec<(u32, u32)>,
}

impl ToString for FontFace {
	fn to_string(&self) -> String {
		format!(r#"@font-face{{font-family:"{}";src:{};font-display:{};font-stretch:{} {};font-style:{};font-weight:{} {};}}"#,
			&self.font_family,
			self.src.iter().map(|(src, fmt)| format!(r#"url("{}"){}"#, src, if let Some(fmt) = fmt { format!(r#" format("{}")"#, fmt) } else { "".to_string() })).collect::<Vec<_>>().join(","),
			&self.font_display,
			&self.font_stretch.0, &self.font_stretch.1,
			&self.font_style,
			&self.font_weight.0, &self.font_weight.1,
		)
	}
}

#[test]
fn font_face() {
	FontFace {
		src: vec![("https://fonts.gstatic.com/s/montserrat/v14/JTUSjIg1_i6t8kCHKm459Wlhyw.woff2".into(), Some(FontFormat::Woff2))],
		font_family: "Montserrat".into(),
		font_weight: (FontWeight::Normal, FontWeight::Normal),
		..Default::default()
	};
}
