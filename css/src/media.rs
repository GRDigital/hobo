use crate::prelude::*;

// @media rundown:
// media/not media
// bunch of queries/not queries

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum MediaType {
	All,
	Print,
	Screen,
	Speech,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Orientation {
	Portrait,
	Landscape,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Scan {
	Interlace,
	Progressive,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Nottable<T: std::fmt::Debug + PartialEq + Eq + std::hash::Hash + Clone> {
	not: bool,
	data: T,
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct MediaQuery {
	media: Nottable<MediaType>,
	features: Vec<Nottable<MediaFeature>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct MediaSelector(pub Vec<MediaQuery>);

#[test]
fn woo() {
	assert_eq!(
		css_macros_decl::media_query!(!All && Orientation(Portrait) && !AspectRatio(4, 3)),
		MediaQuery {
			media: Nottable { not: true, data: MediaType::All },
			features: vec![
				Nottable { not: false, data: MediaFeature::Orientation(Orientation::Portrait) },
				Nottable { not: true, data: MediaFeature::AspectRatio(4, 3) },
			],
		},
	);

	assert_eq!(
		css_macros_decl::media_selector!(
			!All && Orientation(Portrait) && !AspectRatio(4, 3),
			Print && Color(4) && !Width(crate::Unit::Px(crate::new_f32(200.)))
		),
		MediaSelector(
			vec![
				css_macros_decl::media_query!(!All && Orientation(Portrait) && !AspectRatio(4, 3)),
				css_macros_decl::media_query!(Print && Color(4) && !Width(crate::Unit::Px(crate::new_f32(200.)))),
			],
		),
	);

	assert_eq!(
		crate::style!(
			@media !All && Orientation(Portrait) && !AspectRatio(4, 3), Print && Color(4) && !Width(crate::Unit::Px(crate::new_f32(200.))) {
				html {
					background_color!(rgb 0xFF_00_00)
				}
			}
		),
		crate::Style(
			vec![
				crate::Rule::Media(
					css_macros_decl::media_selector!(!All && Orientation(Portrait) && !AspectRatio(4, 3), Print && Color(4) && !Width(crate::Unit::Px(crate::new_f32(200.)))),
					crate::style!(
						html {
							background_color!(rgb 0xFF_00_00)
						}
					),
				),
			],
		),
	);
}

/*
// add and when transitioning to ConditionBuilder
pub struct ConditionStart;
impl ConditionStart {
	fn not() -> ConditionStartNot { ConditionStartNot(vec![Component::InitialNot]) }
	fn all(self) -> ConditionBuilder { ConditionBuilder(vec![Component::All]) }
	fn print(self) -> ConditionBuilder { ConditionBuilder(vec![Component::Print]) }
	fn screen(self) -> ConditionBuilder { ConditionBuilder(vec![Component::Screen]) }
	fn speech(self) -> ConditionBuilder { ConditionBuilder(vec![Component::Speech]) }
}

pub struct ConditionStartNot(Vec<Component>);
impl ConditionStartNot {
	fn all(mut self) -> ConditionBuilder { self.0.push(Component::All); ConditionBuilder(self.0) }
	fn print(mut self) -> ConditionBuilder { self.0.push(Component::Print); ConditionBuilder(self.0) }
	fn screen(mut self) -> ConditionBuilder { self.0.push(Component::Screen); ConditionBuilder(self.0) }
	fn speech(mut self) -> ConditionBuilder { self.0.push(Component::Speech); ConditionBuilder(self.0) }
}

pub struct ConditionBuilder(Vec<Component>);
impl ConditionBuilder {
	pub fn not(mut self, mut x: ConditionBuilder) -> Self { self.0.append(&mut x.0); self }
	pub fn aspect_ratio(mut self, w: u32, h: u32) -> ConditionBuilderWithProp { self.0.push(Component::AspectRatio(w, h)); ConditionBuilderWithProp(self.0) }
}

pub struct ConditionBuilderNot(Vec<Component>);

pub struct ConditionBuilderWithProp(Vec<Component>);
impl ConditionBuilderWithProp {
	fn or(self) -> ConditionBuilder { todo!() }
	fn and(self) -> ConditionBuilder { todo!() }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Component {
	InitialNot,
	Not(Condition), And, Or,
	AspectRatio(u32, u32), MinAspectRatio(u32, u32), MaxAspectRatio(u32, u32),
	Color(u32), MinColor(u32), MaxColor(u32),
	Monochrome(u32), MinMonochrome(u32), MaxMonochrome(u32),
	Width(Unit), MinWidth(Unit), MaxWidth(Unit),
	Height(Unit), MinHeight(Unit), MaxHeight(Unit),
	Resolution(u32), MinResolution(u32), MaxResolution(u32),
	Orientation(Orientation), Scan(Scan),
	All, Print, Screen, Speech,
	Raw(String),
}

impl std::fmt::Display for Component {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Not(x) => write!(f, "(not {})", x),
			// Self::Only(x) => write!(f, "only ({})", x),
			// Self::And(x) => write!(f, "and ({})", x),
			Self::And => write!(f, "and"),
			// Self::Or(x) => write!(f, ", ({})", x),
			Self::Or => write!(f, ","),
			Self::AspectRatio(w, h) => write!(f, "(aspect-ratio: {}/{})", w, h),
			Self::MinAspectRatio(w, h) => write!(f, "(min-aspect-ratio: {}/{}", w, h),
			Self::MaxAspectRatio(w, h) => write!(f, "(max-aspect-ratio: {}/{}", w, h),
			Self::Raw(x) => x.fmt(f),
			_ => todo!(),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Condition(Vec<Component>);

impl std::fmt::Display for Condition {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for component in &self.0 {
			write!(f, "{} ", component)?;
		}
		Ok(())
	}
}
*/

/*
Condition(vec![
	Only(Screen),
	And(Condition(vec![MinWidth(100 px)])),
	Or
	// Or(Condition(
])
*/

// impl ToString for Media {
//     fn to_string(&self) -> String {
//         match self {
//             Self::MinWidth(unit) => format!("@media(min-width:{})", unit.to_string()),
//             Self::MaxWidth(unit) => format!("@media(max-width:{})", unit.to_string()),
//             Self::MinAspectRatio(width, height) => format!("@media(min-aspect-ratio:{}/{})", width, height),
//             Self::MaxAspectRatio(width, height) => format!("@media(max-aspect-ratio:{}/{})", width, height),
//         }
//     }
// }

// TODO: kind of hacky
// #[derive(Debug, PartialEq, Eq, Hash, Clone)]
// pub struct MediaMaxWidth(pub Unit);

// impl ToString for MediaMaxWidth {
//     fn to_string(&self) -> String {
//         format!("@media (max-width:{})", self.0.to_string())
//     }
// }



// combiners:
// * and
// * ,
// modifiers:
// * not
// * only
//
// * aspect-ratio(u32, u32)
//   * min-aspect-ratio
//   * max-aspect-ratio
// * color(u32)
//   * min-color
//   * max-color
// * monochrome(u32)
//   * min-monochrome
//   * max-monochrome
// * height(Unit)
//   * min-height
//   * max-height
// * width(Unit)
//   * min-width
//   * max-width
// * orientation(portrait | landscape)
// * resolution(u32 [dpi])
// * scan(interlace | progressive)
// * raw(String)
