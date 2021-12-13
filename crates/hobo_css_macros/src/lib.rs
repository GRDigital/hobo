mod easy_enum;
mod media;
mod prelude;
mod selector;

use derive_utils::quick_derive as enum_derive;
use prelude::*;
use quote::ToTokens;

#[derive(Debug, Clone)]
struct HyphenatedName(String);

impl Parse for HyphenatedName {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self(<Punctuated<TokenTree, Token![-]>>::parse_separated_nonempty(input)?.into_iter().map(|x| x.to_string()).join("-")))
	}
}

#[proc_macro] pub fn easy_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream { easy_enum::easy_enum(input) }
#[proc_macro] pub fn easy_color(input: proc_macro::TokenStream) -> proc_macro::TokenStream { easy_enum::easy_color(input) }

#[proc_macro_error]
#[proc_macro]
pub fn selector(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let crate_name = css_crate_name();
	let selector = syn::parse_macro_input!(input as selector::Selector);

	// maybe move into selector's to tokens
	(quote! ({#crate_name::selector::Selector::from(#crate_name::selector::SelectorBuilder #selector)})).into()
}

struct UnitValueMacro {
	macro_name: syn::Ident,
	property_name: syn::Ident,
}

impl Parse for UnitValueMacro {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			macro_name: input.parse()?,
			property_name: input.parse()?,
		})
	}
}

#[proc_macro_error]
#[proc_macro]
pub fn unit_value_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let UnitValueMacro { macro_name, property_name } = syn::parse_macro_input!(input);

	let test_fn_name = quote::format_ident!("{}_initial_inherit_unset", macro_name);

	(quote! {
		#[macro_export]
		macro_rules! #macro_name {
			(initial)     => {$crate::Property::#property_name($crate::UnitValue::Initial)};
			(inherit)     => {$crate::Property::#property_name($crate::UnitValue::Inherit)};
			(unset)       => {$crate::Property::#property_name($crate::UnitValue::Unset)};
			($($val:tt)+) => {$crate::Property::#property_name($crate::UnitValue::Unit($crate::unit!($($val)+)))};
		}

		#[test]
		fn #test_fn_name() {
			assert_eq!(#macro_name!(initial), crate::Property::#property_name(crate::UnitValue::Initial));
			assert_eq!(#macro_name!(inherit), crate::Property::#property_name(crate::UnitValue::Inherit));
			assert_eq!(#macro_name!(unset), crate::Property::#property_name(crate::UnitValue::Unset));
		}
	}).into()
}

#[proc_macro_error]
#[proc_macro]
pub fn media_query(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let crate_name = css_crate_name();
	let input: media::Query = syn::parse_macro_input!(input);
	(quote! {{
		use #crate_name::media::{MediaQuery, MediaSelector, MediaType, Orientation::*, Scan::*, Nottable, MediaFeature};
		#input
	}}).into()
}

#[proc_macro_error]
#[proc_macro]
pub fn media_selector(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let crate_name = css_crate_name();
	let media::Selector(input) = syn::parse_macro_input!(input);
	(quote! {{
		use #crate_name::media::{MediaQuery, MediaSelector, MediaType, Orientation::*, Scan::*, Nottable, MediaFeature};
		MediaSelector(vec![#(#input),*])
	}}).into()
}

#[proc_macro_derive(AppendProperty)]
pub fn derive_append_property(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as syn::DeriveInput);

	match &input.data {
		syn::Data::Enum(_) => enum_derive! {
			input.to_token_stream(),
			::hobo::css::AppendProperty,
			trait AppendProperty {
				fn append_property(self, decls: &mut Vec<::hobo::css::Property>);
			}
		},
		_ => unimplemented!(),
	}
}
