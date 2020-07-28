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

impl std::fmt::Display for BorderImageSource {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::None    => "border-image-source:none;".fmt(f),
			Self::Initial => "border-image-source:initial;".fmt(f),
			Self::Inherit => "border-image-source:inherit;".fmt(f),
			Self::Unset   => "border-image-source:unset;".fmt(f),
			Self::Revert  => "border-image-source:revert;".fmt(f),
			Self::Some(images) => {
				"border-image-source:".fmt(f)?;
				if let Some((first, rest)) = images.split_first() {
					write!(f, "{}", first)?;
					for image in rest {
						write!(f, ",{}", image)?;
					}
				}
				";".fmt(f)
			},
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
impl std::fmt::Display for BorderWidth {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Medium  => "medium".fmt(f),
			Self::Thin    => "thin".fmt(f),
			Self::Thick   => "thick".fmt(f),
			Self::Zero    => "0".fmt(f),
			Self::Some(x) => x.fmt(f),
			Self::Initial => "initial".fmt(f),
			Self::Inherit => "inherit".fmt(f),
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
	pub color: crate::Color,
}

impl std::fmt::Display for BoxShadowEffect {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.inset { "inset ".fmt(f)? }
		write!(f, "{} {} {} {} {}", self.offset_x, self.offset_y, self.blur_radius, self.spread_radius, self.color)
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

impl std::fmt::Display for BoxShadow {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::None    => "box-shadow:none;".fmt(f),
			Self::Initial => "box-shadow:initial;".fmt(f),
			Self::Inherit => "box-shadow:inherit;".fmt(f),
			Self::Unset   => "box-shadow:unset;".fmt(f),
			Self::Revert  => "box-shadow:revert;".fmt(f),
			Self::Some(effects) => {
				"box-shadow:".fmt(f)?;
				if let Some((first, rest)) = effects.split_first() {
					write!(f, "{}", first)?;
					for effect in rest {
						write!(f, ",{}", effect)?;
					}
				}
				";".fmt(f)
			},
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

css_macros::easy_color! {border_left_color}
css_macros::easy_color! {border_right_color}
css_macros::easy_color! {border_top_color}
css_macros::easy_color! {border_bottom_color}
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
