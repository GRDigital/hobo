#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, strum::Display)]
#[allow(non_camel_case_types)]
pub enum PseudoElement {
	#[strum(to_string = "::after")] after,
	#[strum(to_string = "::before")] before,
	#[strum(to_string = "::selection")] selection,
	// etc
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
	nth_child(u32),
	nth_of_type(u32),
	not(Selector),
	only_child,
	read_only,
	valid,
	// etc
}

#[rustfmt::skip]
impl ToString for PseudoClass {
	fn to_string(&self) -> String {
		match self {
			Self::active         => ":active".to_owned(),
			Self::first_child    => ":first-child".to_owned(),
			Self::focus          => ":focus".to_owned(),
			Self::hover          => ":hover".to_owned(),
			Self::last_child     => ":last-child".to_owned(),
			Self::checked        => ":checked".to_owned(),
			Self::disabled       => ":disabled".to_owned(),
			Self::enabled        => ":enabled".to_owned(),
			Self::in_range       => ":in-range".to_owned(),
			Self::invalid        => ":invalid".to_owned(),
			Self::nth_child(n)   => format!(":nth-child({})", n),
			Self::nth_of_type(n) => format!(":nth-of-type({})", n),
			Self::not(selector)  => format!(":not({}", selector.to_string()),
			Self::only_child     => ":only-child".to_owned(),
			Self::read_only      => ":read-only".to_owned(),
			Self::valid          => ":valid".to_owned(),
		}
	}
}

#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, strum::AsRefStr, strum::Display)]
#[allow(non_camel_case_types)]
pub enum Element {
	div, span, input, a, img, textarea,
	html, body, h1, h2, h3, h4, blockquote,
	h5, h6, p, header, var, nav,
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
#[rustfmt::skip]
#[macro_export]
macro_rules! selector {
	// finish
	(@($acc:expr) $element:ident)                       => { $acc.element($crate::selector::Element::$element) };
	(@($acc:expr) .($class:expr))                       => { $acc.class($class.into()) };
	(@($acc:expr) [$ty:ty])                             => { $acc.class(<$ty>::class().into()) };
	(@($acc:expr) .&)                                   => { $acc.class_placeholder() };
	(@($acc:expr) #($id:expr))                          => { $acc.id($id.into()) };
	(@($acc:expr) :last_child)                          => { $acc.pseudo_class($crate::selector::PseudoClass::last_child) };
	(@($acc:expr) :focus)                               => { $acc.pseudo_class($crate::selector::PseudoClass::focus) };
	(@($acc:expr) :nth_child($n:expr))                  => { $acc.pseudo_class($crate::selector::PseudoClass::nth_child($n)) };
	(@($acc:expr) :nth_of_type($n:expr))                => { $acc.pseudo_class($crate::selector::PseudoClass::nth_of_type($n)) };
	(@($acc:expr) :not($($selector:tt)+))               => { $acc.pseudo_class($crate::selector::PseudoClass::not($crate::selector!($($selector)+))) };
	(@($acc:expr) :$pseudo_class:ident)                 => { $acc.pseudo_class($crate::selector::PseudoClass::$pseudo_class) };
	(@($acc:expr) ::$pseudo_element:ident)              => { $crate::selector::Selector::from($acc.pseudo_element($crate::selector::PseudoElement::$pseudo_element)) };
	(@($acc:expr) *)                                    => { $acc.any() };

	// middle
	(@($acc:expr) + $($rest:tt)+)                       => { $crate::selector!(@($acc.adjacent()) $($rest)+) };
	(@($acc:expr) >> $($rest:tt)+)                      => { $crate::selector!(@($acc.descendant()) $($rest)+) };
	(@($acc:expr) > $($rest:tt)+)                       => { $crate::selector!(@($acc.child()) $($rest)+) };
	(@($acc:expr) , $($rest:tt)+)                       => { $crate::selector!(@($acc.and()) $($rest)+) };
	(@($acc:expr) $element:ident $($rest:tt)+)          => { $crate::selector!(@($acc.element($crate::selector::Element::$element)) $($rest)+) };
	(@($acc:expr) .($class:expr) $($rest:tt)+)          => { $crate::selector!(@($acc.class($class.into())) $($rest)+) };
	(@($acc:expr) [$ty:ty] $($rest:tt)+)                => { $crate::selector!(@($acc.class(<$ty>::class().into())) $($rest)+) };
	(@($acc:expr) .& $($rest:tt)+)                      => { $crate::selector!(@($acc.class_placeholder()) $($rest)+) };
	(@($acc:expr) #($id:expr) $($rest:tt)+)             => { $crate::selector!(@($acc.id($id.into())) $($rest)+) };
	(@($acc:expr) :last_child $($rest:tt)+)             => { $crate::selector!(@($acc.pseudo_class($crate::selector::PseudoClass::last_child)) $($rest)+) };
	(@($acc:expr) :focus $($rest:tt)+)                  => { $crate::selector!(@($acc.pseudo_class($crate::selector::PseudoClass::focus)) $($rest)+) };
	(@($acc:expr) :nth_child($n:expr) $($rest:tt)+)     => { $crate::selector!(@($acc.pseudo_class($crate::selector::PseudoClass::nth_child($n))) $($rest)+) };
	(@($acc:expr) :nth_of_type($n:expr) $($rest:tt)+)   => { $crate::selector!(@($acc.pseudo_class($crate::selector::PseudoClass::nth_of_type($n))) $($rest)+) };
	(@($acc:expr) :not($($selector:tt)+) $($rest:tt)+)  => { $crate::selector!(@($acc.pseudo_class($crate::selector::PseudoClass::not($crate::selector!($($selector)+)))) $($rest)+) };
	(@($acc:expr) :$pseudo_class:ident $($rest:tt)+)    => { $crate::selector!(@($acc.pseudo_class($crate::selector::PseudoClass::$pseudo_class)) $($rest)+) };
	(@($acc:expr) ::$pseudo_element:ident $($rest:tt)+) => { $crate::selector!(@($acc.pseudo_element($crate::selector::PseudoElement::$pseudo_element)) $($rest)+) };
	(@($acc:expr) * $($rest:tt)+)                       => { $crate::selector!(@($acc.any()) $($rest)+) };

	// start
	($element:ident $($rest:tt)+)                       => { $crate::selector!(@($crate::selector::Selector::build().element($crate::selector::Element::$element)) $($rest)+) };
	(.($class:expr) $($rest:tt)+)                       => { $crate::selector!(@($crate::selector::Selector::build().class($class.into())) $($rest)+) };
	([$ty:ty] $($rest:tt)+)                             => { $crate::selector!(@($crate::selector::Selector::build().class(<$ty>::class().into())) $($rest)+) };
	(.& $($rest:tt)+)                                   => { $crate::selector!(@($crate::selector::Selector::build().class_placeholder()) $($rest)+) };
	(#($id:expr) $($rest:tt)+)                          => { $crate::selector!(@($crate::selector::Selector::build().id($id.into())) $($rest)+) };
	(* $($rest:tt)+)                                    => { $crate::selector!(@($crate::selector::Selector::build().any()) $($rest)+) };

	// only
	(@font-face)                                        => { $crate::selector::Selector::font_face() };
	($elem:ident)                                       => { $crate::selector::Selector::build().element($crate::selector::Element::$elem) };
	(.($class:expr))                                    => { $crate::selector::Selector::build().class($class.into()) };
	([$ty:ty])                                          => { $crate::selector::Selector::build().class(<$ty>::class().into()) };
	(.&)                                                => { $crate::selector::Selector::build().class_placeholder() };
	(#($id:expr))                                       => { $crate::selector::Selector::build().id($id.into()) };
	(:last_child)                                       => { $crate::selector::Selector::build().pseudo_class($crate::selector::PseudoClass::last_child) };
	(:focus)                                            => { $crate::selector::Selector::build().pseudo_class($crate::selector::PseudoClass::focus) };
	(:nth_child($n:expr))                               => { $crate::selector::Selector::build().pseudo_class($crate::selector::PseudoClass::nth_child($n)) };
	(:nth_of_type($n:expr))                             => { $crate::selector::Selector::build().pseudo_class($crate::selector::PseudoClass::nth_of_type($n)) };
	(:not($($selector:tt)+))                            => { $crate::selector::Selector::build().pseudo_class($crate::selector::PseudoClass::not($crate::selector!($($selector)+))) };
	(:$pseudo_class:ident)                              => { $crate::selector::Selector::build().pseudo_class($crate::selector::PseudoClass::$pseudo_class) };
	(::$pseudo_element:ident)                           => { $crate::selector::Selector::build().pseudo_element($crate::selector::PseudoElement::$pseudo_element) };
	(*)                                                 => { $crate::selector::Selector::build().any() };
}
