#[macro_use] mod display;
#[macro_use] mod flex_wrap;
#[macro_use] mod margin;
#[macro_use] mod padding;
#[macro_use] mod dimensions;
#[macro_use] mod position;
#[macro_use] mod box_sizing;
#[macro_use] mod visibility;
#[macro_use] mod overflow;

// use crate::prelude::*;
use std::string::ToString;
pub use display::*;
pub use flex_wrap::*;
pub use margin::*;
pub use padding::*;
pub use dimensions::*;
pub use position::*;
pub use box_sizing::*;
pub use visibility::*;
pub use overflow::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Property {
	None,
	Combined(Vec<Property>), // TODO: this is maybe hack?
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
	BackgroundColor((u8, u8, u8, u8)),
	FlexWrap(FlexWrap),
	Position(Position),
	Top(Dimension),
	Right(Dimension),
	Left(Dimension),
	Bottom(Dimension),
	BoxSizing(BoxSizing),
	Visibility(Visibility),
	ZIndex(ZIndex),
	Overflow(Overflow),
	// etc
}

impl ToString for Property {
	fn to_string(&self) -> String {
		match self {
			Self::None                          => "".to_owned(),
			Self::Combined(x)                   => x.iter().map(ToString::to_string).collect::<String>(),
			Self::Raw(x)                        => x.clone(),

			Self::MarginLeft(x)                 => format!("margin-left:{};", x.to_string()),
			Self::MarginRight(x)                => format!("margin-right:{};", x.to_string()),
			Self::MarginTop(x)                  => format!("margin-top:{};", x.to_string()),
			Self::MarginBottom(x)               => format!("margin-bottom:{};", x.to_string()),
			Self::PaddingLeft(x)                => format!("padding-left:{};", x.to_string()),
			Self::PaddingRight(x)               => format!("padding-right:{};", x.to_string()),
			Self::PaddingTop(x)                 => format!("padding-top:{};", x.to_string()),
			Self::PaddingBottom(x)              => format!("padding-bottom:{};", x.to_string()),
			Self::Width(x)                      => format!("width:{};", x.to_string()),
			Self::Height(x)                     => format!("height:{};", x.to_string()),
			Self::MinWidth(x)                   => format!("min-width:{};", x.to_string()),
			Self::MaxWidth(x)                   => format!("max-width:{};", x.to_string()),
			Self::MinHeight(x)                  => format!("min-height:{};", x.to_string()),
			Self::MaxHeight(x)                  => format!("max-height:{};", x.to_string()),
			Self::Top(x)                        => format!("top:{};", x.to_string()),
			Self::Right(x)                      => format!("right:{};", x.to_string()),
			Self::Left(x)                       => format!("left:{};", x.to_string()),
			Self::Bottom(x)                     => format!("bottom:{};", x.to_string()),

			Self::BackgroundColor((r, g, b, a)) => format!("background-color:#{:02x}{:02x}{:02x}{:02x};", r, g, b, a),

			Self::FlexWrap(x)                   => x.to_string(),
			Self::Position(x)                   => x.to_string(),
			Self::Display(x)                    => x.to_string(),
			Self::BoxSizing(x)                  => x.to_string(),
			Self::Visibility(x)                 => x.to_string(),
			Self::ZIndex(x)                     => x.to_string(),
			Self::Overflow(x)                   => x.to_string(),
		}
	}
}

impl Into<Property> for Vec<Property> {
	fn into(self) -> Property {
		Property::Combined(self)
	}
}

impl Into<Property> for () {
	fn into(self) -> Property {
		Property::None
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

#[macro_export]
macro_rules! background_color {
	($r:tt $g:tt $b:tt $a:tt) => { $crate::css::Property::BackgroundColor(($r, $g, $b, $a)) };
	($r:tt $g:tt $b:tt) => { $crate::css::Property::BackgroundColor(($r, $g, $b, 255)) };
}
