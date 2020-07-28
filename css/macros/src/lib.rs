mod prelude;
mod media;
mod selector;
mod easy_enum;

use prelude::*;

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
#[proc_macro_hack]
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
	(quote! {
		#[macro_export]
		macro_rules! #macro_name {
			(initial)     => {$crate::Property::#property_name($crate::UnitValue::Initial)};
			(inherit)     => {$crate::Property::#property_name($crate::UnitValue::Inherit)};
			(unset)       => {$crate::Property::#property_name($crate::UnitValue::Unset)};
			(revert)      => {$crate::Property::#property_name($crate::UnitValue::Revert)};
			(0)           => {$crate::Property::#property_name($crate::UnitValue::Zero)};
			($($val:tt)+) => {$crate::Property::#property_name($crate::UnitValue::Unit($crate::unit!($($val)+)))};
		}
	}).into()
}

#[proc_macro_error]
#[proc_macro_hack]
pub fn media_query(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let crate_name = css_crate_name();
	let input: media::Query = syn::parse_macro_input!(input);
	(quote! {{
		use #crate_name::media::{MediaType::*, Orientation::*, Scan::*, Nottable, MediaFeature};
		#input
	}}).into()
}

#[proc_macro_error]
#[proc_macro_hack]
pub fn media_selector(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let crate_name = css_crate_name();
	let media::Selector(input) = syn::parse_macro_input!(input);
	(quote! {{
		use #crate_name::media::{MediaType::*, Orientation::*, Scan::*, Nottable, MediaFeature};
		MediaSelector(vec![#(#input),*])
	}}).into()
}
