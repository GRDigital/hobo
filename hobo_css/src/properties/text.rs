use crate::prelude::*;
use crate::Color;

crate::macros::easy_enum! {direction ltr rtl}
crate::macros::easy_enum! {unicode-bidi normal embed bidi-override isolate isolate-override plaintext}
crate::macros::easy_enum! {white-space normal nowrap pre pre-line pre-wrap break-spaces}
crate::macros::easy_enum! {writing-mode horizontal-tb vertical-rl vertical-lr}
crate::macros::easy_enum! {hanging-punctuation none first last allow-end force-end}
crate::macros::easy_enum! {-*-hyphens manual none auto}
crate::macros::easy_enum! {text-align left right center justify}
crate::macros::easy_enum! {text-align-last left right center justify start end}
crate::macros::easy_enum! {text-justify auto inter-word inter-character none}
crate::macros::easy_enum! {font-stretch normal ultra-condensed extra-condensed condensed semi-condensed semi-expanded expanded extra-expanded ultra-expanded}
crate::macros::easy_enum! {list-style-type disc armenian circle cjk decimal decimal-leading-zero georgian hebrew hiragana hiragana-iroha katakana katakana-iroha lower-alpha lower-greek lower-latin lower-roman none square upper-alpha upper-greek upper-latin upper-roman}
crate::macros::easy_enum! {list-style-position inside outside}
crate::macros::easy_enum! {list-style-image none [string]}
crate::macros::easy_enum! {break-after auto avoid always all avoid-page page left right recto verso avoid-column column avoid-region region}
crate::macros::easy_enum! {break-before auto avoid always all avoid-page page left right recto verso avoid-column column avoid-region region}
crate::macros::easy_enum! {break-inside auto avoid avoid-page avoid-column avoid-region}
crate::macros::easy_enum! {font-variant normal small-caps}
crate::macros::easy_enum! {word-break normal break-all keep-all}
crate::macros::easy_enum! {word-wrap normal break-word}
crate::macros::easy_enum! {font-style normal italic oblique}
crate::macros::easy_enum! {font-size medium xx-small x-small small large x-large xx-large smaller larger [unit]}
crate::macros::easy_enum! {text-transform none capitalize uppercase lowercase}
crate::macros::easy_enum! {font-kerning auto normal none}
crate::macros::easy_enum! {word-spacing normal [unit]}
crate::macros::easy_enum! {text-overflow clip ellipsis [string]}
crate::macros::easy_enum! {vertical-align baseline sub super top text-top middle bottom text-bottom [unit]}
crate::macros::easy_enum! {line-height normal [float] [unit]}
crate::macros::easy_enum! {letter-spacing normal [unit]}
crate::macros::easy_enum! {tab-size [number]}
crate::macros::easy_enum! {text-decoration-style solid double dotted dashed wavy}
crate::macros::easy_enum! {text-decoration-line none underline overline line-through}
crate::macros::easy_enum! {text-rendering auto optimizeSpeed optimizeLegibility geometricPrecision}
crate::macros::easy_enum! {overflow-wrap normal break-word anywhere}
crate::macros::easy_enum! {font-weight normal bold bolder lighter [number]}
crate::macros::easy_color! {color}
crate::macros::easy_color! {text-decoration-color}
crate::macros::unit_value_macro! {text_indent}
crate::macros::unit_value_macro! {outline_offset}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum font_family {
	initial,
	inherit,
	unset,
	Some(Vec<String>),
}

impl font_family {
	#[inline] pub fn str(x: impl Into<String>) -> Self { Self::Some(vec![x.into()]) }
}

#[rustfmt::skip]
impl std::fmt::Display for font_family {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::initial => "font-family:initial;".fmt(f),
			Self::inherit => "font-family:inherit;".fmt(f),
			Self::unset   => "font-family:unset;".fmt(f),
			Self::Some(fonts) => {
				"font-family:".fmt(f)?;
				if let Some((first, rest)) = fonts.split_first() {
					write!(f, r#""{first}""#)?;
					for font in rest {
						write!(f, r#","{font}""#)?;
					}
				}
				";".fmt(f)
			},
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord, SmartDefault)]
pub struct TextShadowEffect {
	#[default(crate::colors::BLACK)]
	pub color: Color,
	pub offset_x: Unit,
	pub offset_y: Unit,
	pub blur_radius: Unit,
}

#[rustfmt::skip]
impl std::fmt::Display for TextShadowEffect {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} {} {} {}", self.color, self.offset_x, self.offset_y, self.blur_radius)
	}
}

impl crate::AppendProperty for TextShadowEffect {
	fn append_property(self, properties: &mut Vec<crate::Property>) {
		text_shadow::Some(vec![self]).append_property(properties);
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum text_shadow {
	initial,
	inherit,
	unset,
	Some(Vec<TextShadowEffect>),
}

impl text_shadow {
	pub fn effect(shadow_color: impl Into<Color>, offset_x: Unit, offset_y: Unit, blur_radius: Unit) -> Self {
		Self::Some(vec![TextShadowEffect { color: shadow_color.into(), offset_x, offset_y, blur_radius }])
	}
}

#[rustfmt::skip]
impl std::fmt::Display for text_shadow {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::initial => "text-shadow:initial;".fmt(f),
			Self::inherit => "text-shadow:inherit;".fmt(f),
			Self::unset   => "text-shadow:unset;".fmt(f),
			Self::Some(effects) => {
				"text-shadow:".fmt(f)?;
				if let Some((first, rest)) = effects.split_first() {
					write!(f, r#"{first}"#)?;
					for effect in rest {
						write!(f, r#",{effect}"#)?;
					}
				}
				";".fmt(f)
			},
		}
	}
}

#[test]
fn font_family_values() {
	assert_eq!(font_family!(initial).to_string(), "font-family:initial;");
	assert_eq!(font_family!(inherit).to_string(), "font-family:inherit;");
	assert_eq!(font_family!(unset).to_string(), "font-family:unset;");
	assert_eq!(font_family!("Helvetica", "Arial", "sans-serif").to_string(), r#"font-family:"Helvetica","Arial","sans-serif";"#);
}

#[test]
fn text_shadow_values() {
	assert_eq!(TextShadow::Initial.to_string(), "text-shadow:initial;");
	assert_eq!(TextShadow::Inherit.to_string(), "text-shadow:inherit;");
	assert_eq!(TextShadow::Unset.to_string(), "text-shadow:unset;");
	assert_eq!(TextShadow::Some(vec![TextShadowEffect::default()]).to_string(), "text-shadow:#000000ff 0 0 0;");
	assert_eq!(TextShadow::Some(vec![TextShadowEffect {
		color: Color::from_hex(0xff_00_00_ff),
		offset_x: unit!(1 px),
		offset_y: unit!(2 px),
		blur_radius: unit!(3 px),
	}]).to_string(), "text-shadow:#ff0000ff 1px 2px 3px;");
	assert_eq!(TextShadow::Some(vec![
		TextShadowEffect {
			color: crate::color::RED,
			offset_x: unit!(1 px),
			offset_y: unit!(2 px),
			blur_radius: unit!(3 px),
		},
		TextShadowEffect {
			color: crate::color::LIME,
			offset_x: unit!(5 px),
			offset_y: unit!(6 px),
			blur_radius: unit!(7 px),
		},
	]).to_string(), "text-shadow:#ff0000ff 1px 2px 3px,#00ff00ff 5px 6px 7px;");
}

// css::font!(
//     font "Roboto" 500 italic normal,
//     size 18 px,
//     spacing 3 px,
//     line-height 1.20,
//     color 0xFF,
//     transform upppercase,
//     decoration double underline,
// )
