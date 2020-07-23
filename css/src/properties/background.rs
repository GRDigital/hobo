use crate::prelude::*;

css_macros::easy_enum! {background-repeat repeat repeat-x repeat-y no-repeat}
css_macros::easy_enum! {background-attachment scroll fixed local}
css_macros::easy_enum! {background-size auto cover contain [unit]}
css_macros::easy_enum! {background-origin border-box padding-box content-box}
css_macros::easy_color! {background-color}
css_macros::unit_value_macro! {background_position_x BackgroundPositionX}
css_macros::unit_value_macro! {background_position_y BackgroundPositionY}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BackgroundImage {
	None,
	Initial,
	Inherit,
	Unset,
	Revert,
	Some(Vec<crate::Image>),
}

impl ToString for BackgroundImage {
	fn to_string(&self) -> String {
		match self {
			Self::None    => "background-image:none;".to_owned(),
			Self::Initial => "background-image:initial;".to_owned(),
			Self::Inherit => "background-image:inherit;".to_owned(),
			Self::Unset   => "background-image:unset;".to_owned(),
			Self::Revert  => "background-image:revert;".to_owned(),
			Self::Some(x) => format!("background-image:{};", x.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(",")),
		}
	}
}
