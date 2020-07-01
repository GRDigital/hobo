#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, strum::Display)]
#[allow(non_camel_case_types)]
pub enum PseudoElement {
	#[strum(to_string = "::after")] after,
	#[strum(to_string = "::before")] before,
	#[strum(to_string = "::selection")] selection,
	#[strum(to_string = "::first-letter")] first_letter,
	#[strum(to_string = "::first-line")] first_line,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
#[allow(non_camel_case_types)]
pub enum PseudoClass {
	active,
	first_child,
	focus,
	hover,
	last_child,
	checked,
	disabled,
	enabled,
	in_range,
	invalid,
	nth_child(u32, u32),
	nth_of_type(u32),
	not(Selector),
	only_child,
	read_only,
	valid,
	raw(String),
	// etc
}

#[rustfmt::skip]
impl ToString for PseudoClass {
	fn to_string(&self) -> String {
		match self {
			Self::active               => ":active".to_owned(),
			Self::first_child          => ":first-child".to_owned(),
			Self::focus                => ":focus".to_owned(),
			Self::hover                => ":hover".to_owned(),
			Self::last_child           => ":last-child".to_owned(),
			Self::checked              => ":checked".to_owned(),
			Self::disabled             => ":disabled".to_owned(),
			Self::enabled              => ":enabled".to_owned(),
			Self::in_range             => ":in-range".to_owned(),
			Self::invalid              => ":invalid".to_owned(),
			Self::nth_child(n, offset) => format!(":nth-child({}n+{})", n, offset),
			Self::nth_of_type(n)       => format!(":nth-of-type({})", n),
			Self::not(selector)        => format!(":not({})", selector.to_string()),
			Self::only_child           => ":only-child".to_owned(),
			Self::read_only            => ":read-only".to_owned(),
			Self::valid                => ":valid".to_owned(),
			Self::raw(x)               => format!(":{}", x),
		}
	}
}

#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, strum::AsRefStr, strum::Display)]
#[allow(non_camel_case_types)]
pub enum Element {
	div, span, input, a, img, textarea,
	html, body, h1, h2, h3, h4, blockquote,
	h5, h6, p, header, var, nav, label,
	li, ul, ol, footer, strong, hr, button,
	svg, path, select, option, address,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum SelectorComponent {
	Element(Element),
	Class(String),
	Id(String),
	PseudoClass(PseudoClass),
	PseudoElement(PseudoElement),
	Child,
	Descendant,
	Adjacent,
	And,
	ClassPlaceholder,
	Any,
	FontFace,
	Attribute(String),
}

#[rustfmt::skip]
impl ToString for SelectorComponent {
	fn to_string(&self) -> String {
		match self {
			Self::Element(x)       => x.to_string(),
			Self::Class(x)         => format!(".{}", x),
			Self::Id(x)            => format!("#{}", x),
			Self::PseudoClass(x)   => x.to_string(),
			Self::PseudoElement(x) => x.to_string(),
			Self::Child            => ">".to_owned(),
			Self::Descendant       => " ".to_owned(),
			Self::Adjacent         => "+".to_owned(),
			Self::And              => ",".to_owned(),
			Self::ClassPlaceholder => ".&".to_owned(),
			Self::Any              => "*".to_owned(),
			Self::FontFace         => "@font-face".to_owned(),
			Self::Attribute(x)     => format!("[{}]", x),
		}
	}
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Selector(pub Vec<SelectorComponent>);

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct PseudoElementSelector(Vec<SelectorComponent>);

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct CombiningSelector(Vec<SelectorComponent>);

#[rustfmt::skip]
impl Selector {
	pub fn build()                                    -> CombiningSelector     { CombiningSelector::default() }
	pub fn class(mut self, x: String)                 -> Self                  { self.0.push(SelectorComponent::Class(x)); self }
	pub fn class_placeholder(mut self)                -> Self                  { self.0.push(SelectorComponent::ClassPlaceholder); self }
	pub fn id(mut self, x: String)                    -> Self                  { self.0.push(SelectorComponent::Id(x)); self }
	pub fn pseudo_class(mut self, x: PseudoClass)     -> Self                  { self.0.push(SelectorComponent::PseudoClass(x)); self }
	pub fn pseudo_element(mut self, x: PseudoElement) -> PseudoElementSelector { self.0.push(SelectorComponent::PseudoElement(x)); PseudoElementSelector(self.0) }
	pub fn child(mut self)                            -> CombiningSelector     { self.0.push(SelectorComponent::Child); CombiningSelector(self.0) }
	pub fn descendant(mut self)                       -> CombiningSelector     { self.0.push(SelectorComponent::Descendant); CombiningSelector(self.0) }
	pub fn adjacent(mut self)                         -> CombiningSelector     { self.0.push(SelectorComponent::Adjacent); CombiningSelector(self.0) }
	pub fn and(mut self)                              -> CombiningSelector     { self.0.push(SelectorComponent::And); CombiningSelector(self.0) }
	pub fn font_face()                                -> Self                  { Selector(vec![SelectorComponent::FontFace]) }
	pub fn attribute(mut self, x: String)             -> Self                  { self.0.push(SelectorComponent::Attribute(x)); self }
}

#[rustfmt::skip]
impl CombiningSelector {
	pub fn element(mut self, x: Element)              -> Selector              { self.0.push(SelectorComponent::Element(x)); Selector(self.0) }
	pub fn any(mut self)                              -> Selector              { self.0.push(SelectorComponent::Any); Selector(self.0) }
	pub fn class(mut self, x: String)                 -> Selector              { self.0.push(SelectorComponent::Class(x)); Selector(self.0) }
	pub fn class_placeholder(mut self)                -> Selector              { self.0.push(SelectorComponent::ClassPlaceholder); Selector(self.0) }
	pub fn id(mut self, x: String)                    -> Selector              { self.0.push(SelectorComponent::Id(x)); Selector(self.0) }
	pub fn pseudo_class(mut self, x: PseudoClass)     -> Selector              { self.0.push(SelectorComponent::PseudoClass(x)); Selector(self.0) }
	pub fn pseudo_element(mut self, x: PseudoElement) -> Selector              { self.0.push(SelectorComponent::PseudoElement(x)); Selector(self.0) }
	pub fn attribute(mut self, x: String)             -> Selector              { self.0.push(SelectorComponent::Attribute(x)); Selector(self.0) }
}

#[rustfmt::skip]
impl PseudoElementSelector {
	pub fn child(mut self)                            -> CombiningSelector     { self.0.push(SelectorComponent::Child); CombiningSelector(self.0) }
	pub fn descendant(mut self)                       -> CombiningSelector     { self.0.push(SelectorComponent::Descendant); CombiningSelector(self.0) }
	pub fn adjacent(mut self)                         -> CombiningSelector     { self.0.push(SelectorComponent::Adjacent); CombiningSelector(self.0) }
	pub fn and(mut self)                              -> CombiningSelector     { self.0.push(SelectorComponent::And); CombiningSelector(self.0) }
}

impl From<PseudoElementSelector> for Selector {
	fn from(x: PseudoElementSelector) -> Self { Self(x.0) }
}

impl ToString for PseudoElementSelector {
	fn to_string(&self) -> String { self.0.iter().map(ToString::to_string).collect::<String>() }
}

impl ToString for Selector {
	fn to_string(&self) -> String { self.0.iter().map(ToString::to_string).collect::<String>() }
}

/// ```edition2018,compile_fail
/// selector!(> div);
/// ```
/// ```edition2018,compile_fail
/// selector!(div div);
/// ```
#[test]
fn test_new_selector() {
	css_macros::selector!(
		.& >> div:nth_child(2, 1),
		.& >> *,
		div + * >> span > .&,
		div > .("raw class") [active] #("raw id") ::after,
		span :["raw pseudo_class"] > div :nth_child(0, 2) > span :nth_of_type(15) :hover,
		:not(div > span)
	);
}
