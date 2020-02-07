css_macros::easy_enum! {background-repeat repeat repeat-x repeat-y no-repeat}
css_macros::easy_enum! {background-attachment scroll fixed local}
css_macros::easy_enum! {background-image none [raw]} // TODO:
css_macros::easy_enum! {background-size auto cover contain @}
css_macros::easy_enum! {background-origin border-box padding-box content-box}
css_macros::easy_color! {background-color}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BackgroundPosition {
	Inherit,
	Initial,
	Unset,
	Revert,
	Some(Vec<[BackgroundPositionElement; 4]>),
}

#[rustfmt::skip]
impl ToString for BackgroundPosition {
	fn to_string(&self) -> String {
		match self {
			Self::Inherit => "background-position:inherit;".to_owned(),
			Self::Initial => "background-position:initial;".to_owned(),
			Self::Unset => "background-position:unset;".to_owned(),
			Self::Revert => "background-position:revert;".to_owned(),
			Self::Some(x) => format!("background-position:{};", x.iter().map(|x| x.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(" ")).collect::<Vec<_>>().join(",")),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BackgroundPositionElement {
	Left,
	Center,
	Right,
	Top,
	Bottom,
	Zero,
	Some(crate::Unit),
}

#[rustfmt::skip]
impl ToString for BackgroundPositionElement {
	fn to_string(&self) -> String {
		match self {
			Self::Left => "left".to_owned(),
			Self::Center => "center".to_owned(),
			Self::Right => "right".to_owned(),
			Self::Top => "top".to_owned(),
			Self::Bottom => "bottom".to_owned(),
			Self::Zero => "0".to_owned(),
			Self::Some(x) => x.to_string(),
		}
	}
}

#[rustfmt::skip]
#[macro_export]
#[doc(hidden)]
macro_rules! __bg_pos_elem {
	(left) => {$crate::BackgroundPositionElement::Left};
	(center) => {$crate::BackgroundPositionElement::Center};
	(right) => {$crate::BackgroundPositionElement::Right};
	(top) => {$crate::BackgroundPositionElement::Top};
	(bottom) => {$crate::BackgroundPositionElement::Bottom};
	(0) => {$crate::BackgroundPositionElement::Zero};
	(($($tt:tt)+)) => {$crate::BackgroundPositionElement::Some($crate::unit!($($tt)+))};
}

#[macro_export]
macro_rules! background_position {
	(inherit) => {$crate::Property::BackgroundPosition($crate::BackgroundPosition::Inherit)};
	(initial) => {$crate::Property::BackgroundPosition($crate::BackgroundPosition::Initial)};
	(unset)   => {$crate::Property::BackgroundPosition($crate::BackgroundPosition::Unset)};
	(revert)  => {$crate::Property::BackgroundPosition($crate::BackgroundPosition::Revert)};
	($($val1:tt $val2:tt $val3:tt $val4:tt),+$(,)*) => {
		$crate::Property::BackgroundPosition($crate::BackgroundPosition::Some(vec![$(
			[
				$crate::__bg_pos_elem!($val1),
				$crate::__bg_pos_elem!($val2),
				$crate::__bg_pos_elem!($val3),
				$crate::__bg_pos_elem!($val4),
			]
		),+]))
	};
}
