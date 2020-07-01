use heck::*;
use itertools::Itertools;
use proc_quote::quote;
use syn::{
	parse::{Parse, ParseStream},
	punctuated::Punctuated,
	Result, Token,
	ext::IdentExt as _,
};

#[derive(Debug, Clone)]
struct HyphenatedName(String);

impl Parse for HyphenatedName {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self(<Punctuated<proc_macro2::TokenTree, Token![-]>>::parse_separated_nonempty(input)?.into_iter().map(|x| x.to_string()).join("-")))
	}
}

#[derive(Debug)]
struct Input {
	property: HyphenatedName,
	values: Vec<Value>,
}

impl Parse for Input {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut values = vec![
			Value::EnumVariant(HyphenatedName("initial".to_owned())),
			Value::EnumVariant(HyphenatedName("inherit".to_owned())),
			Value::EnumVariant(HyphenatedName("unset".to_owned())),
			Value::EnumVariant(HyphenatedName("revert".to_owned())),
		];
		let property = input.parse()?;

		while let Ok(value) = input.parse() {
			values.push(value);
		}

		Ok(Self { property, values })
	}
}

#[derive(Debug, Clone)]
enum Value {
	EnumVariant(HyphenatedName),
	Unit,
	String,
	Number,
	Float,
	Raw,
}

impl Parse for Value {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.parse::<Token![@]>().is_ok() {
			return Ok(Self::Unit);
		} else if input.parse::<Token![$]>().is_ok() {
			return Ok(Self::String);
		} else if input.parse::<Token![#]>().is_ok() {
			return Ok(Self::Number);
		} else if input.peek(syn::token::Bracket) {
			syn::custom_keyword!(float);
			syn::custom_keyword!(raw);

			let content;
			syn::bracketed!(content in input);
			if content.parse::<float>().is_ok() {
				return Ok(Self::Float);
			} else if content.parse::<raw>().is_ok() {
				return Ok(Self::Raw);
			}
		} else if let Ok(x) = input.parse::<HyphenatedName>() {
			return Ok(Self::EnumVariant(x));
		}

		Err(input.error("unexpected tokens"))
	}
}

#[proc_macro]
pub fn easy_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as Input);

	let property_snek = proc_macro2::Ident::new(&input.property.0.to_snek_case(), proc_macro2::Span::call_site());
	let property_camel = proc_macro2::Ident::new(&input.property.0.to_camel_case(), proc_macro2::Span::call_site());

	let enum_members = input.values.iter().map(|value| match value {
		Value::EnumVariant(value) => {
			let value_camel = proc_macro2::Ident::new(&value.0.to_camel_case(), proc_macro2::Span::call_site());
			quote! {#value_camel,}
		},
		Value::Unit => {
			quote! {
				Zero,
				Some(crate::units::Unit),
			}
		},
		Value::String => {
			quote! {String(String),}
		},
		Value::Raw => {
			quote! {Raw(String),}
		},
		Value::Number => {
			quote! {Number(i32),}
		},
		Value::Float => {
			quote! {Number(crate::units::F32),}
		},
	});

	let to_string_lines = input.values.iter().map(|value| match value {
		Value::EnumVariant(value) => {
			let value_camel = proc_macro2::Ident::new(&value.0.to_camel_case(), proc_macro2::Span::call_site());
			let css_string = format!("{}:{};", input.property.0, value.0);
			quote! {Self::#value_camel => #css_string.to_owned(),}
		},
		Value::Unit => {
			let css_format_string = format!("{}:{{}};", input.property.0);
			let css_zero_string = format!("{}:0;", input.property.0);
			quote! {
				Self::Some(x) => format!(#css_format_string, x.to_string()),
				Self::Zero => #css_zero_string.to_owned(),
			}
		},
		Value::String => {
			let css_format_string = format!(r#"{}:"{{}}";"#, input.property.0);
			quote! {Self::String(x) => format!(#css_format_string, x),}
		},
		Value::Raw => {
			let css_format_string = format!("{}:{{}};", input.property.0);
			quote! {Self::Raw(x) => format!(#css_format_string, x),}
		},
		Value::Number | Value::Float => {
			let css_format_string = format!("{}:{{}};", input.property.0);
			quote! {Self::Number(x) => format!(#css_format_string, x),}
		},
	});

	let macro_values = input.values.iter().map(|value| match value {
		Value::EnumVariant(value) => {
			let value_camel = proc_macro2::Ident::new(&value.0.to_camel_case(), proc_macro2::Span::call_site());
			let value_tt: proc_macro2::TokenStream = syn::parse_str(&value.0).unwrap();
			quote! {(#value_tt) => { $crate::Property::#property_camel($crate::#property_camel::#value_camel) };}
		},
		Value::Unit => {
			quote! {
				(0) => { $crate::Property::#property_camel($crate::#property_camel::Zero) };
				($($val:tt)+) => { $crate::Property::#property_camel($crate::#property_camel::Some($crate::unit!($($val)+))) };
			}
		},
		Value::String => {
			quote! {($str:expr) => { $crate::Property::#property_camel($crate::#property_camel::String($str.into())) };}
		},
		Value::Raw => {
			quote! {($str:expr) => { $crate::Property::#property_camel($crate::#property_camel::Raw($str.into())) };}
		},
		Value::Number => {
			quote! {($num:expr) => { $crate::Property::#property_camel($crate::#property_camel::Number($num)) };}
		},
		Value::Float => {
			quote! {($num:expr) => { $crate::Property::#property_camel($crate::#property_camel::Number(unsafe { $crate::units::F32::unchecked_new($num as _) })) };}
		},
	});

	let res = quote!(
		#[derive(Debug, PartialEq, Eq, Hash, Clone)]
		pub enum #property_camel {
			#(#enum_members)*
		}

		impl ToString for #property_camel {
			fn to_string(&self) -> String {
				match self {
					#(#to_string_lines)*
				}
			}
		}

		#[macro_export]
		macro_rules! #property_snek {
			#(#macro_values)*
		}
	);

	// println!("{}", res.to_string());

	res.into()
}

#[proc_macro]
pub fn easy_color(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let property = input.into_iter().map(|x| x.to_string()).collect::<String>();

	let property_snek = proc_macro2::Ident::new(&property.to_snek_case(), proc_macro2::Span::call_site());
	let property_camel = proc_macro2::Ident::new(&property.to_camel_case(), proc_macro2::Span::call_site());

	let res = quote!(
		#[macro_export]
		macro_rules! #property_snek {
			(initial)                 => {$crate::Property::#property_camel($crate::ColorValue::Initial)};
			(inherit)                 => {$crate::Property::#property_camel($crate::ColorValue::Inherit)};
			(unset)                   => {$crate::Property::#property_camel($crate::ColorValue::Unset)};
			(revert)                  => {$crate::Property::#property_camel($crate::ColorValue::Revert)};
			(...$tuple:expr)          => {$crate::Property::#property_camel($crate::ColorValue::Rgba($tuple.0, $tuple.1, $tuple.2, $tuple.3))};
			($r:tt $g:tt $b:tt $a:tt) => {$crate::Property::#property_camel($crate::ColorValue::Rgba($r, $g, $b, $a))};
			($r:tt $g:tt $b:tt)       => {$crate::Property::#property_camel($crate::ColorValue::Rgba($r, $g, $b, 255))};
			($rgb:expr)               => {$crate::Property::#property_camel($crate::ColorValue::Rgba($rgb, $rgb, $rgb, 255))};
		}
	);

	res.into()
}

struct SelectorElement(proc_macro2::TokenStream);

impl Parse for SelectorElement {
	fn parse(input: ParseStream) -> Result<Self> {
		syn::custom_punctuation!(ClassPlaceholder, .&);

		Ok(Self({
			if input.parse::<Token![+]>().is_ok() { quote! { .adjacent() } }
			else if input.parse::<Token![>>]>().is_ok() { quote! { .descendant() } }
			else if input.parse::<Token![>]>().is_ok() { quote! { .child() } }
			else if input.parse::<Token![,]>().is_ok() { quote! { .and() } }
			else if input.parse::<ClassPlaceholder>().is_ok() { quote! { .class_placeholder() } }
			else if input.parse::<Token![*]>().is_ok() { quote! { .any() } }
			// html/svg element like div/span/a/p/img
			else if let Ok(element) = input.parse::<syn::Ident>() { quote! { .element(crate::selector::Element::#element) } }
			else if input.parse::<Token![.]>().is_ok() {
				if input.peek(syn::token::Bracket) {
					// some element type
					let content = { let content; syn::bracketed!(content in input); content.parse::<syn::Type>().unwrap() };
					quote! { .class(<#content>::type_class_string()) }
				} else if input.peek(syn::token::Paren) {
					// class expr
					let content = { let content; syn::parenthesized!(content in input); content.parse::<syn::Expr>().unwrap() };
					quote! { .class(#content.into()) }
				} else {
					panic!("unknown token")
				}
			} else if input.peek(syn::token::Bracket) {
				// literal attribute
				let content = { let content; syn::bracketed!(content in input); content.parse::<syn::Ident>().unwrap() };
				quote! { .attribute(stringify!(#content)) }
			} else if input.peek(Token![#]) {
				// id expr
				let content = { let content; syn::parenthesized!(content in input); content.parse::<syn::Expr>().unwrap() };
				quote! { .id(#content.into()) }
			} else if input.is_empty() {
				return Err(syn::Error::new(proc_macro2::Span::call_site(), "finished"))
			} else {
				panic!("unknown token")
			}
		}))
	}
}

impl quote::ToTokens for SelectorElement {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		self.0.to_tokens(tokens)
	}
}

struct Selector(Vec<SelectorElement>);

impl Parse for Selector {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut selector = Vec::new();

		while let Ok(element) = input.parse::<SelectorElement>() {
			selector.push(element);
		}

		Ok(Self(selector))
	}
}

impl quote::ToTokens for Selector {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		self.0.iter().for_each(|x| x.to_tokens(tokens));
	}
}

// (@($acc:expr) + $($rest:tt)+)                                 => { $crate::selector!(@($acc.adjacent()) $($rest)+) };
// (@($acc:expr) >> $($rest:tt)+)                                => { $crate::selector!(@($acc.descendant()) $($rest)+) };
// (@($acc:expr) > $($rest:tt)+)                                 => { $crate::selector!(@($acc.child()) $($rest)+) };
// (@($acc:expr) , $($rest:tt)+)                                 => { $crate::selector!(@($acc.and()) $($rest)+) };
// (@($acc:expr) .& $($rest:tt)+)                                => { $crate::selector!(@($acc.class_placeholder()) $($rest)+) };
// (@($acc:expr) * $($rest:tt)+)                                 => { $crate::selector!(@($acc.any()) $($rest)+) };
// (@($acc:expr) $element:ident $($rest:tt)+)                    => { $crate::selector!(@($acc.element($crate::selector::Element::$element)) $($rest)+) };
//
// (@($acc:expr) .[$ty:ty] $($rest:tt)+)                         => { $crate::selector!(@($acc.class(<$ty>::type_class_string().into())) $($rest)+) };
// (@($acc:expr) .($class:expr) $($rest:tt)+)                    => { $crate::selector!(@($acc.class($class.into())) $($rest)+) };
// (@($acc:expr) [$($attr:tt)+] $($rest:tt)+)                    => { $crate::selector!(@($acc.attribute(stringify!($($attr)+).into())) $($rest)+) };
// (@($acc:expr) #($id:expr) $($rest:tt)+)                       => { $crate::selector!(@($acc.id($id.into())) $($rest)+) };
//
// (@($acc:expr) :nth_child($n:expr, $offset:expr) $($rest:tt)+) => { $crate::selector!(@($acc.pseudo_class($crate::selector::PseudoClass::nth_child($n, $offset))) $($rest)+) };
// (@($acc:expr) :nth_child($offset:expr) $($rest:tt)+)          => { $crate::selector!(@($acc.pseudo_class($crate::selector::PseudoClass::nth_child(0, $offset))) $($rest)+) };
// (@($acc:expr) :nth_of_type($n:expr) $($rest:tt)+)             => { $crate::selector!(@($acc.pseudo_class($crate::selector::PseudoClass::nth_of_type($n))) $($rest)+) };
// (@($acc:expr) :not($($selector:tt)+) $($rest:tt)+)            => { $crate::selector!(@($acc.pseudo_class($crate::selector::PseudoClass::not($crate::selector!($($selector)+)))) $($rest)+) };
// (@($acc:expr) :[$raw:expr] $($rest:tt)+)                      => { $crate::selector!(@($acc.pseudo_class($crate::selector::PseudoClass::raw($raw.into()))) $($rest)+) };
// (@($acc:expr) :$pseudo_class:ident $($rest:tt)+)              => { $crate::selector!(@($acc.pseudo_class($crate::selector::PseudoClass::$pseudo_class)) $($rest)+) };
// (@($acc:expr) ::$pseudo_element:ident $($rest:tt)+)           => { $crate::selector!(@($acc.pseudo_element($crate::selector::PseudoElement::$pseudo_element)) $($rest)+) };
#[proc_macro]
pub fn selector(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let selector = syn::parse_macro_input!(input as Selector);

	(quote! {crate::selector::Selector::build() #selector}).into()
}
