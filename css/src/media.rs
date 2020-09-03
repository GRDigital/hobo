use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, strum::Display)]
pub enum MediaType {
	#[strum(to_string = "all")] All,
	#[strum(to_string = "print")] Print,
	#[strum(to_string = "screen")] Screen,
	#[strum(to_string = "speech")] Speech,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, strum::Display)]
pub enum Orientation {
	#[strum(to_string = "portrait")] Portrait,
	#[strum(to_string = "landscape")] Landscape,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, strum::Display)]
pub enum Scan {
	#[strum(to_string = "interlace")] Interlace,
	#[strum(to_string = "progressive")] Progressive,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Nottable<T: std::fmt::Debug + PartialEq + Eq + std::hash::Hash + Clone + std::fmt::Display> {
	pub not: bool,
	pub data: T,
}

impl<T: std::fmt::Debug + PartialEq + Eq + std::hash::Hash + Clone + std::fmt::Display> std::fmt::Display for Nottable<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.not {
			write!(f, "(not ({}))", self.data)
		} else {
			write!(f, "({})", self.data)
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum MediaFeature {
	AspectRatio(u32, u32), MinAspectRatio(u32, u32), MaxAspectRatio(u32, u32),
	Color(u32), MinColor(u32), MaxColor(u32),
	Monochrome(u32), MinMonochrome(u32), MaxMonochrome(u32),
	Width(Unit), MinWidth(Unit), MaxWidth(Unit),
	Height(Unit), MinHeight(Unit), MaxHeight(Unit),
	Resolution(u32), MinResolution(u32), MaxResolution(u32),
	Orientation(Orientation), Scan(Scan),
}

impl std::fmt::Display for MediaFeature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::AspectRatio(w, h)    => write!(f, "aspect-ratio:{}/{}", w, h),
			Self::MinAspectRatio(w, h) => write!(f, "min-aspect-ratio:{}/{}", w, h),
			Self::MaxAspectRatio(w, h) => write!(f, "max-aspect-ratio:{}/{}", w, h),

			Self::Color(x)             => write!(f, "color:{}", x),
			Self::MinColor(x)          => write!(f, "min-color:{}", x),
			Self::MaxColor(x)          => write!(f, "max-color:{}", x),

			Self::Monochrome(x)        => write!(f, "monochrome:{}", x),
			Self::MinMonochrome(x)     => write!(f, "min-monochrome:{}", x),
			Self::MaxMonochrome(x)     => write!(f, "max-monochrome:{}", x),

			Self::Width(x)             => write!(f, "width:{}", x),
			Self::MinWidth(x)          => write!(f, "min-width:{}", x),
			Self::MaxWidth(x)          => write!(f, "max-width:{}", x),

			Self::Height(x)            => write!(f, "height:{}", x),
			Self::MinHeight(x)         => write!(f, "min-height:{}", x),
			Self::MaxHeight(x)         => write!(f, "max-height:{}", x),

			Self::Resolution(x)        => write!(f, "resolution:{}", x),
			Self::MinResolution(x)     => write!(f, "min-resolution:{}", x),
			Self::MaxResolution(x)     => write!(f, "max-resolution:{}", x),

			Self::Orientation(x)       => write!(f, "orientation:{}", x),
			Self::Scan(x)              => write!(f, "scan:{}", x),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct MediaQuery {
	pub media: Nottable<MediaType>,
	pub features: Vec<Nottable<MediaFeature>>,
}

impl std::fmt::Display for MediaQuery {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.media.not { "not ".fmt(f)? }
		self.media.data.fmt(f)?;
		for feature in &self.features {
			write!(f, " and {}", feature)?;
		}
		Ok(())
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct MediaSelector(pub Vec<MediaQuery>);

impl std::fmt::Display for MediaSelector {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some((first, rest)) = self.0.split_first() {
			first.fmt(f)?;
			for query in rest {
				write!(f, ",{}", query)?;
			}
		}
		Ok(())
	}
}

#[test]
fn woo() {
	assert_eq!(
		macros::media_query!(!All && Orientation(Portrait) && !AspectRatio(4, 3)),
		MediaQuery {
			media: Nottable { not: true, data: MediaType::All },
			features: vec![
				Nottable { not: false, data: MediaFeature::Orientation(Orientation::Portrait) },
				Nottable { not: true, data: MediaFeature::AspectRatio(4, 3) },
			],
		},
	);

	assert_eq!(
		macros::media_selector!(
			!All && Orientation(Portrait) && !AspectRatio(4, 3),
			Print && Color(4) && !Width(crate::Unit::Px(crate::F32::new_unwrap(200.)))
		),
		MediaSelector(
			vec![
				macros::media_query!(!All && Orientation(Portrait) && !AspectRatio(4, 3)),
				macros::media_query!(Print && Color(4) && !Width(crate::Unit::Px(crate::F32::new_unwrap(200.)))),
			],
		),
	);

	assert_eq!(
		crate::style!(
			@media !All && Orientation(Portrait) && !AspectRatio(4, 3), Print && Color(4) && !Width(crate::Unit::Px(crate::F32::new_unwrap(200.))) {
				html {
					background_color!(rgb 0xFF_00_00)
				}
			}
		),
		crate::Style(
			vec![
				crate::Rule::Media(
					macros::media_selector!(!All && Orientation(Portrait) && !AspectRatio(4, 3), Print && Color(4) && !Width(crate::Unit::Px(crate::F32::new_unwrap(200.)))),
					crate::style!(
						html {
							background_color!(rgb 0xFF_00_00)
						}
					),
				),
			],
		),
	);

	assert_eq!(
		crate::style!(
			@media !All && Orientation(Portrait) && !AspectRatio(4, 3), Print && Color(4) && !Width(crate::Unit::Px(crate::F32::new_unwrap(200.))) {
				html {
					background_color!(rgb 0xFF_00_00)
				}
			}
		).to_string(),
		"@media not all and (orientation:portrait) and (not (aspect-ratio:4/3)),print and (color:4) and (not (width:200px)){html{background-color:#ff0000ff;}}",
	);
}
