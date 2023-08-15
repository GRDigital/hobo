use crate::{append_property::AppendProperty, prelude::*, Property};
use num_traits::cast::AsPrimitive;

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, PartialOrd, Ord)]
pub enum filter {
	#[default]
	none,
	initial,
	inherit,
	multiple(Vec<FilterFunction>),
}

impl filter {
	pub fn blur(x: i32) -> Self { Self::multiple(vec![FilterFunction::blur(x)]) }
	pub fn brightness(x: impl num_traits::AsPrimitive<f32>) -> Self { Self::multiple(vec![FilterFunction::brightness(x)]) }
	pub fn contrast(x: impl num_traits::AsPrimitive<f32>) -> Self { Self::multiple(vec![FilterFunction::contrast(x)]) }
	pub fn drop_shadow(h_shadow: Unit, v_shadow: Unit, blur: Unit, color: Option<crate::Color>) -> Self { Self::multiple(vec![FilterFunction::drop_shadow(h_shadow, v_shadow, blur, color)]) }
	pub fn grayscale(x: impl num_traits::AsPrimitive<f32>) -> Self { Self::multiple(vec![FilterFunction::grayscale(x)]) }
	pub fn hue_rotate(x: impl num_traits::AsPrimitive<f32>) -> Self { Self::multiple(vec![FilterFunction::hue_rotate(x)]) }
	pub fn invert(x: impl num_traits::AsPrimitive<f32>) -> Self { Self::multiple(vec![FilterFunction::invert(x)]) }
	pub fn opacity(x: impl num_traits::AsPrimitive<f32>) -> Self { Self::multiple(vec![FilterFunction::opacity(x)]) }
	pub fn saturate(x: impl num_traits::AsPrimitive<f32>) -> Self { Self::multiple(vec![FilterFunction::saturate(x)]) }
	pub fn sepia(x: impl num_traits::AsPrimitive<f32>) -> Self { Self::multiple(vec![FilterFunction::sepia(x)]) }
	pub fn url(x: impl Into<String>) -> Self { Self::multiple(vec![FilterFunction::url(x)]) }
}

#[rustfmt::skip]
impl std::fmt::Display for filter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::none       => "filter:none;-webkit-filter:none;".fmt(f),
			Self::initial    => "filter:initial;-webkit-filter:initial;".fmt(f),
			Self::inherit    => "filter:inherit;-webkit-filter:inherit;".fmt(f),
			Self::multiple(functions)  => {
				let write = |f: &mut std::fmt::Formatter<'_>| -> std::fmt::Result {
					if let Some((first, rest)) = functions.split_first() {
						write!(f, "{first}")?;
						for func in rest { write!(f, ",{func}")?; }
					}

					Ok(())
				};
				"filter:".fmt(f)?; write(f)?; ";".fmt(f)?;
				"-webkit-filter:".fmt(f)?; write(f)?; ";".fmt(f)
			},
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum FilterFunction {
	Blur(i32),
	Brightness(F32),
	Contrast(F32),
	DropShadow(Unit, Unit, Unit, Option<crate::Color>),
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
	pub fn drop_shadow(h_shadow: Unit, v_shadow: Unit, blur: Unit, color: Option<crate::Color>) -> Self { Self::DropShadow(h_shadow, v_shadow, blur, color) }
	pub fn grayscale<T: AsPrimitive<f32>>(x: T) -> Self { Self::Grayscale(F32::new(x.as_()).unwrap()) }
	pub fn hue_rotate<T: AsPrimitive<f32>>(x: T) -> Self { Self::HueRotate(F32::new(x.as_()).unwrap()) }
	pub fn invert<T: AsPrimitive<f32>>(x: T) -> Self { Self::Invert(F32::new(x.as_()).unwrap()) }
	pub fn opacity<T: AsPrimitive<f32>>(x: T) -> Self { Self::Opacity(F32::new(x.as_()).unwrap()) }
	pub fn saturate<T: AsPrimitive<f32>>(x: T) -> Self { Self::Saturate(F32::new(x.as_()).unwrap()) }
	pub fn sepia<T: AsPrimitive<f32>>(x: T) -> Self { Self::Sepia(F32::new(x.as_()).unwrap()) }
	pub fn url<T: Into<String>>(x: T) -> Self { Self::Url(x.into()) }
}

impl AppendProperty for FilterFunction {
	fn append_property(self, props: &mut Vec<Property>) { filter::multiple(vec![self]).append_property(props) }
}

impl std::fmt::Display for FilterFunction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Blur(x)                                     => write!(f, "blur({x}px)"),
			Self::Brightness(x)                               => write!(f, "brightness({x})"),
			Self::Contrast(x)                                 => write!(f, "contrast({x})"),
			Self::DropShadow(h_shadow, v_shadow, blur, color) => write!(f, "drop-shadow({h_shadow} {v_shadow} {blur} {})", color.unwrap_or_else(|| crate::Color::from_hex(0x00_00_00_00))),
			Self::Grayscale(x)                                => write!(f, "grayscale({x})"),
			Self::HueRotate(x)                                => write!(f, "hue-rotate({x}deg)"),
			Self::Invert(x)                                   => write!(f, "invert({x})"),
			Self::Opacity(x)                                  => write!(f, "opacity({x})"),
			Self::Saturate(x)                                 => write!(f, "saturate({x})"),
			Self::Sepia(x)                                    => write!(f, "sepia({x})"),
			Self::Url(x)                                      => write!(f, r#"url("{x}")"#),
		}
	}
}
