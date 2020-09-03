crate::macros::easy_enum! {direction ltr rtl}
crate::macros::easy_enum! {unicode-bidi normal embed bidi-override isolate isolate-override plaintext}
crate::macros::easy_enum! {white-space normal nowrap pre pre-line pre-wrap}
crate::macros::easy_enum! {writing-mode horizontal-tb vertical-rl vertical-lr}
crate::macros::easy_enum! {hanging-punctuation none first last allow-end force-end}
crate::macros::easy_enum! {hyphens manual none auto}
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
crate::macros::unit_value_macro! {text_indent TextIndent}
crate::macros::unit_value_macro! {outline_offset OutlineOffset}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum FontFamily {
	Initial,
	Inherit,
	Unset,
	Some(Vec<String>),
}

#[rustfmt::skip]
impl std::fmt::Display for FontFamily {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Initial => "font-family:initial;".fmt(f),
			Self::Inherit => "font-family:inherit;".fmt(f),
			Self::Unset   => "font-family:unset;".fmt(f),
			Self::Some(fonts) => {
				"font-family:".fmt(f)?;
				if let Some((first, rest)) = fonts.split_first() {
					write!(f, r#""{}""#, first)?;
					for font in rest {
						write!(f, r#","{}""#, font)?;
					}
				}
				";".fmt(f)
			},
		}
	}
}

#[macro_export]
macro_rules! font_family {
	(initial)         => {$crate::Property::FontFamily($crate::FontFamily::Initial)};
	(inherit)         => {$crate::Property::FontFamily($crate::FontFamily::Inherit)};
	(unset)           => {$crate::Property::FontFamily($crate::FontFamily::Unset)};
	($($font:expr),+) => {$crate::Property::FontFamily($crate::FontFamily::Some(vec![$($font.into()),+]))};
}

#[test]
fn font_family_values() {
	assert_eq!(font_family!(initial).to_string(), "font-family:initial;");
	assert_eq!(font_family!(inherit).to_string(), "font-family:inherit;");
	assert_eq!(font_family!(unset).to_string(), "font-family:unset;");
	assert_eq!(font_family!("Helvetica", "Arial", "sans-serif").to_string(), r#"font-family:"Helvetica","Arial","sans-serif";"#);
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
