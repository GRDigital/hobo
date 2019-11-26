css_macros::easy_enum!{background-repeat repeat repeat-x repeat-y no-repeat}
css_macros::easy_enum!{background-attachment scroll fixed local}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BackgroundColor {
	Rgba(u8, u8, u8, u8),
	Initial,
	Inherit,
}

impl ToString for BackgroundColor {
	fn to_string(&self) -> String {
		match self {
			Self::Rgba(r, g, b, a) => format!("background-color:#{:02x}{:02x}{:02x}{:02x};", r, g, b, a),
			Self::Initial          => "background-color:initial;".to_owned(),
			Self::Inherit          => "background-color:inherit;".to_owned(),
		}
	}
}

#[macro_export]
macro_rules! background_color {
	($r:tt $g:tt $b:tt $a:tt) => { $crate::Property::BackgroundColor::Rgba(($r, $g, $b, $a)) };
	($r:tt $g:tt $b:tt)       => { $crate::Property::BackgroundColor::Rgba(($r, $g, $b, 255)) };
	(initial)                 => {$crate::Property::BackgroundColor($crate::BackgroundColor::Initial)};
	(inherit)                 => {$crate::Property::BackgroundColor($crate::BackgroundColor::Inherit)};
}

