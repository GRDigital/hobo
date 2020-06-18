use derive_utils::quick_derive as enum_derive;
use proc_macro2::TokenStream;
use proc_quote::quote;
use quote::ToTokens;

#[proc_macro_derive(Element)]
pub fn derive_element(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as syn::DeriveInput);

	match &input.data {
		syn::Data::Enum(_) => enum_derive! {
			input.to_token_stream(),
			::hobo::Element,
			trait Element {
				fn element(&self) -> ::std::borrow::Cow<'_, ::hobo::web_sys::Element>;
			}
		},
		_ => {
			let name = input.ident;
			let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
			(quote! {
				impl #impl_generics ::hobo::Element for #name #ty_generics #where_clause {
					fn element(&self) -> ::std::borrow::Cow<'_, ::hobo::web_sys::Element> { self.element.element() }
				}
			}).into()
		},
	}
}

#[proc_macro_derive(EventTarget)]
pub fn derive_event_target(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as syn::DeriveInput);

	match &input.data {
		syn::Data::Enum(_) => enum_derive! {
			input.to_token_stream(),
			::hobo::EventTarget,
			trait EventTarget {
				fn event_handlers(&self) -> ::std::cell::RefMut<::std::vec::Vec<::hobo::EventHandler>>;
			}
		},
		_ => {
			let name = input.ident;
			let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
			(quote! {
				impl #impl_generics ::hobo::EventTarget for #name #ty_generics #where_clause {
					fn event_handlers(&self) -> ::std::cell::RefMut<::std::vec::Vec<::hobo::EventHandler>> { self.element.event_handlers() }
				}
			}).into()
		},
	}
}

#[proc_macro_derive(Container)]
pub fn derive_container(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as syn::DeriveInput);

	match &input.data {
		syn::Data::Enum(_) => enum_derive! {
			input.to_token_stream(),
			::hobo::Container,
			trait Container {
				fn children(&self) -> &::std::vec::Vec<Box<dyn ::hobo::Element>>;
				fn children_mut(&mut self) -> &mut ::std::vec::Vec<Box<dyn ::hobo::Element>>;
			}
		},
		_ => {
			let name = input.ident;
			let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
			(quote! {
				impl #impl_generics ::hobo::Container for #name #ty_generics #where_clause {
					fn children(&self) -> &::std::vec::Vec<Box<dyn ::hobo::Element>> { self.element.children() }
					fn children_mut(&mut self) -> &mut ::std::vec::Vec<Box<dyn ::hobo::Element>> { self.element.children_mut() }
				}
			}).into()
		},
	}
}

#[proc_macro_derive(Replaceable)]
pub fn derive_replaceable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as syn::DeriveInput);

	match &input.data {
		syn::Data::Enum(_) => enum_derive! {
			input.to_token_stream(),
			::hobo::Replaceable,
			trait Replaceable<T> {
				fn replace_element(&self, element: T);
			}
		},
		_ => {
			let name = input.ident;
			let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
			(quote! {
				impl<T: ::hobo::Element + 'static> ::hobo::Replaceable<T> for #name #ty_generics #where_clause {
					fn replace_element(&self, element: T) { self.element.replace_element(element) }
				}
			}).into()
		},
	}
}

#[proc_macro_derive(RawElement)]
pub fn derive_raw_element(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as syn::DeriveInput);
	let name = input.ident;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let element_type = match &input.data {
		syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(syn::FieldsNamed { named, .. }), .. }) => {
			let mut res = None;
			for field in named.iter() {
				if let Some(ident) = &field.ident {
					if ident == "element" {
						res = Some(&field.ty);
						break;
					}
				}
			}
			if let Some(x) = res { x } else { panic!("element not found") }
		},
		_ => unimplemented!(),
	};

	(quote! {
		impl #impl_generics ::hobo::RawElement for #name #ty_generics #where_clause {
			type RawElementType = <#element_type as ::hobo::RawElement>::RawElementType;
			fn raw_element(&self) -> &<#element_type as ::hobo::RawElement>::RawElementType { self.element.raw_element() }
		}
	}).into()
}
