#[macro_use] mod display;
#[macro_use] mod flex_wrap;
#[macro_use] mod margin;
#[macro_use] mod padding;
#[macro_use] mod dimensions;

use crate::prelude::*;
use std::string::ToString;
pub use display::*;
pub use flex_wrap::*;
pub use margin::*;
pub use padding::*;
pub use dimensions::*;

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
	// etc
}

impl ToString for Property {
	fn to_string(&self) -> String {
		match self {
			Self::None                          => "".to_owned(),
			Self::Combined(x)                   => x.iter().map(ToString::to_string).collect::<String>(),
			Self::Raw(x)                        => x.clone(),
			Self::Display(x)                    => x.to_string(),
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
			Self::BackgroundColor((r, g, b, a)) => format!("background-color:#{:02x}{:02x}{:02x}{:02x};", r, g, b, a),
			Self::FlexWrap(x)                   => x.to_string(),
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
}

#[macro_export]
macro_rules! background_color {
	($r:tt $g:tt $b:tt $a:tt) => { $crate::css::Property::BackgroundColor(($r, $g, $b, $a)) };
	($r:tt $g:tt $b:tt) => { $crate::css::Property::BackgroundColor(($r, $g, $b, 255)) };
}
