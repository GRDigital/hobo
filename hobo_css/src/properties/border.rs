use crate::{append_property::AppendProperty, prelude::*, Property};

crate::macros::easy_enum! {border-collapse separate collapse}
crate::macros::easy_enum! {box-decoration-break slice clone}
crate::macros::easy_enum! {outline-width medium thin thick [unit]}
crate::macros::easy_enum! {outline-style none hidden dotted dashed solid double groove ridge inset outset}
crate::macros::easy_enum! {border-image-slice fill [raw]} // TODO:
crate::macros::easy_enum! {border-image-width auto [raw]} // TODO:
crate::macros::easy_enum! {border-image-outset [raw]} // TODO:
crate::macros::easy_enum! {border-image-repeat stretch repeat round space}
crate::macros::easy_color! {outline-color}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum border_image_source {
	none,
	initial,
	inherit,
	unset,
	Some(Vec<crate::Image>),
}

impl border_image_source {
	pub fn single(x: crate::Image) -> Self { Self::Some(vec![x]) }
	pub fn url(x: impl Into<String>) -> Self { Self::Some(vec![crate::Image::Url(x.into())]) }
}

impl std::fmt::Display for border_image_source {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::none    => "border-image-source:none;".fmt(f),
			Self::initial => "border-image-source:initial;".fmt(f),
			Self::inherit => "border-image-source:inherit;".fmt(f),
			Self::unset   => "border-image-source:unset;".fmt(f),
			Self::Some(images) => {
				"border-image-source:".fmt(f)?;
				if let Some((first, rest)) = images.split_first() {
					write!(f, "{first}")?;
					for image in rest {
						write!(f, ",{image}")?;
					}
				}
				";".fmt(f)
			},
		}
	}
}

#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, strum::Display, PartialOrd, Ord)]
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
	#[strum(to_string = "unset")] Unset,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum BorderWidth {
	Medium,
	Thin,
	Thick,
	Some(crate::Unit),
	Initial,
	Inherit,
	Unset,
}

#[rustfmt::skip]
impl std::fmt::Display for BorderWidth {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Medium  => "medium".fmt(f),
			Self::Thin    => "thin".fmt(f),
			Self::Thick   => "thick".fmt(f),
			Self::Some(x) => x.fmt(f),
			Self::Initial => "initial".fmt(f),
			Self::Inherit => "inherit".fmt(f),
			Self::Unset   => "unset".fmt(f),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default, PartialOrd, Ord)]
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

impl AppendProperty for BoxShadowEffect {
	fn append_property(self, props: &mut Vec<Property>) { box_shadow::Some(vec![self]).append_property(props) }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum box_shadow {
	none,
	initial,
	inherit,
	unset,
	Some(Vec<BoxShadowEffect>),
}

impl std::fmt::Display for box_shadow {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::none    => "box-shadow:none;".fmt(f),
			Self::initial => "box-shadow:initial;".fmt(f),
			Self::inherit => "box-shadow:inherit;".fmt(f),
			Self::unset   => "box-shadow:unset;".fmt(f),
			Self::Some(effects) => {
				"box-shadow:".fmt(f)?;
				if let Some((first, rest)) = effects.split_first() {
					write!(f, "{first}")?;
					for effect in rest {
						write!(f, ",{effect}")?;
					}
				}
				";".fmt(f)
			},
		}
	}
}

macro_rules! decl_border {
	($($sides:ident),*) => {paste::paste!{$(
		pub struct [<border_ $sides _width>];
		impl [<border_ $sides _width>] {
			insert_enumlike![crate::Property::[<Border $sides:camel Width>],
				(initial, BorderWidth::Initial),
				(inherit, BorderWidth::Inherit),
				(unset, BorderWidth::Unset),
				(medium, BorderWidth::Medium),
				(thin, BorderWidth::Thin),
				(thick, BorderWidth::Thick),
			];
			insert_unitlike!(crate::Property::[<Border $sides:camel Width>], BorderWidth::Some);
		}

		pub struct [<border_ $sides _style>];
		impl [<border_ $sides _style>] {
			insert_enumlike![crate::Property::[<Border $sides:camel Style>],
				(initial, BorderStyle::Initial),
				(inherit, BorderStyle::Inherit),
				(unset, BorderStyle::Unset),
				(none, BorderStyle::None),
				(hidden, BorderStyle::Hidden),
				(dotted, BorderStyle::Dotted),
				(dashed, BorderStyle::Dashed),
				(solid, BorderStyle::Solid),
				(double, BorderStyle::Double),
				(groove, BorderStyle::Groove),
				(ridge, BorderStyle::Ridge),
				(inset, BorderStyle::Inset),
				(outset, BorderStyle::Outset),
			];
		}
	)*}};
}
decl_border![left, right, top, bottom];
crate::macros::easy_join!(border_horizontal_width, (border_left_width, border_right_width), (medium, thin, thick, [unit]));
crate::macros::easy_join!(border_vertical_width, (border_top_width, border_bottom_width), (medium, thin, thick, [unit]));
crate::macros::easy_join!(border_width, (border_horizontal_width, border_vertical_width), (medium, thin, thick, [unit]));
crate::macros::easy_join!(border_horizontal_style, (border_left_style, border_right_style), (none, hidden, dotted, dashed, solid, double, groove, ridge, inset, outset));
crate::macros::easy_join!(border_vertical_style, (border_top_style, border_bottom_style), (none, hidden, dotted, dashed, solid, double, groove, ridge, inset, outset));
crate::macros::easy_join!(border_style, (border_horizontal_style, border_vertical_style), (none, hidden, dotted, dashed, solid, double, groove, ridge, inset, outset));
crate::macros::easy_join!(border_horizontal_color, (border_left_color, border_right_color), ([color]));
crate::macros::easy_join!(border_vertical_color, (border_top_color, border_bottom_color), ([color]));
crate::macros::easy_join!(border_color, (border_horizontal_color, border_vertical_color), ([color]));
crate::macros::easy_join!(border_radius, (border_top_left_radius, border_top_right_radius, border_bottom_left_radius, border_bottom_right_radius), ([unit]));

crate::macros::easy_color! {border_left_color}
crate::macros::easy_color! {border_right_color}
crate::macros::easy_color! {border_top_color}
crate::macros::easy_color! {border_bottom_color}

crate::macros::unit_value_macro! {border_top_left_radius}
crate::macros::unit_value_macro! {border_top_right_radius}
crate::macros::unit_value_macro! {border_bottom_left_radius}
crate::macros::unit_value_macro! {border_bottom_right_radius}

#[test]
fn border_width_values() {
	assert_eq!(__border_width!(Left, medium).to_string(), "border-left-width:medium;");
	assert_eq!(__border_width!(Left, thin).to_string(), "border-left-width:thin;");
	assert_eq!(__border_width!(Left, thick).to_string(), "border-left-width:thick;");
	assert_eq!(__border_width!(Left, initial).to_string(), "border-left-width:initial;");
	assert_eq!(__border_width!(Left, inherit).to_string(), "border-left-width:inherit;");
	assert_eq!(__border_width!(Left, unset).to_string(), "border-left-width:unset;");

	assert_eq!(__border_width!(Right, medium).to_string(), "border-right-width:medium;");
	assert_eq!(__border_width!(Right, thin).to_string(), "border-right-width:thin;");
	assert_eq!(__border_width!(Right, thick).to_string(), "border-right-width:thick;");
	assert_eq!(__border_width!(Right, initial).to_string(), "border-right-width:initial;");
	assert_eq!(__border_width!(Right, inherit).to_string(), "border-right-width:inherit;");
	assert_eq!(__border_width!(Right, unset).to_string(), "border-right-width:unset;");

	assert_eq!(__border_width!(Top, medium).to_string(), "border-top-width:medium;");
	assert_eq!(__border_width!(Top, thin).to_string(), "border-top-width:thin;");
	assert_eq!(__border_width!(Top, thick).to_string(), "border-top-width:thick;");
	assert_eq!(__border_width!(Top, initial).to_string(), "border-top-width:initial;");
	assert_eq!(__border_width!(Top, inherit).to_string(), "border-top-width:inherit;");
	assert_eq!(__border_width!(Top, unset).to_string(), "border-top-width:unset;");

	assert_eq!(__border_width!(Bottom, medium).to_string(), "border-bottom-width:medium;");
	assert_eq!(__border_width!(Bottom, thin).to_string(), "border-bottom-width:thin;");
	assert_eq!(__border_width!(Bottom, thick).to_string(), "border-bottom-width:thick;");
	assert_eq!(__border_width!(Bottom, initial).to_string(), "border-bottom-width:initial;");
	assert_eq!(__border_width!(Bottom, inherit).to_string(), "border-bottom-width:inherit;");
	assert_eq!(__border_width!(Bottom, unset).to_string(), "border-bottom-width:unset;");
}

#[test]
fn border_style_values() {
	assert_eq!(__border_style!(Left, none).to_string(), "border-left-style:none;");
	assert_eq!(__border_style!(Left, hidden).to_string(), "border-left-style:hidden;");
	assert_eq!(__border_style!(Left, dotted).to_string(), "border-left-style:dotted;");
	assert_eq!(__border_style!(Left, dashed).to_string(), "border-left-style:dashed;");
	assert_eq!(__border_style!(Left, solid).to_string(), "border-left-style:solid;");
	assert_eq!(__border_style!(Left, double).to_string(), "border-left-style:double;");
	assert_eq!(__border_style!(Left, groove).to_string(), "border-left-style:groove;");
	assert_eq!(__border_style!(Left, ridge).to_string(), "border-left-style:ridge;");
	assert_eq!(__border_style!(Left, inset).to_string(), "border-left-style:inset;");
	assert_eq!(__border_style!(Left, outset).to_string(), "border-left-style:outset;");
	assert_eq!(__border_style!(Left, initial).to_string(), "border-left-style:initial;");
	assert_eq!(__border_style!(Left, inherit).to_string(), "border-left-style:inherit;");
	assert_eq!(__border_style!(Left, unset).to_string(), "border-left-style:unset;");

	assert_eq!(__border_style!(Right, none).to_string(), "border-right-style:none;");
	assert_eq!(__border_style!(Right, hidden).to_string(), "border-right-style:hidden;");
	assert_eq!(__border_style!(Right, dotted).to_string(), "border-right-style:dotted;");
	assert_eq!(__border_style!(Right, dashed).to_string(), "border-right-style:dashed;");
	assert_eq!(__border_style!(Right, solid).to_string(), "border-right-style:solid;");
	assert_eq!(__border_style!(Right, double).to_string(), "border-right-style:double;");
	assert_eq!(__border_style!(Right, groove).to_string(), "border-right-style:groove;");
	assert_eq!(__border_style!(Right, ridge).to_string(), "border-right-style:ridge;");
	assert_eq!(__border_style!(Right, inset).to_string(), "border-right-style:inset;");
	assert_eq!(__border_style!(Right, outset).to_string(), "border-right-style:outset;");
	assert_eq!(__border_style!(Right, initial).to_string(), "border-right-style:initial;");
	assert_eq!(__border_style!(Right, inherit).to_string(), "border-right-style:inherit;");
	assert_eq!(__border_style!(Right, unset).to_string(), "border-right-style:unset;");

	assert_eq!(__border_style!(Top, none).to_string(), "border-top-style:none;");
	assert_eq!(__border_style!(Top, hidden).to_string(), "border-top-style:hidden;");
	assert_eq!(__border_style!(Top, dotted).to_string(), "border-top-style:dotted;");
	assert_eq!(__border_style!(Top, dashed).to_string(), "border-top-style:dashed;");
	assert_eq!(__border_style!(Top, solid).to_string(), "border-top-style:solid;");
	assert_eq!(__border_style!(Top, double).to_string(), "border-top-style:double;");
	assert_eq!(__border_style!(Top, groove).to_string(), "border-top-style:groove;");
	assert_eq!(__border_style!(Top, ridge).to_string(), "border-top-style:ridge;");
	assert_eq!(__border_style!(Top, inset).to_string(), "border-top-style:inset;");
	assert_eq!(__border_style!(Top, outset).to_string(), "border-top-style:outset;");
	assert_eq!(__border_style!(Top, initial).to_string(), "border-top-style:initial;");
	assert_eq!(__border_style!(Top, inherit).to_string(), "border-top-style:inherit;");
	assert_eq!(__border_style!(Top, unset).to_string(), "border-top-style:unset;");

	assert_eq!(__border_style!(Bottom, none).to_string(), "border-bottom-style:none;");
	assert_eq!(__border_style!(Bottom, hidden).to_string(), "border-bottom-style:hidden;");
	assert_eq!(__border_style!(Bottom, dotted).to_string(), "border-bottom-style:dotted;");
	assert_eq!(__border_style!(Bottom, dashed).to_string(), "border-bottom-style:dashed;");
	assert_eq!(__border_style!(Bottom, solid).to_string(), "border-bottom-style:solid;");
	assert_eq!(__border_style!(Bottom, double).to_string(), "border-bottom-style:double;");
	assert_eq!(__border_style!(Bottom, groove).to_string(), "border-bottom-style:groove;");
	assert_eq!(__border_style!(Bottom, ridge).to_string(), "border-bottom-style:ridge;");
	assert_eq!(__border_style!(Bottom, inset).to_string(), "border-bottom-style:inset;");
	assert_eq!(__border_style!(Bottom, outset).to_string(), "border-bottom-style:outset;");
	assert_eq!(__border_style!(Bottom, initial).to_string(), "border-bottom-style:initial;");
	assert_eq!(__border_style!(Bottom, inherit).to_string(), "border-bottom-style:inherit;");
	assert_eq!(__border_style!(Bottom, unset).to_string(), "border-bottom-style:unset;");
}
