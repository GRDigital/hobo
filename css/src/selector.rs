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
	nth_child(i32, i32),
	nth_last_child(i32, i32),
	nth_of_type(u32),
	not(Selector),
	only_child,
	read_only,
	valid,
	// etc
}

#[rustfmt::skip]
impl std::fmt::Display for PseudoClass {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::active                    => ":active".fmt(f),
			Self::first_child               => ":first-child".fmt(f),
			Self::focus                     => ":focus".fmt(f),
			Self::hover                     => ":hover".fmt(f),
			Self::last_child                => ":last-child".fmt(f),
			Self::checked                   => ":checked".fmt(f),
			Self::disabled                  => ":disabled".fmt(f),
			Self::enabled                   => ":enabled".fmt(f),
			Self::in_range                  => ":in-range".fmt(f),
			Self::invalid                   => ":invalid".fmt(f),
			Self::nth_child(n, offset)      => write!(f, ":nth-child({}n{:+})", n, offset),
			Self::nth_last_child(n, offset) => write!(f, ":nth-last-child({}n{:+})", n, offset),
			Self::nth_of_type(n)            => write!(f, ":nth-of-type({})", n),
			Self::not(selector)             => write!(f, ":not({})", selector),
			Self::only_child                => ":only-child".fmt(f),
			Self::read_only                 => ":read-only".fmt(f),
			Self::valid                     => ":valid".fmt(f),
		}
	}
}

#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, strum::AsRefStr, strum::Display)]
#[allow(non_camel_case_types)]
pub enum Element {
	// svg
	svg,
	animate, animateMotion, animateTransform,
	circle, clipPath, defs, desc, ellipse,
	feBlend, feColorMatrix, feComponentTransfer, feComposite,
	feConvolveMatrix, feDiffuseLighting, feDisplacementMap, feDistantLight,
	feDropShadow, feFlood, feFuncA, feFuncB, feFuncG, feFuncR, feGaussianBlur,
	feImage, feMerge, feMergeNode, feMorphology, feOffset, fePointLight,
	feSpecularLighting, feSpotLight, feTile, feTurbulence,
	filter, foreignObject, gSvg, image, line, linearGradient, marker, mask,
	metadata, mpath, path, pattern, polygon, polyline, radialGradient,
	rect, set, stop, switch, symbol, text, textPath, tspan, r#use, view,

	// html
	html, base, head, link, meta,style, title,

	body,

	address, article, aside, footer, header,
	h1, h2, h3, h4, h5, h6,
	main, nav, section,

	blockquote,
	dd, div, dl, dt,
	figcaption, figure,
	hr, li, ol, p, pre, ul,

	a, abbr, b, bdi,
	bdo, br, cite,
	code, data, dfn,
	em, i, kbd, mark,
	q, rb, rp, rt, rtc,
	ruby, s, samp, small,
	span, strong, sub, sup,
	time, u, var, wbr,

	area, audio, img,
	map, track, video,

	embed, iframe, object,
	param, picture, source,

	canvas, noscript, script,

	del, ins,

	caption, col, colgroup, table,
	tbody, td, tfoot, th, thead, tr,

	button, datalist, fieldset, form,
	input, label, legend, meter,
	optgroup, option, output, progress,
	select, textarea,

	details, dialog, menu, summary,

	slot, template,
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
	Attribute(String),
	Raw(String),
}

#[rustfmt::skip]
impl std::fmt::Display for SelectorComponent {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Element(x)       => x.fmt(f),
			Self::Class(x)         => write!(f, ".{}", x),
			Self::Id(x)            => write!(f, "#{}", x),
			Self::PseudoClass(x)   => x.fmt(f),
			Self::PseudoElement(x) => x.fmt(f),
			Self::Child            => ">".fmt(f),
			Self::Descendant       => " ".fmt(f),
			Self::Adjacent         => "+".fmt(f),
			Self::And              => ",".fmt(f),
			Self::ClassPlaceholder => ".&".fmt(f),
			Self::Any              => "*".fmt(f),
			Self::Attribute(x)     => write!(f, "[{}]", x),
			Self::Raw(x)           => x.fmt(f),
		}
	}
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Selector(pub Vec<SelectorComponent>);

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct PseudoElementSelector(Vec<SelectorComponent>);

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct CombiningSelector(Vec<SelectorComponent>);

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct SelectorBuilder;

#[rustfmt::skip]
impl SelectorBuilder {
	pub fn element(self, x: Element)                  -> Selector              { Selector(vec![SelectorComponent::Element(x)]) }
	pub fn any(self)                                  -> Selector              { Selector(vec![SelectorComponent::Any]) }

	pub fn class(self, x: String)                     -> Selector              { Selector(vec![SelectorComponent::Class(x)]) }
	pub fn class_placeholder(self)                    -> Selector              { Selector(vec![SelectorComponent::ClassPlaceholder]) }
	pub fn id(self, x: String)                        -> Selector              { Selector(vec![SelectorComponent::Id(x)]) }
	pub fn pseudo_class(self, x: PseudoClass)         -> Selector              { Selector(vec![SelectorComponent::PseudoClass(x)]) }
	pub fn pseudo_element(self, x: PseudoElement)     -> Selector              { Selector(vec![SelectorComponent::PseudoElement(x)]) }
	pub fn attribute(self, x: String)                 -> Selector              { Selector(vec![SelectorComponent::Attribute(x)]) }

	pub fn raw(self, x: String)                       -> Selector              { Selector(vec![SelectorComponent::Raw(x)]) }
}

#[rustfmt::skip]
impl Selector {
	pub fn class(mut self, x: String)                 -> Self                  { self.0.push(SelectorComponent::Class(x)); self }
	pub fn class_placeholder(mut self)                -> Self                  { self.0.push(SelectorComponent::ClassPlaceholder); self }
	pub fn id(mut self, x: String)                    -> Self                  { self.0.push(SelectorComponent::Id(x)); self }
	pub fn pseudo_class(mut self, x: PseudoClass)     -> Self                  { self.0.push(SelectorComponent::PseudoClass(x)); self }
	pub fn pseudo_element(mut self, x: PseudoElement) -> PseudoElementSelector { self.0.push(SelectorComponent::PseudoElement(x)); PseudoElementSelector(self.0) }
	pub fn attribute(mut self, x: String)             -> Self                  { self.0.push(SelectorComponent::Attribute(x)); self }

	pub fn child(mut self)                            -> CombiningSelector     { self.0.push(SelectorComponent::Child); CombiningSelector(self.0) }
	pub fn descendant(mut self)                       -> CombiningSelector     { self.0.push(SelectorComponent::Descendant); CombiningSelector(self.0) }
	pub fn adjacent(mut self)                         -> CombiningSelector     { self.0.push(SelectorComponent::Adjacent); CombiningSelector(self.0) }
	pub fn and(mut self)                              -> CombiningSelector     { self.0.push(SelectorComponent::And); CombiningSelector(self.0) }

	pub fn raw(mut self, x: String)                   -> Self                  { self.0.push(SelectorComponent::Raw(x)); self }
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

	pub fn raw(mut self, x: String)                   -> Selector              { self.0.push(SelectorComponent::Raw(x)); Selector(self.0) }
}

#[rustfmt::skip]
impl PseudoElementSelector {
	pub fn child(mut self)                            -> CombiningSelector     { self.0.push(SelectorComponent::Child); CombiningSelector(self.0) }
	pub fn descendant(mut self)                       -> CombiningSelector     { self.0.push(SelectorComponent::Descendant); CombiningSelector(self.0) }
	pub fn adjacent(mut self)                         -> CombiningSelector     { self.0.push(SelectorComponent::Adjacent); CombiningSelector(self.0) }
	pub fn and(mut self)                              -> CombiningSelector     { self.0.push(SelectorComponent::And); CombiningSelector(self.0) }

	pub fn raw(mut self, x: String)                   -> Selector              { self.0.push(SelectorComponent::Raw(x)); Selector(self.0) }
}

impl From<PseudoElementSelector> for Selector {
	fn from(x: PseudoElementSelector) -> Self { Self(x.0) }
}

impl std::fmt::Display for PseudoElementSelector {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { for x in &self.0 { x.fmt(f)? } Ok(()) }
}

impl std::fmt::Display for Selector {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { for x in &self.0 { x.fmt(f)? } Ok(()) }
}

/*
/// ```edition2018,compile_fail
/// selector!(> div);
/// ```
/// ```edition2018,compile_fail
/// selector!(div div);
/// ```
#[test]
fn test_new_selector() {
	crate::css_macros::selector!(
		.& >> div:nth_child(2, 1),
		.& >> *,
		div + * >> span > .&,
		span raw("::raw_pseudo_class") > div :nth_child(0, 2) > span :nth_of_type(15) :hover,
		:not(div > span),
		div > .("raw class") [active] #("raw id") ::after
	);
}
*/
