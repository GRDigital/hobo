css_macros::easy_enum!{direction ltr rtl}
css_macros::easy_enum!{unicode-bidi normal embed bidi-override isolate isolate-override plaintext}
css_macros::easy_enum!{white-space normal nowrap pre pre-line pre-wrap}
css_macros::easy_enum!{writing-mode horizontal-tb vertical-rl vertical-lr}
css_macros::easy_enum!{hanging-punctuation none first last allow-end force-end}
css_macros::easy_enum!{hyphens manual none auto}
css_macros::easy_enum!{text-align left right center justify}
css_macros::easy_enum!{text-align-last left right center justify start end}
css_macros::easy_enum!{text-justify auto inter-word inter-character none}
css_macros::easy_enum!{font-stretch normal ultra-condensed extra-condensed condensed semi-condensed semi-expanded expanded extra-expanded ultra-expanded}
css_macros::easy_enum!{list-style-type disc armenian circle cjk-ideographic decimal decimal-leading-zero georgian hebrew hiragana hiragana-iroha katakana katakana-iroha lower-alpha lower-greek lower-latin lower-roman none square upper-alpha upper-greek upper-latin upper-roman}
css_macros::easy_enum!{page-break-after auto always avoid left right}
css_macros::easy_enum!{page-break-before auto always avoid left right}
css_macros::easy_enum!{page-break-inside auto avoid}
css_macros::easy_enum!{font-variant normal small-caps}
css_macros::easy_enum!{word-break normal break-all keep-all break-word}
css_macros::easy_enum!{word-wrap normal break-word}
css_macros::easy_enum!{font-style normal italic oblique}
css_macros::easy_enum!{font-size medium xx-small x-small small large x-large xx-large smaller larger @}
css_macros::easy_enum!{text-transform none capitalize uppercase lowercase}
css_macros::easy_enum!{font-kerning auto normal none}
css_macros::easy_enum!{font-family $}
css_macros::easy_enum!{word-spacing normal @}
css_macros::easy_enum!{text-indent @}
css_macros::easy_enum!{text-overflow clip ellipsis $}
css_macros::easy_enum!{vertical-align baseline sub super top text-top middle bottom text-bottom @}
css_macros::easy_enum!{line-height normal # @}
css_macros::easy_enum!{letter-spacing normal @}
css_macros::easy_enum!{tab-size #}
css_macros::easy_enum!{text-decoration-style solid double dotted dashed wavy}
css_macros::easy_enum!{text-decoration-line none underline overline line-through}
css_macros::easy_color!{color}
css_macros::easy_color!{text-decoration-color}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, strum_macros::Display)]
pub enum FontWeight {
	#[strum(to_string = "font-weight:normal;")] Normal,
	#[strum(to_string = "font-weight:bold;")] Bold,
	#[strum(to_string = "font-weight:bolder;")] Bolder,
	#[strum(to_string = "font-weight:lighter;")] Lighter,
	#[strum(to_string = "font-weight:100;")] Weight100,
	#[strum(to_string = "font-weight:200;")] Weight200,
	#[strum(to_string = "font-weight:300;")] Weight300,
	#[strum(to_string = "font-weight:400;")] Weight400,
	#[strum(to_string = "font-weight:500;")] Weight500,
	#[strum(to_string = "font-weight:600;")] Weight600,
	#[strum(to_string = "font-weight:700;")] Weight700,
	#[strum(to_string = "font-weight:800;")] Weight800,
	#[strum(to_string = "font-weight:900;")] Weight900,
	#[strum(to_string = "font-weight:initial;")] Initial,
	#[strum(to_string = "font-weight:inherit;")] Inherit,
}
#[macro_export]
macro_rules! font_weight {
	(normal)  => {$crate::Property::FontWeight($crate::FontWeight::Normal)};
	(bold)    => {$crate::Property::FontWeight($crate::FontWeight::Bold)};
	(bolder)  => {$crate::Property::FontWeight($crate::FontWeight::Bolder)};
	(lighter) => {$crate::Property::FontWeight($crate::FontWeight::Lighter)};
	(100)     => {$crate::Property::FontWeight($crate::FontWeight::Weight100)};
	(200)     => {$crate::Property::FontWeight($crate::FontWeight::Weight200)};
	(300)     => {$crate::Property::FontWeight($crate::FontWeight::Weight300)};
	(400)     => {$crate::Property::FontWeight($crate::FontWeight::Weight400)};
	(500)     => {$crate::Property::FontWeight($crate::FontWeight::Weight500)};
	(600)     => {$crate::Property::FontWeight($crate::FontWeight::Weight600)};
	(700)     => {$crate::Property::FontWeight($crate::FontWeight::Weight700)};
	(800)     => {$crate::Property::FontWeight($crate::FontWeight::Weight800)};
	(900)     => {$crate::Property::FontWeight($crate::FontWeight::Weight900)};
	(initial) => {$crate::Property::FontWeight($crate::FontWeight::Initial)};
	(inherit) => {$crate::Property::FontWeight($crate::FontWeight::Inherit)};
}
