macro_rules! declare_basic_shortcuts {
	($($name:ident),+$(,)*) => {paste::item!{$(
		pub type $name = crate::BasicElement<web_sys::[<Html $name Element>]>;
	)+}};
}

declare_basic_shortcuts! {
	Div, Span, Option, Select, Input, Anchor, IFrame
}
