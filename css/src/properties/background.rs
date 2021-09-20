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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum MaskImage {
	None,
	Initial,
	Inherit,
	Unset,
	Some(Vec<crate::Image>),
}

impl std::fmt::Display for MaskImage {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::None    => "mask-image:none;-webkit-mask-image:none;".fmt(f),
			Self::Initial => "mask-image:initial;-webkit-mask-image:initial;".fmt(f),
			Self::Inherit => "mask-image:inherit;-webkit-mask-image:inherit;".fmt(f),
			Self::Unset   => "mask-image:unset;-webkit-mask-image:unset;".fmt(f),
			Self::Some(images) => {
				let write = |f: &mut std::fmt::Formatter<'_>| -> std::fmt::Result {
					if let Some((first, rest)) = images.split_first() {
						write!(f, "{}", first)?;
						for image in rest {
							write!(f, ",{}", image)?;
						}
					}

					Ok(())
				};
				"mask-image:".fmt(f)?; write(f)?; ";".fmt(f)?;
				"-webkit-mask-image:".fmt(f)?; write(f)?; ";".fmt(f)
			},
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MaskSize {
	Initial,
	Inherit,
	Unset,
	Auto,
	Cover,
	Contain,
}

#[rustfmt::skip]
impl std::fmt::Display for MaskSize {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Initial => "mask-size:initial;-webkit-mask-size:initial;".fmt(f),
			Self::Inherit => "mask-size:inherit;-webkit-mask-size:inherit;".fmt(f),
			Self::Unset   => "mask-size:unset;-webkit-mask-size:unset;".fmt(f),
			Self::Auto    => "mask-size:auto;-webkit-mask-size:auto;".fmt(f),
			Self::Cover   => "mask-size:cover;-webkit-mask-size:cover;".fmt(f),
			Self::Contain => "mask-size:contain;-webkit-mask-size:contain;".fmt(f),
		}
	}
}

#[macro_export]
macro_rules! mask_size {
	(initial) => {$crate::Property::MaskSize($crate::MaskSize::Initial)};
	(inherit) => {$crate::Property::MaskSize($crate::MaskSize::Inherit)};
	(unset)   => {$crate::Property::MaskSize($crate::MaskSize::Unset)};
	(auto)    => {$crate::Property::MaskSize($crate::MaskSize::Auto)};
	(cover)   => {$crate::Property::MaskSize($crate::MaskSize::Cover)};
	(contain) => {$crate::Property::MaskSize($crate::MaskSize::Contain)};
}
