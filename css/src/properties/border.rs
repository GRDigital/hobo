use crate::prelude::*;

css_macros::easy_enum! {border-collapse separate collapse}
css_macros::easy_enum! {box-decoration-break slice clone}
css_macros::easy_enum! {outline-width medium thin thick [unit]}
css_macros::easy_enum! {outline-style none hidden dotted dashed solid double groove ridge inset outset}
css_macros::easy_enum! {border-image-slice fill [raw]} // TODO:
css_macros::easy_enum! {border-image-width auto [raw]} // TODO:
css_macros::easy_enum! {border-image-outset [raw]} // TODO:
css_macros::easy_enum! {border-image-repeat stretch repeat round space}
css_macros::easy_color! {outline-color}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BorderImageSource {
	None,
	Initial,
	Inherit,
	Unset,
	Revert,
	Some(Vec<crate::Image>),
}

impl ToString for BorderImageSource {
	fn to_string(&self) -> String {
		match self {
			Self::None    => "border-image-source:none;".to_owned(),
			Self::Initial => "border-image-source:initial;".to_owned(),
			Self::Inherit => "border-image-source:inherit;".to_owned(),
			Self::Unset   => "border-image-source:unset;".to_owned(),
			Self::Revert  => "border-image-source:revert;".to_owned(),
			Self::Some(x) => format!("border-image-source:{};", x.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(",")),
		}
	}
}

#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, strum::Display)]
pub enum BorderStyle {
	#[strum(to_string = "none")] None,
	#[strum(to_string = "hidden")] Hidden,
	#[strum(to_string = "dotted")] Dotted,
	#[strum(to_string = "dashed")] Dashed,
	#[strum(to_string = "solid")] Solid,
	#[strum(to_string = "double")] Double,
	#[strum(to_string = "groove")] Groove,
	#[strum(to_string = "ridge")] Ridge,
	#[strum(to_string = "inset")] Inset,
	#[strum(to_string = "outset")] Outset,
	#[strum(to_string = "initial")] Initial,
	#[strum(to_string = "inherit")] Inherit,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BorderWidth {
	Medium,
	Thin,
	Thick,
	Zero,
	Some(crate::Unit),
	Initial,
	Inherit,
}

#[rustfmt::skip]
impl ToString for BorderWidth {
	fn to_string(&self) -> String {
		match self {
			Self::Medium  => "medium".to_owned(),
			Self::Thin    => "thin".to_owned(),
			Self::Thick   => "thick".to_owned(),
			Self::Zero    => "0".to_owned(),
			Self::Some(x) => x.to_string(),
			Self::Initial => "initial".to_owned(),
			Self::Inherit => "inherit".to_owned(),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct BoxShadowEffect {
	pub inset: bool,
	pub offset_x: Unit,
	pub offset_y: Unit,
	pub blur_radius: Unit,
	pub spread_radius: Unit,
	pub color: (u8, u8, u8, u8),
}

impl ToString for BoxShadowEffect {
	fn to_string(&self) -> String {
		format!(
			"{}{} {} {} {} #{:02x}{:02x}{:02x}{:02x}",
			if self.inset { "inset " } else { "" },
			self.offset_x.to_string(),
			self.offset_y.to_string(),
			self.blur_radius.to_string(),
			self.spread_radius.to_string(),
			self.color.0, self.color.1, self.color.2, self.color.3,
		)
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BoxShadow {
	None,
	Initial,
	Inherit,
	Unset,
	Revert,
	Some(Vec<BoxShadowEffect>),
}

impl ToString for BoxShadow {
	fn to_string(&self) -> String {
		match self {
			Self::None    => "box-shadow:none;".to_owned(),
			Self::Initial => "box-shadow:initial;".to_owned(),
			Self::Inherit => "box-shadow:inherit;".to_owned(),
			Self::Unset   => "box-shadow:unset;".to_owned(),
			Self::Revert  => "box-shadow:revert;".to_owned(),
			Self::Some(x) => format!("box-shadow:{};", x.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(",")),
		}
	}
}

#[rustfmt::skip]
#[macro_export]
#[doc(hidden)]
macro_rules! __border_width {
	($side:ident, medium)      => {$crate::paste::item!{$crate::Property::[<Border $side Width>]($crate::BorderWidth::Medium)}};
	($side:ident, thin)        => {$crate::paste::item!{$crate::Property::[<Border $side Width>]($crate::BorderWidth::Thin)}};
	($side:ident, thick)       => {$crate::paste::item!{$crate::Property::[<Border $side Width>]($crate::BorderWidth::Thick)}};
	($side:ident, initial)     => {$crate::paste::item!{$crate::Property::[<Border $side Width>]($crate::BorderWidth::Initial)}};
	($side:ident, inherit)     => {$crate::paste::item!{$crate::Property::[<Border $side Width>]($crate::BorderWidth::Inherit)}};
	($side:ident, 0)           => {$crate::paste::item!{$crate::Property::[<Border $side Width>]($crate::BorderWidth::Zero)}};
	($side:ident, $($val:tt)+) => {$crate::paste::item!{$crate::Property::[<Border $side Width>]($crate::BorderWidth::Some($crate::unit!($($val)+)))}};
}

#[macro_export] macro_rules! border_left_width {($($tt:tt)+)   => {$crate::__border_width!(Left, $($tt)+)}}
#[macro_export] macro_rules! border_right_width {($($tt:tt)+)  => {$crate::__border_width!(Right, $($tt)+)}}
#[macro_export] macro_rules! border_top_width {($($tt:tt)+)    => {$crate::__border_width!(Top, $($tt)+)}}
#[macro_export] macro_rules! border_bottom_width {($($tt:tt)+) => {$crate::__border_width!(Bottom, $($tt)+)}}
#[macro_export] macro_rules! border_width {($($tt:tt)+) => {
	vec![
		$crate::border_left_width!($($tt)+),
		$crate::border_right_width!($($tt)+),
		$crate::border_top_width!($($tt)+),
		$crate::border_bottom_width!($($tt)+),
	]
}}


#[rustfmt::skip]
#[macro_export]
#[doc(hidden)]
macro_rules! __border_style {
	($side:ident, none)    => {$crate::paste::item!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::None)}};
	($side:ident, hidden)  => {$crate::paste::item!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::Hidden)}};
	($side:ident, dotted)  => {$crate::paste::item!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::Dotted)}};
	($side:ident, dashed)  => {$crate::paste::item!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::Dashed)}};
	($side:ident, solid)   => {$crate::paste::item!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::Solid)}};
	($side:ident, double)  => {$crate::paste::item!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::Double)}};
	($side:ident, groove)  => {$crate::paste::item!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::Groove)}};
	($side:ident, ridge)   => {$crate::paste::item!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::Ridge)}};
	($side:ident, inset)   => {$crate::paste::item!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::Inset)}};
	($side:ident, outset)  => {$crate::paste::item!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::Outset)}};
	($side:ident, initial) => {$crate::paste::item!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::Initial)}};
	($side:ident, inherit) => {$crate::paste::item!{$crate::Property::[<Border $side Style>]($crate::BorderStyle::Inherit)}};
}

#[macro_export] macro_rules! border_left_style {($($tt:tt)+)   => {$crate::__border_style!(Left, $($tt)+)}}
#[macro_export] macro_rules! border_right_style {($($tt:tt)+)  => {$crate::__border_style!(Right, $($tt)+)}}
#[macro_export] macro_rules! border_top_style {($($tt:tt)+)    => {$crate::__border_style!(Top, $($tt)+)}}
#[macro_export] macro_rules! border_bottom_style {($($tt:tt)+) => {$crate::__border_style!(Bottom, $($tt)+)}}
#[macro_export] macro_rules! border_style {($($tt:tt)+) => {
	vec![
		$crate::border_left_style!($($tt)+),
		$crate::border_right_style!($($tt)+),
		$crate::border_top_style!($($tt)+),
		$crate::border_bottom_style!($($tt)+),
	]
}}


#[rustfmt::skip]
#[macro_export]
#[doc(hidden)]
macro_rules! __border_color {
	($side:ident, initial)   => {$crate::paste::item!{$crate::Property::[<Border $side Color>]($crate::ColorValue::Initial)}};
	($side:ident, inherit)   => {$crate::paste::item!{$crate::Property::[<Border $side Color>]($crate::ColorValue::Inherit)}};
	($side:ident, unset)     => {$crate::paste::item!{$crate::Property::[<Border $side Color>]($crate::ColorValue::Unset)}};
	($side:ident, revert)    => {$crate::paste::item!{$crate::Property::[<Border $side Color>]($crate::ColorValue::Revert)}};
	($side:ident, $rgb:expr) => {$crate::paste::item!{$crate::Property::[<Border $side Color>]($crate::ColorValue::Rgba($rgb.into()))}};
}

#[macro_export] macro_rules! border_left_color {($($tt:tt)+)   => {$crate::__border_color!(Left, $($tt)+)}}
#[macro_export] macro_rules! border_right_color {($($tt:tt)+)  => {$crate::__border_color!(Right, $($tt)+)}}
#[macro_export] macro_rules! border_top_color {($($tt:tt)+)    => {$crate::__border_color!(Top, $($tt)+)}}
#[macro_export] macro_rules! border_bottom_color {($($tt:tt)+) => {$crate::__border_color!(Bottom, $($tt)+)}}
#[macro_export] macro_rules! border_color {($($tt:tt)+) => {
	vec![
		$crate::border_left_color!($($tt)+),
		$crate::border_right_color!($($tt)+),
		$crate::border_top_color!($($tt)+),
		$crate::border_bottom_color!($($tt)+),
	]
}}

css_macros::unit_value_macro! {border_top_left_radius BorderTopLeftRadius}
css_macros::unit_value_macro! {border_top_right_radius BorderTopRightRadius}
css_macros::unit_value_macro! {border_bottom_left_radius BorderBottomLeftRadius}
css_macros::unit_value_macro! {border_bottom_right_radius BorderBottomRightRadius}
#[macro_export] macro_rules! border_radius {($($tt:tt)+) => {
	vec![
		$crate::border_top_left_radius!($($tt)+),
		$crate::border_top_right_radius!($($tt)+),
		$crate::border_bottom_left_radius!($($tt)+),
		$crate::border_bottom_right_radius!($($tt)+),
	]
}}
