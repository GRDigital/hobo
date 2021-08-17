crate::macros::easy_enum! {background-repeat repeat repeat-x repeat-y no-repeat}
crate::macros::easy_enum! {background-attachment scroll fixed local}
crate::macros::easy_enum! {background-size auto cover contain [unit]}
crate::macros::easy_enum! {background-origin border-box padding-box content-box}
crate::macros::easy_enum! {background-clip border-box padding-box content-box text}
crate::macros::easy_color! {background-color}
crate::macros::unit_value_macro! {background_position_x BackgroundPositionX}
crate::macros::unit_value_macro! {background_position_y BackgroundPositionY}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BackgroundImage {
	None,
	Initial,
	Inherit,
	Unset,
	Some(Vec<crate::Image>),
}

impl std::fmt::Display for BackgroundImage {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::None    => "background-image:none;".fmt(f),
			Self::Initial => "background-image:initial;".fmt(f),
			Self::Inherit => "background-image:inherit;".fmt(f),
			Self::Unset   => "background-image:unset;".fmt(f),
			Self::Some(images) => {
				"background-image:".fmt(f)?;
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
