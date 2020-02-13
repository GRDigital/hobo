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

use crate::prelude::*;
pub use background::*;
pub use border::*;
pub use dimensions::*;
pub use flex::*;
pub use margin::*;
pub use padding::*;
pub use position::*;
use std::string::ToString;
pub use svg::*;
pub use text::*;
pub use animation::*;
pub use transform::*;
pub use filter::*;
pub use grid::*;

pub use Property::{
	MarginLeft,
	MarginRight,
	MarginTop,
	MarginBottom,
	PaddingLeft,
	PaddingRight,
	PaddingTop,
	PaddingBottom,
	Width,
	Height,
	MinWidth,
	MaxWidth,
	MinHeight,
	MaxHeight,
	Top,
	Right,
	Left,
	Bottom,
	BorderLeftColor,
	BorderRightColor,
	BorderTopColor,
	BorderBottomColor,
	BorderLeftStyle,
	BorderRightStyle,
	BorderTopStyle,
	BorderBottomStyle,
	BorderLeftWidth,
	BorderRightWidth,
	BorderTopWidth,
	BorderBottomWidth,
	BorderTopLeftRadius,
	BorderTopRightRadius,
	BorderBottomLeftRadius,
	BorderBottomRightRadius,
	BackgroundColor,
	Color,
	TextDecorationColor,
	Fill,
	Stroke,
	OutlineColor,
	GridTemplateColumns,
	GridTemplateRows,
	GridAutoColumns,
	GridAutoRows,
	TextIndent,
	OutlineOffset,
};


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ColorValue {
	Rgba(u8, u8, u8, u8),
	Initial,
	Inherit,
	Unset,
	Revert,
}

impl From<(u8, u8, u8, u8)> for ColorValue {
	fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
		Self::Rgba(r, g, b, a)
	}
}

impl From<(u8, u8, u8)> for ColorValue {
	fn from((r, g, b): (u8, u8, u8)) -> Self {
		Self::Rgba(r, g, b, 1)
	}
}

impl From<u8> for ColorValue {
	fn from(rgb: u8) -> Self {
		Self::Rgba(rgb, rgb, rgb, 1)
	}
}

#[rustfmt::skip]
impl ToString for ColorValue {
	fn to_string(&self) -> String {
		match self {
			Self::Rgba(r, g, b, a) => format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a),
			Self::Initial          => "initial".to_owned(),
			Self::Inherit          => "inherit".to_owned(),
			Self::Unset            => "unset".to_owned(),
			Self::Revert           => "revert".to_owned(),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum UnitValue {
	Zero,
	Unit(Unit),
	Initial,
	Inherit,
	Unset,
	Revert,
}

#[rustfmt::skip]
impl ToString for UnitValue {
	fn to_string(&self) -> String {
		match self {
			Self::Zero    => "0".to_owned(),
			Self::Unit(x) => x.to_string(),
			Self::Initial => "initial".to_owned(),
			Self::Inherit => "inherit".to_owned(),
			Self::Unset   => "unset".to_owned(),
			Self::Revert  => "revert".to_owned(),
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
	pub stop_list: Vec<((u8, u8, u8, u8), Unit)>,
}

impl ToString for LinearGradient {
	fn to_string(&self) -> String {
		format!("{}deg,{}", self.angle, self.stop_list.iter().map(|(color, stop)| format!("#{:02x}{:02x}{:02x}{:02x} {}", color.0, color.1, color.2, color.3, stop.to_string())).collect::<Vec<_>>().join(","))
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

impl ToString for Image {
	fn to_string(&self) -> String {
		match self {
			Self::Url(x) => format!(r#"url("{}")"#, x),
			Self::LinearGradient(x) => format!("linear-gradient({})", x.to_string()),
			Self::RepeatingLinearGradient(x) => format!("repeating-linear-gradient({})", x.to_string()),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Property {
	Raw(String),
	All(All),

	Display(Display), BoxSizing(BoxSizing), Visibility(Visibility),
	OverflowX(OverflowX), OverflowY(OverflowY),
	Opacity(Opacity),

	// dimensions
	MarginLeft(Margin), MarginRight(Margin), MarginTop(Margin), MarginBottom(Margin),
	PaddingLeft(UnitValue), PaddingRight(UnitValue), PaddingTop(UnitValue), PaddingBottom(UnitValue),
	Width(Dimension), Height(Dimension),
	MinWidth(DimensionExtremity), MaxWidth(DimensionExtremity), MinHeight(DimensionExtremity), MaxHeight(DimensionExtremity),

	// flex
	FlexWrap(FlexWrap), FlexDirection(FlexDirection), JustifyContent(JustifyContent),
	AlignItems(AlignItems), AlignContent(AlignContent), AlignSelf(AlignSelf),
	FlexBasis(FlexBasis), FlexGrow(FlexGrow), FlexShrink(FlexShrink), Order(Order),

	// position
	Position(Position), Top(Dimension), Right(Dimension), Left(Dimension), Bottom(Dimension), ZIndex(ZIndex),

	// grid
	GridTemplateColumns(GridTemplate), GridTemplateRows(GridTemplate), RowGap(RowGap), ColumnGap(ColumnGap),
	GridColumnStart(GridColumnStart), GridColumnEnd(GridColumnEnd), GridRowStart(GridRowStart), GridRowEnd(GridRowEnd),
	GridAutoFlow(GridAutoFlow), GridAutoRows(GridAuto), GridAutoColumns(GridAuto),

	// background
	BackgroundColor(ColorValue), BackgroundBlendMode(BackgroundBlendMode),
	BackgroundRepeat(BackgroundRepeat), BackgroundAttachment(BackgroundAttachment),
	BackgroundImage(BackgroundImage), BackgroundSize(BackgroundSize),
	BackgroundPosition(BackgroundPosition), BackgroundOrigin(BackgroundOrigin),

	// border
	BorderLeftColor(ColorValue), BorderRightColor(ColorValue), BorderTopColor(ColorValue), BorderBottomColor(ColorValue),
	BorderLeftStyle(BorderStyle), BorderRightStyle(BorderStyle), BorderTopStyle(BorderStyle), BorderBottomStyle(BorderStyle),
	BorderLeftWidth(BorderWidth), BorderRightWidth(BorderWidth), BorderTopWidth(BorderWidth), BorderBottomWidth(BorderWidth),
	BorderTopLeftRadius(UnitValue), BorderTopRightRadius(UnitValue), BorderBottomLeftRadius(UnitValue), BorderBottomRightRadius(UnitValue),
	BorderImageSource(BorderImageSource), BorderImageSlice(BorderImageSlice), BorderImageWidth(BorderImageWidth), BorderImageOutset(BorderImageOutset),
	BorderImageRepeat(BorderImageRepeat),
	BorderCollapse(BorderCollapse),
	OutlineWidth(OutlineWidth), OutlineColor(ColorValue), OutlineStyle(OutlineStyle),
	OutlineOffset(UnitValue),
	BoxShadow(BoxShadow),

	// animation
	AnimationDirection(AnimationDirection),
	AnimationFillMode(AnimationFillMode),
	AnimationIterationCount(AnimationIterationCount),
	AnimationName(AnimationName),
	AnimationPlayState(AnimationPlayState),
	AnimationTimingFunction(AnimationTimingFunction),
	// AnimationDuration(AnimationDuration),
	// AnimationDelay(AnimationDelay),

	// svg
	Fill(ColorValue), Stroke(ColorValue),
	VectorEffect(VectorEffect),
	ClipPath(ClipPath),

	// pointer/cursor-related
	UserSelect(UserSelect), Resize(Resize),
	Cursor(Cursor), ScrollBehavior(ScrollBehavior), PointerEvents(PointerEvents),

	// text
	Color(ColorValue),
	Direction(Direction),
	UnicodeBidi(UnicodeBidi),
	WhiteSpace(WhiteSpace),
	WritingMode(WritingMode),
	HangingPunctuation(HangingPunctuation),
	Hyphens(Hyphens),
	TextAlign(TextAlign), TextAlignLast(TextAlignLast), TextJustify(TextJustify),
	BreakAfter(BreakAfter), BreakBefore(BreakBefore), BreakInside(BreakInside),
	WordBreak(WordBreak), WordWrap(WordWrap),
	FontFamily(FontFamily), FontStyle(FontStyle), FontVariant(FontVariant), FontSize(FontSize),
	FontWeight(FontWeight), FontStretch(FontStretch), FontKerning(FontKerning),
	TextTransform(TextTransform),
	WordSpacing(WordSpacing),
	TextIndent(UnitValue),
	TextOverflow(TextOverflow),
	OverflowWrap(OverflowWrap),
	VerticalAlign(VerticalAlign),
	LineHeight(LineHeight),
	LetterSpacing(LetterSpacing),
	TabSize(TabSize),
	TextDecorationStyle(TextDecorationStyle), TextDecorationLine(TextDecorationLine), TextDecorationColor(ColorValue), TextRendering(TextRendering),

	// etc
	BoxDecorationBreak(BoxDecorationBreak),
	ListStyleType(ListStyleType),
	MixBlendMode(MixBlendMode),
	Isolation(Isolation),
	CaptionSide(CaptionSide),
	EmptyCells(EmptyCells),
	TableLayout(TableLayout),
	Content(Content),
	ObjectFit(ObjectFit),
	Transform(Transform), TransformStyle(TransformStyle),
	Filter(Filter),
	BackfaceVisibility(BackfaceVisibility),
	Perspective(Perspective),
}

#[rustfmt::skip]
impl ToString for Property {
	fn to_string(&self) -> String {
		match self {
			Self::Raw(x) => x.clone(),

			// different properties that take the same argument
			Self::MarginLeft(x)              => format!("margin-left:{};", x.to_string()),
			Self::MarginRight(x)             => format!("margin-right:{};", x.to_string()),
			Self::MarginTop(x)               => format!("margin-top:{};", x.to_string()),
			Self::MarginBottom(x)            => format!("margin-bottom:{};", x.to_string()),

			Self::PaddingLeft(x)             => format!("padding-left:{};", x.to_string()),
			Self::PaddingRight(x)            => format!("padding-right:{};", x.to_string()),
			Self::PaddingTop(x)              => format!("padding-top:{};", x.to_string()),
			Self::PaddingBottom(x)           => format!("padding-bottom:{};", x.to_string()),

			Self::Width(x)                   => format!("width:{};", x.to_string()),
			Self::Height(x)                  => format!("height:{};", x.to_string()),
			Self::MinWidth(x)                => format!("min-width:{};", x.to_string()),
			Self::MaxWidth(x)                => format!("max-width:{};", x.to_string()),
			Self::MinHeight(x)               => format!("min-height:{};", x.to_string()),
			Self::MaxHeight(x)               => format!("max-height:{};", x.to_string()),

			Self::Top(x)                     => format!("top:{};", x.to_string()),
			Self::Right(x)                   => format!("right:{};", x.to_string()),
			Self::Left(x)                    => format!("left:{};", x.to_string()),
			Self::Bottom(x)                  => format!("bottom:{};", x.to_string()),

			Self::BorderLeftColor(x)         => format!("border-left-color:{};", x.to_string()),
			Self::BorderRightColor(x)        => format!("border-right-color:{};", x.to_string()),
			Self::BorderTopColor(x)          => format!("border-top-color:{};", x.to_string()),
			Self::BorderBottomColor(x)       => format!("border-bottom-color:{};", x.to_string()),

			Self::BorderLeftStyle(x)         => format!("border-left-style:{};", x.to_string()),
			Self::BorderRightStyle(x)        => format!("border-right-style:{};", x.to_string()),
			Self::BorderTopStyle(x)          => format!("border-top-style:{};", x.to_string()),
			Self::BorderBottomStyle(x)       => format!("border-bottom-style:{};", x.to_string()),

			Self::BorderLeftWidth(x)         => format!("border-left-width:{};", x.to_string()),
			Self::BorderRightWidth(x)        => format!("border-right-width:{};", x.to_string()),
			Self::BorderTopWidth(x)          => format!("border-top-width:{};", x.to_string()),
			Self::BorderBottomWidth(x)       => format!("border-bottom-width:{};", x.to_string()),

			Self::BorderTopLeftRadius(x)     => format!("border-top-left-radius:{};", x.to_string()),
			Self::BorderTopRightRadius(x)    => format!("border-top-right-radius:{};", x.to_string()),
			Self::BorderBottomLeftRadius(x)  => format!("border-bottom-left-radius:{};", x.to_string()),
			Self::BorderBottomRightRadius(x) => format!("border-bottom-right-radius:{};", x.to_string()),

			Self::BackgroundColor(x)         => format!("background-color:{};", x.to_string()),
			Self::Color(x)                   => format!("color:{};", x.to_string()),
			Self::TextDecorationColor(x)     => format!("text-decoration-color:{};", x.to_string()),
			Self::Fill(x)                    => format!("fill:{};", x.to_string()),
			Self::Stroke(x)                  => format!("stroke:{};", x.to_string()),
			Self::OutlineColor(x)            => format!("outline-color:{};", x.to_string()),

			Self::GridTemplateColumns(x)     => format!("grid-template-columns:{};", x.to_string()),
			Self::GridTemplateRows(x)        => format!("grid-template-rows:{};", x.to_string()),
			Self::GridAutoColumns(x)         => format!("grid-auto-columns:{};", x.to_string()),
			Self::GridAutoRows(x)            => format!("grid-auto-rows:{};", x.to_string()),

			Self::TextIndent(x)              => format!("text-indent:{};", x.to_string()),
			Self::OutlineOffset(x)           => format!("outline-offset:{};", x.to_string()),

			// different properties that have specific to them arguments
			// basis/grow/shrink/order kind of take the same, but basis and shrink are 1 by default while others are 0 so /shrug
			Self::FlexWrap(x)                => x.to_string(),
			Self::FlexDirection(x)           => x.to_string(),
			Self::JustifyContent(x)          => x.to_string(),
			Self::AlignItems(x)              => x.to_string(),
			Self::AlignContent(x)            => x.to_string(),
			Self::AlignSelf(x)               => x.to_string(),
			Self::FlexBasis(x)               => x.to_string(),
			Self::FlexGrow(x)                => x.to_string(),
			Self::FlexShrink(x)              => x.to_string(),
			Self::Order(x)                   => x.to_string(),
			Self::Position(x)                => x.to_string(),
			Self::Display(x)                 => x.to_string(),
			Self::BoxSizing(x)               => x.to_string(),
			Self::Visibility(x)              => x.to_string(),
			Self::ZIndex(x)                  => x.to_string(),
			Self::OverflowX(x)               => x.to_string(),
			Self::OverflowY(x)               => x.to_string(),
			Self::Direction(x)               => x.to_string(),
			Self::UnicodeBidi(x)             => x.to_string(),
			Self::WhiteSpace(x)              => x.to_string(),
			Self::WritingMode(x)             => x.to_string(),
			Self::HangingPunctuation(x)      => x.to_string(),
			Self::Hyphens(x)                 => x.to_string(),
			Self::TextAlign(x)               => x.to_string(),
			Self::TextAlignLast(x)           => x.to_string(),
			Self::TextJustify(x)             => x.to_string(),
			Self::FontStretch(x)             => x.to_string(),
			Self::UserSelect(x)              => x.to_string(),
			Self::ScrollBehavior(x)          => x.to_string(),
			Self::PointerEvents(x)           => x.to_string(),
			Self::Resize(x)                  => x.to_string(),
			Self::ObjectFit(x)               => x.to_string(),
			Self::ListStyleType(x)           => x.to_string(),
			Self::BreakAfter(x)              => x.to_string(),
			Self::BreakBefore(x)             => x.to_string(),
			Self::BreakInside(x)             => x.to_string(),
			Self::FontVariant(x)             => x.to_string(),
			Self::WordBreak(x)               => x.to_string(),
			Self::WordWrap(x)                => x.to_string(),
			Self::FontStyle(x)               => x.to_string(),
			Self::TransformStyle(x)          => x.to_string(),
			Self::BackgroundBlendMode(x)     => x.to_string(),
			Self::MixBlendMode(x)            => x.to_string(),
			Self::Isolation(x)               => x.to_string(),
			Self::CaptionSide(x)             => x.to_string(),
			Self::EmptyCells(x)              => x.to_string(),
			Self::TableLayout(x)             => x.to_string(),
			Self::BorderCollapse(x)          => x.to_string(),
			Self::All(x)                     => x.to_string(),
			Self::FontWeight(x)              => x.to_string(),
			Self::FontSize(x)                => x.to_string(),
			Self::BackgroundRepeat(x)        => x.to_string(),
			Self::BackgroundAttachment(x)    => x.to_string(),
			Self::Cursor(x)                  => x.to_string(),
			Self::TextTransform(x)           => x.to_string(),
			Self::FontKerning(x)             => x.to_string(),
			Self::FontFamily(x)              => x.to_string(),
			Self::WordSpacing(x)             => x.to_string(),
			Self::TextOverflow(x)            => x.to_string(),
			Self::VerticalAlign(x)           => x.to_string(),
			Self::LineHeight(x)              => x.to_string(),
			Self::LetterSpacing(x)           => x.to_string(),
			Self::TabSize(x)                 => x.to_string(),
			Self::BoxDecorationBreak(x)      => x.to_string(),
			Self::OutlineWidth(x)            => x.to_string(),
			Self::OutlineStyle(x)            => x.to_string(),
			Self::Content(x)                 => x.to_string(),
			Self::Opacity(x)                 => x.to_string(),
			Self::Perspective(x)             => x.to_string(),
			Self::BackfaceVisibility(x)      => x.to_string(),
			Self::TextDecorationStyle(x)     => x.to_string(),
			Self::TextDecorationLine(x)      => x.to_string(),
			Self::TextRendering(x)           => x.to_string(),
			Self::VectorEffect(x)            => x.to_string(),
			Self::BackgroundImage(x)         => x.to_string(),
			Self::BackgroundSize(x)          => x.to_string(),
			Self::AnimationDirection(x)      => x.to_string(),
			Self::AnimationFillMode(x)       => x.to_string(),
			Self::AnimationIterationCount(x) => x.to_string(),
			Self::AnimationName(x)           => x.to_string(),
			Self::AnimationPlayState(x)      => x.to_string(),
			Self::AnimationTimingFunction(x) => x.to_string(),
			Self::Transform(x)               => x.to_string(),
			Self::Filter(x)                  => x.to_string(),
			Self::BorderImageSource(x)       => x.to_string(),
			Self::BorderImageSlice(x)        => x.to_string(),
			Self::BorderImageWidth(x)        => x.to_string(),
			Self::BorderImageOutset(x)       => x.to_string(),
			Self::BorderImageRepeat(x)       => x.to_string(),
			Self::ClipPath(x)                => x.to_string(),
			Self::BackgroundPosition(x)      => x.to_string(),
			Self::BackgroundOrigin(x)        => x.to_string(),
			Self::GridAutoFlow(x)            => x.to_string(),
			Self::RowGap(x)                  => x.to_string(),
			Self::ColumnGap(x)               => x.to_string(),
			Self::GridColumnStart(x)         => x.to_string(),
			Self::GridColumnEnd(x)           => x.to_string(),
			Self::GridRowStart(x)            => x.to_string(),
			Self::GridRowEnd(x)              => x.to_string(),
			Self::OverflowWrap(x)            => x.to_string(),
			Self::BoxShadow(x)               => x.to_string(),
		}
	}
}

macro_rules! from_properties {
	($($name:ident),+$(,)*) => {$(
		impl From<$name> for Property {
			fn from(x: $name) -> Self { Self::$name(x) }
		}

		impl crate::AppendProperty for $name {
			fn append_property(self, decls: &mut Vec<Property>) { decls.push(Property::$name(self)); }
		}
	)+};
}

from_properties! {
	FlexWrap,
	FlexDirection,
	JustifyContent,
	AlignItems,
	AlignContent,
	AlignSelf,
	FlexBasis,
	FlexGrow,
	FlexShrink,
	Order,
	Position,
	Display,
	BoxSizing,
	Visibility,
	ZIndex,
	OverflowX,
	OverflowY,
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
	ListStyleType,
	BreakAfter,
	BreakBefore,
	BreakInside,
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
	AnimationDirection,
	AnimationFillMode,
	AnimationIterationCount,
	AnimationName,
	AnimationPlayState,
	AnimationTimingFunction,
	Transform,
	Filter,
	BorderImageSource,
	BorderImageSlice,
	BorderImageWidth,
	BorderImageOutset,
	BorderImageRepeat,
	ClipPath,
	BackgroundPosition,
	BackgroundOrigin,
	GridAutoFlow,
	RowGap,
	ColumnGap,
	GridColumnStart,
	GridColumnEnd,
	GridRowStart,
	GridRowEnd,
	BoxShadow,
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
css_macros::easy_enum! {content normal none counter open-quote close-quote no-open-quote no-close-quote $}
css_macros::easy_enum! {opacity [float]}
css_macros::easy_enum! {perspective none @}
css_macros::easy_enum! {backface-visibility visible hidden}
css_macros::easy_enum! {overflow-x visible hidden scroll auto}
css_macros::easy_enum! {overflow-y visible hidden scroll auto}
css_macros::easy_enum! {clip-path none margin-box border-box padding-box content-box fill-box stroke-box view-box [raw]}
