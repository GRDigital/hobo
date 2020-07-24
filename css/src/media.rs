use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Media {
	MinWidth(Unit),
	MaxWidth(Unit),
	MinAspectRatio(u32, u32),
	MaxAspectRatio(u32, u32),
}

impl ToString for Media {
	fn to_string(&self) -> String {
		match self {
			Self::MinWidth(unit) => format!("@media(min-width:{})", unit.to_string()),
			Self::MaxWidth(unit) => format!("@media(max-width:{})", unit.to_string()),
			Self::MinAspectRatio(width, height) => format!("@media(min-aspect-ratio:{}/{})", width, height),
			Self::MaxAspectRatio(width, height) => format!("@media(max-aspect-ratio:{}/{})", width, height),
		}
	}
}

// TODO: kind of hacky
// #[derive(Debug, PartialEq, Eq, Hash, Clone)]
// pub struct MediaMaxWidth(pub Unit);

// impl ToString for MediaMaxWidth {
//     fn to_string(&self) -> String {
//         format!("@media (max-width:{})", self.0.to_string())
//     }
// }
