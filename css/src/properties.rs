#[macro_use] mod flex;
#[macro_use] mod margin;
#[macro_use] mod padding;
#[macro_use] mod dimensions;
#[macro_use] mod position;
#[macro_use] mod text;
#[macro_use] mod border;
#[macro_use] mod background;
#[macro_use] mod svg;
#[macro_use] mod animation;
#[macro_use] mod transform;
#[macro_use] mod filter;
#[macro_use] mod grid;
#[macro_use] mod clip_path;
#[macro_use] mod appearance;

use crate::prelude::*;
pub use animation::*;
pub use background::*;
pub use border::*;
pub use clip_path::*;
pub use dimensions::*;
pub use filter::*;
pub use flex::*;
pub use grid::*;
pub use margin::*;
pub use padding::*;
pub use position::*;
pub use svg::*;
pub use text::*;
pub use transform::*;
pub use appearance::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Image {
	Url(String),
	LinearGradient(LinearGradient),
	RepeatingLinearGradient(LinearGradient),
	// RadialGradient(RadialGradient),
	// RepeatingRadialGradient(RadialGradient),
	// conic ??
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
	) => {
		pub use Property::{$($named_name),*};

		#[derive(Debug, PartialEq, Eq, Hash, Clone)]
		pub enum Property {
			Raw(String),
			$($stutter_name($stutter_name),)*
			$($named_name($named_type)),*
		}

		impl std::fmt::Display for Property {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				match self {
					Self::Raw(x) => x.fmt(f),
					$(Self::$named_name(x) => write!(f, "{}:{};", $css_name, x),)*
					$(Self::$stutter_name(x) => x.fmt(f)),*
				}
			}
		}

		$(
			impl From<$stutter_name> for Property {
				fn from(x: $stutter_name) -> Self { Self::$stutter_name(x) }
			}

			impl crate::AppendProperty for $stutter_name {
				fn append_property(self, decls: &mut Vec<Property>) { decls.push(Property::$stutter_name(self)); }
			}
		)*
	};
}

generate_properties! {
	// different properties that have specific to them arguments
	// basis/grow/shrink/order kind of take the same, but basis and shrink are 1 by default while others are 0 so /shrug
	stutter => (
		FlexWrap, FlexDirection,
		JustifyContent, AlignItems, AlignContent, AlignSelf,
		JustifyItems, JustifySelf,
		FlexBasis, FlexGrow, FlexShrink,

		Order,
		Position,
		Display,
		BoxSizing,
		Visibility,
		ZIndex,
		OverflowX, OverflowY,
		Direction,
		UnicodeBidi,
		WhiteSpace,
		WritingMode,
		HangingPunctuation,
		Hyphens,
		TextAlign,
		TextAlignLast,
		TextJustify,
		FontStretch,
		UserSelect,
		ScrollBehavior,
		PointerEvents,
		Resize,
		ObjectFit,
		ListStyleType, ListStyleImage, ListStylePosition,

		BreakAfter, BreakBefore, BreakInside,

		FontVariant,
		WordBreak,
		WordWrap,
		FontStyle,
		TransformStyle,
		BackgroundBlendMode,
		MixBlendMode,
		Isolation,
		CaptionSide,
		EmptyCells,
		TableLayout,
		BorderCollapse,
		All,
		FontWeight,
		FontSize,
		BackgroundRepeat,
		BackgroundAttachment,
		Cursor,
		TextTransform,
		FontKerning,
		FontFamily,
		WordSpacing,
		TextOverflow,
		VerticalAlign,
		LineHeight,
		LetterSpacing,
		TabSize,
		BoxDecorationBreak,
		OutlineWidth,
		OutlineStyle,
		Content,
		Opacity,
		Perspective,
		BackfaceVisibility,
		TextDecorationStyle,
		TextDecorationLine,
		TextRendering,
		VectorEffect,
		BackgroundImage,
		BackgroundSize,

		AnimationDirection, AnimationFillMode, AnimationIterationCount,
		AnimationName, AnimationPlayState, AnimationTimingFunction,

		Transform,
		Filter,

		BorderImageSource, BorderImageSlice, BorderImageWidth,
		BorderImageOutset, BorderImageRepeat,

		ClipPath,
		BackgroundOrigin,
		GridAutoFlow,
		RowGap,
		ColumnGap,
		OverflowWrap,
		BoxShadow,
		TransformOrigin,
		Appearance,
		Float,
		Clear,
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
		"min-width" => MinWidth(DimensionExtremity),
		"max-width" => MaxWidth(DimensionExtremity),
		"min-height" => MinHeight(DimensionExtremity),
		"max-height" => MaxHeight(DimensionExtremity),

		"top" => Top(Dimension),
		"right" => Right(Dimension),
		"left" => Left(Dimension),
		"bottom" => Bottom(Dimension),

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

css_macros::easy_enum! {box-sizing content-box border-box}
css_macros::easy_enum! {visibility visible hidden collapse}
css_macros::easy_enum! {display block none inline inline-block flex inline-flex grid}
css_macros::easy_enum! {user-select auto none text all}
css_macros::easy_enum! {scroll-behavior auto smooth}
css_macros::easy_enum! {pointer-events auto none}
css_macros::easy_enum! {resize none both horizontal vertical}
css_macros::easy_enum! {object-fit fill contain cover scale-down none}
css_macros::easy_enum! {transform-style flat preserve-3d}
css_macros::easy_enum! {background-blend-mode normal multiply screen overlay darken lighten color-dodge saturation color luminosity}
css_macros::easy_enum! {mix-blend-mode normal multiply screen overlay darken lighten color-dodge color-burn difference exclusion hue saturation color luminosity}
css_macros::easy_enum! {isolation auto isolate}
css_macros::easy_enum! {caption-side top bottom}
css_macros::easy_enum! {empty-cells show hide}
css_macros::easy_enum! {table-layout auto fixed}
css_macros::easy_enum! {all}
css_macros::easy_enum! {cursor auto alias all-scroll cell context-menu col-resize copy crosshair default e-resize ew-resize grab grabbing help move n-resize ne-resize nesw-resize ns-resize nw-resize nwse-resize no-drop none not-allowed pointer progress row-resize s-resize se-resize sw-resize text vertical-text w-resize wait zoom-in zoom-out}
css_macros::easy_enum! {content normal none counter open-quote close-quote no-open-quote no-close-quote [string]}
css_macros::easy_enum! {opacity [float]}
css_macros::easy_enum! {perspective none [unit]}
css_macros::easy_enum! {backface-visibility visible hidden}
css_macros::easy_enum! {overflow-x visible hidden scroll auto}
css_macros::easy_enum! {overflow-y visible hidden scroll auto}
css_macros::easy_enum! {float none left right inline-start inline-end}
css_macros::easy_enum! {clear none left right inline-start inline-end both}
