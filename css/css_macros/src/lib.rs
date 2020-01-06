extern crate proc_macro;
use heck::*;
use proc_quote::quote;

enum Value {
	EnumVariant(String),
	Unit,
	String,
	Number,
}

#[proc_macro]
pub fn easy_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let mut args: Vec<String> = vec![];
	let mut current_ident: String = String::new();
	let input = input.into_iter();
	for t in input {
		match t {
			proc_macro::TokenTree::Punct(x) if x.as_char() == '-' => {
				current_ident += "-";
			},
			x => {
				if current_ident.is_empty() {
					current_ident = x.to_string();
				} else if current_ident.ends_with('-') {
					current_ident += &x.to_string();
				} else {
					args.push(current_ident);
					current_ident = x.to_string();
				}
			},
		}
	}
	args.push(current_ident);
	args.push("initial".to_owned());
	args.push("inherit".to_owned());

	let property: &str = args.get(0).unwrap();
	let values = args
		.get(1..)
		.unwrap()
		.iter()
		.map(|x| match &x as &str {
			"@" => Value::Unit,
			"$" => Value::String,
			"#" => Value::Number,
			x => Value::EnumVariant(x.to_owned()),
		})
		.collect::<Vec<_>>();

	let property_snek = proc_macro2::Ident::new(&property.to_snek_case(), proc_macro2::Span::call_site());
	let property_camel = proc_macro2::Ident::new(&property.to_camel_case(), proc_macro2::Span::call_site());

	let enum_members = values.iter().map(|value| match value {
		Value::EnumVariant(value) => {
			let value_camel = proc_macro2::Ident::new(&value.to_camel_case(), proc_macro2::Span::call_site());
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
		Value::Number => {
			quote! {Number(i32),}
		},
	});

	let to_string_lines = values.iter().map(|value| match value {
		Value::EnumVariant(value) => {
			let value_camel = proc_macro2::Ident::new(&value.to_camel_case(), proc_macro2::Span::call_site());
			let css_string = format!("{}:{};", property, value);
			quote! {Self::#value_camel => #css_string.to_owned(),}
		},
		Value::Unit => {
			let css_format_string = format!("{}:{{}};", property);
			let css_zero_string = format!("{}:0;", property);
			quote! {
				Self::Some(x) => format!(#css_format_string, x.to_string()),
				Self::Zero => #css_zero_string.to_owned(),
			}
		},
		Value::String => {
			let css_format_string = format!("{}:{{}};", property);
			quote! {Self::String(x) => format!(#css_format_string, x),}
		},
		Value::Number => {
			let css_format_string = format!("{}:{{}};", property);
			quote! {Self::Number(x) => format!(#css_format_string, x),}
		},
	});

	let macro_values = values.iter().map(|value| match value {
		Value::EnumVariant(value) => {
			let value_camel = proc_macro2::Ident::new(&value.to_camel_case(), proc_macro2::Span::call_site());
			let value_tt: proc_macro2::TokenStream = syn::parse_str(value).unwrap();
			quote! {(#value_tt) => { $crate::Property::#property_camel($crate::#property_camel::#value_camel) };}
		},
		Value::Unit => {
			quote! {
				($($val:tt)+) => { $crate::Property::#property_camel($crate::#property_camel::Some($crate::unit!($($val)+))) };
				(0) => { $crate::Property::#property_camel($crate::#property_camel::Zero) };
			}
		},
		Value::String => {
			quote! {($str:expr) => { $crate::Property::#property_camel($crate::#property_camel::String($str.into())) };}
		},
		Value::Number => {
			quote! {($str:expr) => { $crate::Property::#property_camel($crate::#property_camel::Number($str)) };}
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
			(initial)                 => {$crate::Property::#property_camel($crate::Color::Initial)};
			(inherit)                 => {$crate::Property::#property_camel($crate::Color::Inherit)};
			($r:tt $g:tt $b:tt $a:tt) => {$crate::Property::#property_camel($crate::Color::Rgba($r, $g, $b, $a))};
			($r:tt $g:tt $b:tt)       => {$crate::Property::#property_camel($crate::Color::Rgba($r, $g, $b, 255))};
			($rgb:tt)                 => {$crate::Property::#property_camel($crate::Color::Rgba($rgb, $rgb, $rgb, 255))};
		}
	);

	res.into()
}
