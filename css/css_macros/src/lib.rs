extern crate proc_macro;
use proc_macro2::TokenStream;
use proc_quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, DataStruct};
use darling::{FromField, FromDeriveInput};
use heck::*;

#[proc_macro]
pub fn easy_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let mut args: Vec<String> = vec![];
	let mut current_ident: String = String::new();
	let mut input = input.into_iter();
	while let Some(t) = input.next() {
		match t {
			proc_macro::TokenTree::Punct(x) if x.as_char() == '-' => {
				current_ident += "-";
				// args[args.len() - 1] += "'";
			},
			proc_macro::TokenTree::Ident(x) => {
				if current_ident.is_empty() {
					current_ident = x.to_string();
				} else if current_ident.chars().last().unwrap() == '-' {
					current_ident += &x.to_string();
				} else {
					args.push(current_ident);
					current_ident = x.to_string();
				}
			},
			_ => {},
		}
	}
	args.push(current_ident);
	args.push("initial".to_owned());
	args.push("inherit".to_owned());

	let property: &str = args.get(0).unwrap();
	let values = args.get(1..).unwrap();

	let property_snek = proc_macro2::Ident::new(&property.to_snek_case(), proc_macro2::Span::call_site());
	let property_camel = proc_macro2::Ident::new(&property.to_camel_case(), proc_macro2::Span::call_site());

	let enum_members = values.iter().map(|value| {
		let value_camel = proc_macro2::Ident::new(&value.to_camel_case(), proc_macro2::Span::call_site());
		let css_string = format!("{}:{};", property, value);
		quote!{#[strum(to_string = #css_string)] #value_camel,}
	});
	let macro_values = values.iter().map(|value| {
		let value_camel = proc_macro2::Ident::new(&value.to_camel_case(), proc_macro2::Span::call_site());
		let value_tt: proc_macro2::TokenStream = syn::parse_str(value).unwrap();
		quote!{(#value_tt) => { $crate::Property::#property_camel($crate::#property_camel::#value_camel) };}
	});

	let will_ret = quote!(
		#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone, Copy, strum_macros::Display)]
		pub enum #property_camel {
			#[default]
			#(#enum_members)*
		}

		#[macro_export]
		macro_rules! #property_snek {
			#(#macro_values)*
		}
	);

	// println!("{}", will_ret.to_string());

	will_ret.into()
}
