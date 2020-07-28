use crate::prelude::*;
use crate::HyphenatedName;

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
		if input.peek(syn::token::Bracket) {
			syn::custom_keyword!(unit);
			syn::custom_keyword!(string);
			syn::custom_keyword!(number);
			syn::custom_keyword!(float);
			syn::custom_keyword!(raw);

			let content;
			syn::bracketed!(content in input);
			if content.parse::<unit>().is_ok() {
				return Ok(Self::Unit);
			} else if content.parse::<string>().is_ok() {
				return Ok(Self::String);
			} else if content.parse::<number>().is_ok() {
				return Ok(Self::Number);
			} else if content.parse::<float>().is_ok() {
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

pub fn easy_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as Input);

	let property_snek = proc_macro2::Ident::new(&input.property.0.to_snek_case(), Span::call_site());
	let property_camel = proc_macro2::Ident::new(&input.property.0.to_camel_case(), Span::call_site());

	let enum_members = input.values.iter().map(|value| match value {
		Value::EnumVariant(value) => {
			let value_camel = proc_macro2::Ident::new(&value.0.to_camel_case(), Span::call_site());
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

	let display_lines = input.values.iter().map(|value| match value {
		Value::EnumVariant(value) => {
			let value_camel = proc_macro2::Ident::new(&value.0.to_camel_case(), Span::call_site());
			let css_string = format!("{}:{};", input.property.0, value.0);
			quote! {Self::#value_camel => write!(f, #css_string),}
		},
		Value::Unit => {
			let css_format_string = format!("{}:{{}};", input.property.0);
			let css_zero_string = format!("{}:0;", input.property.0);
			quote! {
				Self::Some(x) => write!(f, #css_format_string, x),
				Self::Zero => write!(f, #css_zero_string),
			}
		},
		Value::String => {
			let css_format_string = format!(r#"{}:"{{}}";"#, input.property.0);
			quote! {Self::String(x) => write!(f, #css_format_string, x),}
		},
		Value::Raw => {
			let css_format_string = format!("{}:{{}};", input.property.0);
			quote! {Self::Raw(x) => write!(f, #css_format_string, x),}
		},
		Value::Number | Value::Float => {
			let css_format_string = format!("{}:{{}};", input.property.0);
			quote! {Self::Number(x) => write!(f, #css_format_string, x),}
		},
	});

	let macro_values = input.values.iter().map(|value| match value {
		Value::EnumVariant(value) => {
			let value_camel = proc_macro2::Ident::new(&value.0.to_camel_case(), Span::call_site());
			let value_tt: TokenStream = syn::parse_str(&value.0).unwrap();
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

		impl ::std::fmt::Display for #property_camel {
			fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
				match self {
					#(#display_lines)*
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

pub fn easy_color(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let property = input.into_iter().map(|x| x.to_string()).collect::<String>();

	let property_snek = proc_macro2::Ident::new(&property.to_snek_case(), Span::call_site());
	let property_camel = proc_macro2::Ident::new(&property.to_camel_case(), Span::call_site());

	let res = quote!(
		#[macro_export]
		macro_rules! #property_snek {
			(initial)       => {$crate::Property::#property_camel($crate::ColorValue::Initial)};
			(inherit)       => {$crate::Property::#property_camel($crate::ColorValue::Inherit)};
			(unset)         => {$crate::Property::#property_camel($crate::ColorValue::Unset)};
			(revert)        => {$crate::Property::#property_camel($crate::ColorValue::Revert)};
			(gray $c:expr)  => {$crate::Property::#property_camel($crate::ColorValue::Rgba($crate::Color { r: $c, g: $c, b: $c, a: 0xFF }))};
			(rgb $rgb:expr) => {$crate::Property::#property_camel($crate::ColorValue::Rgba(($rgb << 8 | 0xFF).into()))};
			($rgba:expr)    => {$crate::Property::#property_camel($crate::ColorValue::Rgba($rgba.into()))};
		}
	);

	res.into()
}
