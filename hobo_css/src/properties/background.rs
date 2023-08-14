crate::macros::easy_enum! {background-repeat repeat repeat-x repeat-y no-repeat}
crate::macros::easy_enum! {background-attachment scroll fixed local}
crate::macros::easy_enum! {background-size auto cover contain [unit]}
crate::macros::easy_enum! {background-origin border-box padding-box content-box}
crate::macros::easy_enum! {background-clip border-box padding-box content-box text}
crate::macros::easy_enum! {-*-mask-size auto cover contain}
crate::macros::easy_color! {background-color}
crate::macros::unit_value_macro! {background_position_x}
crate::macros::unit_value_macro! {background_position_y}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum BackgroundImage {
	None,
	Initial,
	Inherit,
	Unset,
	Some(Vec<crate::Image>),
}

impl BackgroundImage {
	pub fn single(x: crate::Image) -> Self { Self::Some(vec![x]) }
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum MaskImage {
	None,
	Initial,
	Inherit,
	Unset,
	Some(Vec<crate::Image>),
}

impl MaskImage {
	pub fn single(x: crate::Image) -> Self { Self::Some(vec![x]) }
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
