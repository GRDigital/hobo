#[macro_use] mod flex;
#[macro_use] mod margin;
#[macro_use] mod padding;
#[macro_use] mod dimensions;
#[macro_use] mod position;
#[macro_use] mod text;
#[macro_use] mod border;
#[macro_use] mod background;

// use crate::prelude::*;
use std::string::ToString;
pub use flex::*;
pub use margin::*;
pub use padding::*;
pub use dimensions::*;
pub use position::*;
pub use text::*;
pub use border::*;
pub use background::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Property {
	Raw(String),
	Display(Display),
	MarginLeft(Margin),
	MarginRight(Margin),
	MarginTop(Margin),
	MarginBottom(Margin),
	PaddingLeft(Padding),
	PaddingRight(Padding),
	PaddingTop(Padding),
	PaddingBottom(Padding),
	Width(Dimension),
	Height(Dimension),
	MinWidth(DimensionExtremity),
	MaxWidth(DimensionExtremity),
	MinHeight(DimensionExtremity),
	MaxHeight(DimensionExtremity),
	BackgroundColor(BackgroundColor),
	FlexWrap(FlexWrap),
	FlexDirection(FlexDirection),
	JustifyContent(JustifyContent),
	AlignItems(AlignItems),
	AlignContent(AlignContent),
	AlignSelf(AlignSelf),
	FlexBasis(FlexBasis),
	FlexGrow(FlexGrow),
	FlexShrink(FlexShrink),
	Order(Order),
	Position(Position),
	Top(Dimension),
	Right(Dimension),
	Left(Dimension),
	Bottom(Dimension),
	BoxSizing(BoxSizing),
	Visibility(Visibility),
	ZIndex(ZIndex),
	Overflow(Overflow),
	Color(Color),
	Direction(Direction),
	UnicodeBidi(UnicodeBidi),
	WhiteSpace(WhiteSpace),
	WritingMode(WritingMode),
	HangingPunctuation(HangingPunctuation),
	Hyphens(Hyphens),
	TextAlign(TextAlign),
	TextAlignLast(TextAlignLast),
	TextJustify(TextJustify),
	FontStretch(FontStretch),
	UserSelect(UserSelect),
	ScrollBehavior(ScrollBehavior),
	PointerEvents(PointerEvents),
	Resize(Resize),
	ObjectFit(ObjectFit),
	ListStyleType(ListStyleType),
	PageBreakAfter(PageBreakAfter),
	PageBreakBefore(PageBreakBefore),
	PageBreakInside(PageBreakInside),
	FontVariant(FontVariant),
	WordBreak(WordBreak),
	WordWrap(WordWrap),
	FontStyle(FontStyle),
	TransformStyle(TransformStyle),
	BackgroundBlendMode(BackgroundBlendMode),
	MixBlendMode(MixBlendMode),
	Isolation(Isolation),
	CaptionSide(CaptionSide),
	EmptyCells(EmptyCells),
	TableLayout(TableLayout),
	BorderCollapse(BorderCollapse),
	All(All),
	FontWeight(FontWeight),
	FontSize(FontSize),
	BackgroundRepeat(BackgroundRepeat),
	BackgroundAttachment(BackgroundAttachment),
	Cursor(Cursor),
	TextTransform(TextTransform),
	FontKerning(FontKerning),
	FontFamily(FontFamily),
	WordSpacing(WordSpacing),
	TextIndent(TextIndent),
	TextOverflow(TextOverflow),
	VerticalAlign(VerticalAlign),
	LineHeight(LineHeight),
	LetterSpacing(LetterSpacing),
	TabSize(TabSize),
	BoxDecorationBreak(BoxDecorationBreak),
	OutlineWidth(OutlineWidth),
	OutlineColor(OutlineColor),
	Content(Content),
	Opacity(Opacity),
	Perspective(Perspective),
	BackfaceVisibility(BackfaceVisibility),
	// etc
}

impl ToString for Property {
	fn to_string(&self) -> String {
		match self {
			Self::Raw(x)                        => x.clone(),

			// different properties that essentially take the same argument
			Self::MarginLeft(x)    => format!("margin-left:{};", x.to_string()),
			Self::MarginRight(x)   => format!("margin-right:{};", x.to_string()),
			Self::MarginTop(x)     => format!("margin-top:{};", x.to_string()),
			Self::MarginBottom(x)  => format!("margin-bottom:{};", x.to_string()),
			Self::PaddingLeft(x)   => format!("padding-left:{};", x.to_string()),
			Self::PaddingRight(x)  => format!("padding-right:{};", x.to_string()),
			Self::PaddingTop(x)    => format!("padding-top:{};", x.to_string()),
			Self::PaddingBottom(x) => format!("padding-bottom:{};", x.to_string()),
			Self::Width(x)         => format!("width:{};", x.to_string()),
			Self::Height(x)        => format!("height:{};", x.to_string()),
			Self::MinWidth(x)      => format!("min-width:{};", x.to_string()),
			Self::MaxWidth(x)      => format!("max-width:{};", x.to_string()),
			Self::MinHeight(x)     => format!("min-height:{};", x.to_string()),
			Self::MaxHeight(x)     => format!("max-height:{};", x.to_string()),
			Self::Top(x)           => format!("top:{};", x.to_string()),
			Self::Right(x)         => format!("right:{};", x.to_string()),
			Self::Left(x)          => format!("left:{};", x.to_string()),
			Self::Bottom(x)        => format!("bottom:{};", x.to_string()),

			// different properties that have specific to them arguments
			// basis/grow/shrink/order kind of take the same, but basis and shrink are 1 by default while others are 0 so /shrug
			Self::FlexWrap(x)             => x.to_string(),
			Self::FlexDirection(x)        => x.to_string(),
			Self::JustifyContent(x)       => x.to_string(),
			Self::AlignItems(x)           => x.to_string(),
			Self::AlignContent(x)         => x.to_string(),
			Self::AlignSelf(x)            => x.to_string(),
			Self::FlexBasis(x)            => x.to_string(),
			Self::FlexGrow(x)             => x.to_string(),
			Self::FlexShrink(x)           => x.to_string(),
			Self::Order(x)                => x.to_string(),
			Self::Position(x)             => x.to_string(),
			Self::Display(x)              => x.to_string(),
			Self::BoxSizing(x)            => x.to_string(),
			Self::Visibility(x)           => x.to_string(),
			Self::ZIndex(x)               => x.to_string(),
			Self::Overflow(x)             => x.to_string(),
			Self::BackgroundColor(x)      => x.to_string(),
			Self::Color(x)                => x.to_string(),
			Self::Direction(x)            => x.to_string(),
			Self::UnicodeBidi(x)          => x.to_string(),
			Self::WhiteSpace(x)           => x.to_string(),
			Self::WritingMode(x)          => x.to_string(),
			Self::HangingPunctuation(x)   => x.to_string(),
			Self::Hyphens(x)              => x.to_string(),
			Self::TextAlign(x)            => x.to_string(),
			Self::TextAlignLast(x)        => x.to_string(),
			Self::TextJustify(x)          => x.to_string(),
			Self::FontStretch(x)          => x.to_string(),
			Self::UserSelect(x)           => x.to_string(),
			Self::ScrollBehavior(x)       => x.to_string(),
			Self::PointerEvents(x)        => x.to_string(),
			Self::Resize(x)               => x.to_string(),
			Self::ObjectFit(x)            => x.to_string(),
			Self::ListStyleType(x)        => x.to_string(),
			Self::PageBreakAfter(x)       => x.to_string(),
			Self::PageBreakBefore(x)      => x.to_string(),
			Self::PageBreakInside(x)      => x.to_string(),
			Self::FontVariant(x)          => x.to_string(),
			Self::WordBreak(x)            => x.to_string(),
			Self::WordWrap(x)             => x.to_string(),
			Self::FontStyle(x)            => x.to_string(),
			Self::TransformStyle(x)       => x.to_string(),
			Self::BackgroundBlendMode(x)  => x.to_string(),
			Self::MixBlendMode(x)         => x.to_string(),
			Self::Isolation(x)            => x.to_string(),
			Self::CaptionSide(x)          => x.to_string(),
			Self::EmptyCells(x)           => x.to_string(),
			Self::TableLayout(x)          => x.to_string(),
			Self::BorderCollapse(x)       => x.to_string(),
			Self::All(x)                  => x.to_string(),
			Self::FontWeight(x)           => x.to_string(),
			Self::FontSize(x)             => x.to_string(),
			Self::BackgroundRepeat(x)     => x.to_string(),
			Self::BackgroundAttachment(x) => x.to_string(),
			Self::Cursor(x)               => x.to_string(),
			Self::TextTransform(x)        => x.to_string(),
			Self::FontKerning(x)          => x.to_string(),
			Self::FontFamily(x)           => x.to_string(),
			Self::WordSpacing(x)          => x.to_string(),
			Self::TextIndent(x)           => x.to_string(),
			Self::TextOverflow(x)         => x.to_string(),
			Self::VerticalAlign(x)        => x.to_string(),
			Self::LineHeight(x)           => x.to_string(),
			Self::LetterSpacing(x)        => x.to_string(),
			Self::TabSize(x)              => x.to_string(),
			Self::BoxDecorationBreak(x)   => x.to_string(),
			Self::OutlineWidth(x)         => x.to_string(),
			Self::OutlineColor(x)         => x.to_string(),
			Self::Content(x)              => x.to_string(),
			Self::Opacity(x)              => x.to_string(),
			Self::Perspective(x)          => x.to_string(),
			Self::BackfaceVisibility(x)   => x.to_string(),
		}
	}
}

macro_rules! from_properties {
	($($name:ident),+$(,)*) => {$(
		impl From<$name> for Property {
			fn from(x: $name) -> Self { Self::$name(x) }
		}
	)+};
}

from_properties! {
	Display,
	FlexWrap,
	Position,
	BoxSizing,
	Visibility,
	Overflow,
	ZIndex,
}

css_macros::easy_enum!{box-sizing content-box border-box}
css_macros::easy_enum!{visibility visible hidden collapse}
css_macros::easy_enum!{overflow visible hidden scroll auto}
css_macros::easy_enum!{display block none inline inline-block flex inline-flex}
css_macros::easy_enum!{user-select auto none text all}
css_macros::easy_enum!{scroll-behavior auto smooth}
css_macros::easy_enum!{pointer-events auto none}
css_macros::easy_enum!{resize none both horizontal vertical}
css_macros::easy_enum!{object-fit fill contain cover scale-down none}
css_macros::easy_enum!{transform-style flat preserve-3d}
css_macros::easy_enum!{background-blend-mode normal multiply screen overlay darken lighten color-dodge saturation color luminosity}
css_macros::easy_enum!{mix-blend-mode normal multiply screen overlay darken lighten color-dodge color-burn difference exclusion hue saturation color luminosity}
css_macros::easy_enum!{isolation auto isolate}
css_macros::easy_enum!{caption-side top bottom}
css_macros::easy_enum!{empty-cells show hide}
css_macros::easy_enum!{table-layout auto fixed}
css_macros::easy_enum!{all unset}
css_macros::easy_enum!{cursor auto alias all-scroll cell context-menu col-resize copy crosshair default e-resize ew-resize grab grabbing help move n-resize ne-resize nesw-resize ns-resize nw-resize nwse-resize no-drop none not-allowed pointer progress row-resize s-resize se-resize sw-resize text vertical-text w-resize wait zoom-in zoom-out}
css_macros::easy_enum!{content normal none counter open-quote close-quote no-open-quote no-close-quote $}
css_macros::easy_enum!{opacity #}
css_macros::easy_enum!{perspective none @}
css_macros::easy_enum!{backface-visibility visible hidden}
