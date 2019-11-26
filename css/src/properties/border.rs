css_macros::easy_enum!{border-collapse separate collapse}
css_macros::easy_enum!{box-decoration-break slice clone unset}
css_macros::easy_enum!{outline-width medium thin thick @}
css_macros::easy_enum!{outline-style none hidden dotted dashed solid double groove ridge inset outset}
css_macros::easy_enum!{outline-offset @}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum OutlineColor {
	Invert,
	Rgba(u8, u8, u8, u8),
	Initial,
	Inherit,
}

impl ToString for OutlineColor {
	fn to_string(&self) -> String {
		match self {
			Self::Invert             => "outline-color:invert;".to_owned(),
			Self::Rgba(r, g, b, a)   => format!("outline-color:#{:02x}{:02x}{:02x}{:02x};", r, g, b, a),
			Self::Initial            => "outline-color:initial;".to_owned(),
			Self::Inherit            => "outline-color:inherit;".to_owned(),
		}
	}
}

#[macro_export]
macro_rules! outline_color {
	(invert)                  => {$crate::Property::OutlineColor($crate::OutlineColor::Invert)};
	($r:tt $g:tt $b:tt $a:tt) => { $crate::Property::OutlineColor::Rgba(($r, $g, $b, $a)) };
	($r:tt $g:tt $b:tt)       => { $crate::Property::OutlineColor::Rgba(($r, $g, $b, 255)) };
	(initial)                 => {$crate::Property::OutlineColor($crate::OutlineColor::Initial)};
	(inherit)                 => {$crate::Property::OutlineColor($crate::OutlineColor::Inherit)};
}
