crate::macros::easy_enum! {background-repeat repeat repeat-x repeat-y no-repeat}
crate::macros::easy_enum! {background-attachment scroll fixed local}
crate::macros::easy_enum! {background-size auto cover contain [unit]}
crate::macros::easy_enum! {background-origin border-box padding-box content-box}
crate::macros::easy_enum! {-*-background-clip border-box padding-box content-box text}
crate::macros::easy_enum! {-*-mask-size auto cover contain}
crate::macros::easy_color! {background-color}
crate::macros::unit_value_macro! {background_position_x}
crate::macros::unit_value_macro! {background_position_y}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum background_image {
	none,
	initial,
	inherit,
	unset,
	Some(Vec<crate::Image>),
}

impl background_image {
	pub fn single(x: crate::Image) -> Self { Self::Some(vec![x]) }
	pub fn url(x: impl Into<String>) -> Self { Self::Some(vec![crate::Image::Url(x.into())]) }
}

impl std::fmt::Display for background_image {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::none    => "background-image:none;".fmt(f),
			Self::initial => "background-image:initial;".fmt(f),
			Self::inherit => "background-image:inherit;".fmt(f),
			Self::unset   => "background-image:unset;".fmt(f),
			Self::Some(images) => {
				"background-image:".fmt(f)?;
				if let Some((first, rest)) = images.split_first() {
					write!(f, "{first}")?;
					for image in rest {
						write!(f, ",{image}")?;
					}
				}
				";".fmt(f)
			},
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum mask_image {
	none,
	initial,
	inherit,
	unset,
	Some(Vec<crate::Image>),
}

impl mask_image {
	pub fn single(x: crate::Image) -> Self { Self::Some(vec![x]) }
	pub fn url(x: impl Into<String>) -> Self { Self::Some(vec![crate::Image::Url(x.into())]) }
}

impl std::fmt::Display for mask_image {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::none    => "mask-image:none;-webkit-mask-image:none;".fmt(f),
			Self::initial => "mask-image:initial;-webkit-mask-image:initial;".fmt(f),
			Self::inherit => "mask-image:inherit;-webkit-mask-image:inherit;".fmt(f),
			Self::unset   => "mask-image:unset;-webkit-mask-image:unset;".fmt(f),
			Self::Some(images) => {
				let write = |f: &mut std::fmt::Formatter<'_>| -> std::fmt::Result {
					if let Some((first, rest)) = images.split_first() {
						write!(f, "{first}")?;
						for image in rest {
							write!(f, ",{image}")?;
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
