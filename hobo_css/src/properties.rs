#![allow(non_camel_case_types)]

macro_rules! insert_enumlike {
	($prop:path,) => {};
	($prop:path, ($from:ident, $to:expr), $($rest:tt)*) => {
		#[allow(non_upper_case_globals)]
		pub const $from: crate::Property = $prop($to);
		insert_enumlike!($prop, $($rest)*);
	};
}

macro_rules! insert_unitlike {
	($prop:path, $val:path) => {
		#[inline] pub fn zero() -> crate::Property { $prop($val(crate::Unit::Zero)) }
		#[inline] pub fn px(  x: impl num_traits::cast::AsPrimitive<f32>) -> crate::Property { $prop($val(crate::Unit::px(x))) }
		#[inline] pub fn pct( x: impl num_traits::cast::AsPrimitive<f32>) -> crate::Property { $prop($val(crate::Unit::pct(x))) }
		#[inline] pub fn em(  x: impl num_traits::cast::AsPrimitive<f32>) -> crate::Property { $prop($val(crate::Unit::em(x))) }
		#[inline] pub fn rem( x: impl num_traits::cast::AsPrimitive<f32>) -> crate::Property { $prop($val(crate::Unit::rem(x))) }
		#[inline] pub fn vw(  x: impl num_traits::cast::AsPrimitive<f32>) -> crate::Property { $prop($val(crate::Unit::vw(x))) }
		#[inline] pub fn vh(  x: impl num_traits::cast::AsPrimitive<f32>) -> crate::Property { $prop($val(crate::Unit::vh(x))) }
		#[inline] pub fn vmin(x: impl num_traits::cast::AsPrimitive<f32>) -> crate::Property { $prop($val(crate::Unit::vmin(x))) }
		#[inline] pub fn vmax(x: impl num_traits::cast::AsPrimitive<f32>) -> crate::Property { $prop($val(crate::Unit::vmax(x))) }
		#[inline] pub fn fr(  x: impl num_traits::cast::AsPrimitive<f32>) -> crate::Property { $prop($val(crate::Unit::fr(x))) }
		#[inline] pub fn dur( x: impl num_traits::cast::AsPrimitive<f32>) -> crate::Property { $prop($val(crate::Unit::dur(x))) }
		#[inline] pub fn unit(x: crate::Unit                            ) -> crate::Property { $prop($val(x)) }
	};
}

#[macro_use] mod flex;
#[macro_use] mod margin_props;
#[macro_use] mod padding_props;
#[macro_use] mod dimensions;
#[macro_use] mod position_props;
#[macro_use] mod text;
#[macro_use] mod border;
#[macro_use] mod background;
#[macro_use] mod svg;
#[macro_use] mod animation;
#[macro_use] mod transform;
#[macro_use] mod filter;
#[macro_use] mod grid;
#[macro_use] mod clip_path;

use crate::prelude::*;
pub use animation::*;
pub use background::*;
pub use border::*;
pub use clip_path::*;
pub use dimensions::*;
pub use filter::*;
pub use flex::*;
pub use grid::*;
pub use margin_props::*;
pub use padding_props::*;
pub use position_props::*;
pub use svg::*;
pub use text::*;
pub use transform::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum ColorValue {
	Rgba(crate::color::Color),
	Initial,
	Inherit,
	Unset,
}

#[rustfmt::skip]
impl std::fmt::Display for ColorValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Rgba(x)    => x.fmt(f),
			Self::Initial    => "initial".fmt(f),
			Self::Inherit    => "inherit".fmt(f),
			Self::Unset      => "unset".fmt(f),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum UnitValue {
	Unit(Unit),
	Initial,
	Inherit,
	Unset,
}

#[rustfmt::skip]
impl std::fmt::Display for UnitValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Unit(x) => x.fmt(f),
			Self::Initial => "initial".fmt(f),
			Self::Inherit => "inherit".fmt(f),
			Self::Unset   => "unset".fmt(f),
		}
	}
}

/*
pub enum RadialGradientShape {
	Circle,
	Ellipse,
	// ???
}

pub struct RadialGradient {
	shape: RadialGradientShape,
	center_point: Vec<[BackgroundPositionElement; 4]>,
	stop_list: Vec<((u8, u8, u8, u8), Unit)>,
}
*/

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct LinearGradient {
	pub angle: F32,
	pub stop_list: Vec<(crate::Color, Unit)>,
}

impl std::fmt::Display for LinearGradient {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}deg", self.angle)?;
		for (color, stop) in &self.stop_list {
			write!(f, ",{} {}", color, stop)?;
		}
		Ok(())
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum Image {
	Url(String),
	LinearGradient(LinearGradient),
	RepeatingLinearGradient(LinearGradient),
	// RadialGradient(RadialGradient),
	// RepeatingRadialGradient(RadialGradient),
	// conic ??
}

impl Image {
	pub fn url(x: impl Into<String>) -> Self { Self::Url(x.into()) }
	pub fn linear_gradient(angle: f32, stop_list: Vec<(crate::Color, Unit)>) -> Self { Self::LinearGradient(LinearGradient { angle: F32::new(angle).unwrap(), stop_list }) }
	pub fn repeating_linear_gradient(angle: f32, stop_list: Vec<(crate::Color, Unit)>) -> Self { Self::RepeatingLinearGradient(LinearGradient { angle: F32::new(angle).unwrap(), stop_list }) }
}

impl std::fmt::Display for Image {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Url(x) => write!(f, r#"url("{}")"#, x),
			Self::LinearGradient(x) => write!(f, "linear-gradient({})", x),
			Self::RepeatingLinearGradient(x) => write!(f, "repeating-linear-gradient({})", x),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum BasicShape {
	Polygon(Vec<(Unit, Unit)>),
	// etc
}

impl std::fmt::Display for BasicShape {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Polygon(points) => {
				"polygon(".fmt(f)?;
				if let Some(((x, y), rest)) = points.split_first() {
					write!(f, "{} {}", x, y)?;
					for (x, y) in rest {
						write!(f, ",{} {}", x, y)?;
					}
				}
				")".fmt(f)
			},
		}
	}
}

macro_rules! generate_properties {
	(
		stutter => ($($stutter_name:ident),*$(,)?),
		named => ($($css_name:expr => $named_name:ident($named_type:ty)),*$(,)?),
	) => {paste::paste!{
		#[derive(Debug, PartialEq, Eq, Hash, Clone, strum::EnumDiscriminants)]
		#[strum_discriminants(derive(PartialOrd, Ord))]
		pub enum Property {
			Raw(String),
			$([<$stutter_name:camel>]($stutter_name),)*
			$($named_name($named_type)),*
		}

		impl std::fmt::Display for Property {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				match self {
					Self::Raw(x) => x.fmt(f),
					$(Self::$named_name(x) => write!(f, "{}:{};", $css_name, x),)*
					$(Self::[<$stutter_name:camel>](x) => x.fmt(f)),*
				}
			}
		}

		$(
			impl From<$stutter_name> for Property {
				fn from(x: $stutter_name) -> Self { Self::[<$stutter_name:camel>](x) }
			}

			impl crate::AppendProperty for $stutter_name {
				fn append_property(self, decls: &mut Vec<Property>) { decls.push(Property::[<$stutter_name:camel>](self)); }
			}
		)*
	}};
}

// transform
// filter
// borderimagesource
// clippath
// gridautoflow
// boxshadow
// transformorigin

// @Awpteamoose: I'm choosing to implement macroless syntax as inherent impl methods on types returning Self
// rather than a mod with smth like `enum Property` and free fns that return Property
// because a mod is completely sealed, while types are extensible (e.g. with extensions traits)
// so one could theoretically do smth like `css::width::columns(4)` and have some complex logic inside
// that would also sum up gaps between columns or smth
//
// the type names generated are snake case because it's 1) closer to css 2) easier to read 3) requires fewer shift presses to type
// it is possible to have type names as camelcase and then just alias them in snake case
// but using enum variants is still annoying because you can't also alias enum variants
//
// theoretically it *is* possible to extend mods
// you could define your own mod and import everything from the mod you're extending and then write new fns/etc
// e.g. instead of css::size you can `mod size { use css::size::*; ... }` then `mod css { use css::{all but size}; use size; }`
//
// also, rather than having this huge enum, it might be possible fo just having a Vec of `Box<Display>` or `Box<Property>` with `Property` being a trait
// the benefits are unclear
generate_properties! {
	// different properties that have specific to them arguments
	// basis/grow/shrink/order kind of take the same, but basis and shrink are 1 by default while others are 0 so /shrug
	stutter => (
		flex_wrap, flex_direction,
		justify_content, align_items, align_content, align_self,
		justify_items, justify_self,
		flex_basis, flex_grow, flex_shrink,

		order,
		position,
		display,
		box_sizing,
		visibility,
		z_index,
		overflow_x, overflow_y,
		direction,
		unicode_bidi,
		white_space,
		writing_mode,
		hanging_punctuation,
		hyphens,
		resize,
		object_fit,
		list_style_type, list_style_image, list_style_position,

		break_after, break_before, break_inside,

		text_align, text_align_last, text_justify,
		text_transform, text_shadow, text_overflow, text_anchor,
		text_decoration_style, text_decoration_line, text_rendering,

		font_stretch, font_variant, font_style, font_weight,
		font_size, font_kerning, font_family,

		word_break, word_wrap,
		overflow_wrap, overflow_anchor,
		transform_style,
		mix_blend_mode,
		isolation,
		caption_side,
		empty_cells,
		table_layout,
		border_collapse,
		all,
		word_spacing,
		vertical_align,
		line_height,
		letter_spacing,
		tab_size,
		box_decoration_break,
		outline_width, outline_style,
		content,
		opacity,
		perspective,
		backface_visibility,
		vector_effect,
		alignment_baseline,
		dominant_baseline,
		stroke_linecap,

		background_image, background_size,
		background_repeat, background_attachment,
		background_blend_mode, background_origin, background_clip,

		animation_direction, animation_fill_mode, animation_iteration_count,
		animation_name, animation_play_state, animation_timing_function,
		animation_duration, animation_delay,

		transition_property, transition_timing_function,
		transition_duration, transition_delay,

		Transform,
		Filter,

		BorderImageSource, border_image_slice, border_image_width,
		border_image_outset, border_image_repeat,

		scroll_behavior, pointer_events, user_select, touch_action, cursor,

		ClipPath,
		GridAutoFlow,
		row_gap, column_gap, grid_gap,
		BoxShadow,
		TransformOrigin,
		appearance,
		mask_image, mask_size,
		float, clear,
		aspect_ratio,
		scrollbar_gutter,
	),
	// different properties that take the same argument
	named => (
		"margin-left" => MarginLeft(Margin),
		"margin-right" => MarginRight(Margin),
		"margin-top" => MarginTop(Margin),
		"margin-bottom" => MarginBottom(Margin),

		"padding-left" => PaddingLeft(UnitValue),
		"padding-right" => PaddingRight(UnitValue),
		"padding-top" => PaddingTop(UnitValue),
		"padding-bottom" => PaddingBottom(UnitValue),

		"width" => Width(Dimension),
		"height" => Height(Dimension),
		"min-width" => MinWidth(Dimension),
		"max-width" => MaxWidth(Dimension),
		"min-height" => MinHeight(Dimension),
		"max-height" => MaxHeight(Dimension),

		"top" => Top(PositionOffset),
		"right" => Right(PositionOffset),
		"left" => Left(PositionOffset),
		"bottom" => Bottom(PositionOffset),

		"border-left-color" => BorderLeftColor(ColorValue),
		"border-right-color" => BorderRightColor(ColorValue),
		"border-top-color" => BorderTopColor(ColorValue),
		"border-bottom-color" => BorderBottomColor(ColorValue),

		"border-left-style" => BorderLeftStyle(BorderStyle),
		"border-right-style" => BorderRightStyle(BorderStyle),
		"border-top-style" => BorderTopStyle(BorderStyle),
		"border-bottom-style" => BorderBottomStyle(BorderStyle),

		"border-left-width" => BorderLeftWidth(BorderWidth),
		"border-right-width" => BorderRightWidth(BorderWidth),
		"border-top-width" => BorderTopWidth(BorderWidth),
		"border-bottom-width" => BorderBottomWidth(BorderWidth),

		"border-top-left-radius" => BorderTopLeftRadius(UnitValue),
		"border-top-right-radius" => BorderTopRightRadius(UnitValue),
		"border-bottom-left-radius" => BorderBottomLeftRadius(UnitValue),
		"border-bottom-right-radius" => BorderBottomRightRadius(UnitValue),

		"background-color" => BackgroundColor(ColorValue),
		"color" => Color(ColorValue),
		"text-decoration-color" => TextDecorationColor(ColorValue),
		"fill" => Fill(ColorValue),
		"stroke" => Stroke(ColorValue),
		"stroke-width" => StrokeWidth(UnitValue),
		"outline-color" => OutlineColor(ColorValue),

		"grid-template-columns" => GridTemplateColumns(GridTemplate),
		"grid-template-rows" => GridTemplateRows(GridTemplate),
		"grid-auto-columns" => GridAutoColumns(GridAuto),
		"grid-auto-rows" => GridAutoRows(GridAuto),

		"text-indent" => TextIndent(UnitValue),
		"outline-offset" => OutlineOffset(UnitValue),

		"grid-column-start" => GridColumnStart(GridSpan),
		"grid-column-end" => GridColumnEnd(GridSpan),
		"grid-row-start" => GridRowStart(GridSpan),
		"grid-row-end" => GridRowEnd(GridSpan),

		"background-position-x" => BackgroundPositionX(UnitValue),
		"background-position-y" => BackgroundPositionY(UnitValue),
	),
}

impl std::cmp::PartialOrd for Property {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		PropertyDiscriminants::from(self).partial_cmp(&PropertyDiscriminants::from(other))
	}
}

impl std::cmp::Ord for Property {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		PropertyDiscriminants::from(self).cmp(&PropertyDiscriminants::from(other))
	}
}

crate::macros::easy_enum! {box-sizing content-box border-box}
crate::macros::easy_enum! {visibility visible hidden collapse}
crate::macros::easy_enum! {display block none inline inline-block flex inline-flex grid inline-grid flow-root contents table table-row table-row-group table-header-group table-footer-group table-cell table-column-group table-column table-caption list-item}
crate::macros::easy_enum! {-*-user-select auto none text all}
crate::macros::easy_enum! {scroll-behavior auto smooth}
crate::macros::easy_enum! {pointer-events auto none}
crate::macros::easy_enum! {touch-action auto none manipulation}
crate::macros::easy_enum! {resize none both horizontal vertical}
crate::macros::easy_enum! {object-fit fill contain cover scale-down none}
crate::macros::easy_enum! {transform-style flat preserve-3d}
crate::macros::easy_enum! {background-blend-mode normal multiply screen overlay darken lighten color-dodge saturation color luminosity}
crate::macros::easy_enum! {mix-blend-mode normal multiply screen overlay darken lighten color-dodge color-burn difference exclusion hue saturation color luminosity}
crate::macros::easy_enum! {isolation auto isolate}
crate::macros::easy_enum! {caption-side top bottom}
crate::macros::easy_enum! {empty-cells show hide}
crate::macros::easy_enum! {table-layout auto fixed}
crate::macros::easy_enum! {all}
crate::macros::easy_enum! {cursor auto alias all-scroll cell context-menu col-resize copy crosshair default e-resize ew-resize grab grabbing help move n-resize ne-resize nesw-resize ns-resize nw-resize nwse-resize no-drop none not-allowed pointer progress row-resize s-resize se-resize sw-resize text vertical-text w-resize wait zoom-in zoom-out}
crate::macros::easy_enum! {content normal none counter open-quote close-quote no-open-quote no-close-quote [string]}
crate::macros::easy_enum! {opacity [float]}
crate::macros::easy_enum! {perspective none [unit]}
crate::macros::easy_enum! {backface-visibility visible hidden}
crate::macros::easy_enum! {overflow-x visible hidden scroll auto}
crate::macros::easy_enum! {overflow-y visible hidden scroll auto}
crate::macros::easy_enum! {float none left right inline-start inline-end}
crate::macros::easy_enum! {clear none left right inline-start inline-end both}
crate::macros::easy_enum! {overflow-anchor auto none}
crate::macros::easy_enum! {scrollbar-gutter auto stable}
crate::macros::easy_enum! {-*-appearance auto none}

crate::macros::easy_join!(overflow, (overflow_x, overflow_y), (visible, hidden, scroll, auto));
crate::macros::easy_join!(size, (width, height), (auto, [unit]));
