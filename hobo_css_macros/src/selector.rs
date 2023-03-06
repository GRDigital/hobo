use crate::prelude::*;

pub struct Selector(Vec<TokenStream>);

// TODO: I could factor out the literal quoted code if I return a Vec of some enum rather than just TokenStream
// and then each enum variant can have their own arguments etc
// in this way, parsing the selector and what code it outputs would be more cleanly seaprated
impl Parse for Selector {
	fn parse(input: ParseStream) -> Result<Self> {
		let crate_name = crate::css_crate_name();
		let mut selector = Vec::new();

		while !input.is_empty() {
			let element = {
				syn::custom_keyword!(raw);

				if input.parse::<Token![+]>().is_ok() { quote! { .adjacent() } }
				else if input.parse::<Token![>>]>().is_ok() { quote! { .descendant() } }
				else if input.parse::<Token![>]>().is_ok() { quote! { .child() } }
				else if input.parse::<Token![,]>().is_ok() { quote! { .and() } }
				else if input.parse::<Token![*]>().is_ok() { quote! { .any() } }
				else if input.parse::<raw>().is_ok() {
					let content = { let content; syn::parenthesized!(content in input); content.parse::<syn::Expr>()? };
					quote! { .raw(#content.into()) }
				} else if let Ok(element) = input.parse::<syn::Ident>() {
					// html/svg element like div/span/a/p/img
					quote! { .element(#crate_name::selector::Element::#element) }
				} else if input.parse::<Token![.]>().is_ok() {
					if input.peek(syn::token::Bracket) {
						// some element type
						let content = { let content; syn::bracketed!(content in input); content.parse::<syn::Type>()? };
						quote! { .class(<#content>::mark_class_string()) }
					} else if input.peek(syn::token::Paren) {
						// class expr
						let content = { let content; syn::parenthesized!(content in input); content.parse::<syn::Expr>()? };
						quote! { .class(#content.to_class_str()) }
					} else if input.parse::<Token![&]>().is_ok() {
						quote! { .class_placeholder() }
					} else {
						abort!(input.parse::<TokenTree>().unwrap(), "unknown token for a class")
					}
				} else if input.peek(syn::token::Bracket) {
					let content; syn::bracketed!(content in input);
					let maybe_ident = content.fork();
					if maybe_ident.parse::<syn::Ident>().is_ok() && maybe_ident.is_empty() {
						// literal attribute
						let content = content.parse::<syn::Ident>()?;
						let content_str = content.to_string();
						quote! { .attribute(#content_str.into()) }
					} else {
						// attribute expr
						let content = content.parse::<syn::Expr>()?;
						quote! { .attribute(#content.into()) }
					}
				} else if input.parse::<Token![#]>().is_ok() {
					// id expr
					let content = { let content; syn::parenthesized!(content in input); content.parse::<syn::Expr>()? };
					quote! { .id(#content.into()) }
				} else if input.parse::<Token![::]>().is_ok() {
					// pseudo element stuff
					let pseudo_element = input.parse::<syn::Ident>()?;
					quote! { .pseudo_element(#crate_name::selector::PseudoElement::#pseudo_element) }
				} else if input.parse::<Token![:]>().is_ok() {
					// pseudo class stuff
					syn::custom_keyword!(not);

					if input.parse::<not>().is_ok() {
						let content = { let content; syn::parenthesized!(content in input); content.parse::<Selector>()? };
						quote! { .pseudo_class(#crate_name::selector::PseudoClass::not(#crate_name::selector::SelectorBuilder #content)) }
					} else if let Ok(pseudo_class) = input.parse::<syn::Ident>() {
						if input.peek(syn::token::Paren) {
							let content = { let content; syn::parenthesized!(content in input); content.parse::<TokenStream>()? };
							quote! { .pseudo_class(#crate_name::selector::PseudoClass::#pseudo_class(#content)) }
						} else {
							quote! { .pseudo_class(#crate_name::selector::PseudoClass::#pseudo_class) }
						}
					} else {
						abort!(input.parse::<TokenTree>().unwrap(), "unknown token for a pseudo_class")
					}
				} else {
					abort!(input.parse::<TokenTree>().unwrap(), "unknown token")
				}
			};
			selector.push(element);
		}

		Ok(Self(selector))
	}
}

impl quote::ToTokens for Selector {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.0.iter().for_each(|x| x.to_tokens(tokens));
	}
}
