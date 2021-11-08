use crate::{append_property::AppendProperty, prelude::*, Property};
use num_traits::cast::AsPrimitive;

#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone, PartialOrd, Ord)]
pub enum Filter {
	#[default]
	None,
	Initial,
	Inherit,
	Some(Vec<FilterFunction>),
}

#[rustfmt::skip]
impl std::fmt::Display for Filter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::None       => "filter:none;".fmt(f),
			Self::Initial    => "filter:initial;".fmt(f),
			Self::Inherit    => "filter:inherit;".fmt(f),
			Self::Some(fns)  => {
				"filter:".fmt(f)?;
				if let Some((first, rest)) = fns.split_first() {
					write!(f, "{}", first)?;
					for func in rest {
						write!(f, " {}", func)?;
					}
				}
				";".fmt(f)
			},
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum FilterFunction {
	Blur(i32),
	Brightness(F32),
	Contrast(F32),
	DropShadow(Unit, Unit, Option<u32>, Option<crate::Color>),
	Grayscale(F32),
	HueRotate(F32),
	Invert(F32),
	Opacity(F32),
	Saturate(F32),
	Sepia(F32),
	Url(String),
}

impl FilterFunction {
	pub fn blur(x: i32) -> Self { Self::Blur(x) }
	pub fn brightness<T: AsPrimitive<f32>>(x: T) -> Self { Self::Brightness(F32::new(x.as_()).unwrap()) }
	pub fn contrast<T: AsPrimitive<f32>>(x: T) -> Self { Self::Contrast(F32::new(x.as_()).unwrap()) }
	pub fn drop_shadow(h_shadow: Unit, v_shadow: Unit, blur: Option<u32>, color: Option<crate::Color>) -> Self { Self::DropShadow(h_shadow, v_shadow, blur, color) }
	pub fn grayscale<T: AsPrimitive<f32>>(x: T) -> Self { Self::Grayscale(F32::new(x.as_()).unwrap()) }
	pub fn hue_rotate<T: AsPrimitive<f32>>(x: T) -> Self { Self::HueRotate(F32::new(x.as_()).unwrap()) }
	pub fn invert<T: AsPrimitive<f32>>(x: T) -> Self { Self::Invert(F32::new(x.as_()).unwrap()) }
	pub fn opacity<T: AsPrimitive<f32>>(x: T) -> Self { Self::Opacity(F32::new(x.as_()).unwrap()) }
	pub fn saturate<T: AsPrimitive<f32>>(x: T) -> Self { Self::Saturate(F32::new(x.as_()).unwrap()) }
	pub fn sepia<T: AsPrimitive<f32>>(x: T) -> Self { Self::Sepia(F32::new(x.as_()).unwrap()) }
	pub fn url<T: Into<String>>(x: T) -> Self { Self::Url(x.into()) }
}

impl AppendProperty for FilterFunction {
	fn append_property(self, props: &mut Vec<Property>) { Filter::Some(vec![self]).append_property(props) }
}

impl std::fmt::Display for FilterFunction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Blur(x)                                     => write!(f, "blur({}px)", x),
			Self::Brightness(x)                               => write!(f, "brightness({})", x),
			Self::Contrast(x)                                 => write!(f, "contrast({})", x),
			Self::DropShadow(h_shadow, v_shadow, blur, color) => write!(f, "drop-shadow({} {} {} {})", h_shadow, v_shadow, blur.unwrap_or(0), color.unwrap_or_else(|| crate::Color::from_hex(0x00_00_00_00))),
			Self::Grayscale(x)                                => write!(f, "grayscale({})", x),
			Self::HueRotate(x)                                => write!(f, "hue-rotate({}deg)", x),
			Self::Invert(x)                                   => write!(f, "invert({})", x),
			Self::Opacity(x)                                  => write!(f, "opacity({})", x),
			Self::Saturate(x)                                 => write!(f, "saturate({})", x),
			Self::Sepia(x)                                    => write!(f, "sepia({})", x),
			Self::Url(x)                                      => write!(f, r#"url("{}")"#, x),
		}
	}
}
