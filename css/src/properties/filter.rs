use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone)]
pub enum Filter {
	#[default]
	None,
	Initial,
	Inherit,
	Some(Vec<FilterFunction>),
}

#[rustfmt::skip]
impl ToString for Filter {
	fn to_string(&self) -> String {
		match self {
			Self::None       => "filter:none;".to_owned(),
			Self::Initial    => "filter:initial;".to_owned(),
			Self::Inherit    => "filter:inherit;".to_owned(),
			Self::Some(fns)  => format!("filter:{};", fns.iter().map(ToString::to_string).collect::<Vec<_>>().join(" ")),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum FilterFunction {
	Blur(i32),
	Brightness(F32),
	Contrast(F32),
	DropShadow(Unit, Unit, Option<u32>, Option<(u8, u8, u8, u8)>), // h-shadow v-shadow blur colour
	Grayscale(F32),
	HueRotate(F32),
	Invert(F32),
	Opacity(F32),
	Saturate(F32),
	Sepia(F32),
	Url(String),
}

impl ToString for FilterFunction {
	fn to_string(&self) -> String {
		match self {
			Self::Blur(x)                                     => format!("blur({}px)", x),
			Self::Brightness(x)                               => format!("brightness({})", x),
			Self::Contrast(x)                                 => format!("contrast({})", x),
			Self::DropShadow(h_shadow, v_shadow, blur, color) => format!("drop-shadow({} {} {} {})", h_shadow.to_string(), v_shadow.to_string(), blur.unwrap_or(0), color.map(|(r, g, b, a)| format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a)).unwrap_or_else(String::new)),
			Self::Grayscale(x)                                => format!("grayscale({})", x),
			Self::HueRotate(x)                                => format!("hue-rotate({}deg)", x),
			Self::Invert(x)                                   => format!("invert({})", x),
			Self::Opacity(x)                                  => format!("opacity({})", x),
			Self::Saturate(x)                                 => format!("saturate({})", x),
			Self::Sepia(x)                                    => format!("sepia({})", x),
			Self::Url(x)                                      => format!("url({})", x),
		}
	}
}
