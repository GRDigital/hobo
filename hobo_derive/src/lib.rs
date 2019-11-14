extern crate proc_macro;
use proc_macro2::TokenStream;
use proc_quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, DataStruct};
use darling::{FromField, FromDeriveInput};

#[proc_macro_derive(Element)]
pub fn derive_element(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as DeriveInput);
	let name = input.ident;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	proc_macro::TokenStream::from(quote! {
		impl #impl_generics Drop for #name #ty_generics #where_clause {
			fn drop(&mut self) {
				self.element().remove();
			}
		}

		impl #impl_generics ::hobo::Element for #name #ty_generics #where_clause {
			fn element(&self) -> &web_sys::Element { &self.element }
		}
	})
}

#[proc_macro_derive(EventTarget)]
pub fn derive_event_target(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as DeriveInput);
	let name = input.ident;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	proc_macro::TokenStream::from(quote! {
		impl #impl_generics ::hobo::EventTarget for #name #ty_generics #where_clause {
			fn event_handlers(&self) -> ::std::cell::RefMut<Vec<::hobo::EventHandler>> { self.event_handlers.borrow_mut() }
		}
	})
}
