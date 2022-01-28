use derive_utils::quick_derive as enum_derive;
use proc_macro2::{Span, TokenStream};
use proc_quote::quote;
use quote::ToTokens;
// use proc_macro_error::proc_macro_error;

fn crate_name() -> TokenStream {
	match proc_macro_crate::crate_name("hobo") {
		Err(_) | Ok(proc_macro_crate::FoundCrate::Itself) => quote! { crate },
		Ok(proc_macro_crate::FoundCrate::Name(x)) => { let hobo = syn::Ident::new(&x, Span::call_site()); quote! { ::#hobo } },
	}
}

// #[proc_macro_error]
// #[proc_macro_attribute]
// pub fn trick(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let syn::ImplItemMethod { attrs, vis, defaultness, mut sig, block } = syn::parse_macro_input!(item as syn::ImplItemMethod);
//     sig.output = syn::parse_quote!(-> ::std::rc::Rc<::std::cell::RefCell<Self>>);
//     (quote! {
//         #(#attrs)* #vis #defaultness #sig {
//             let mut this: ::std::rc::Rc<::std::mem::MaybeUninit<::std::cell::RefCell<Self>>> = ::std::rc::Rc::new(::std::mem::MaybeUninit::uninit());
//             let new_this = #block;
//             unsafe {
//                 let raw_uninit = ::std::rc::Rc::into_raw(this) as *mut ::std::mem::MaybeUninit<_>;
//                 let raw_init = (&mut *raw_uninit).as_mut_ptr();
//                 ::std::ptr::write(raw_init, ::std::cell::RefCell::new(new_this));
//                 ::std::rc::Rc::from_raw(raw_init)
//             }
//         }
//     }).into()
// }

#[proc_macro_derive(AsEntity)]
pub fn derive_as_entity(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let crate_name = crate_name();
	let input = syn::parse_macro_input!(input as syn::DeriveInput);

	match &input.data {
		syn::Data::Enum(_) => enum_derive! {
			input.to_token_stream(),
			::hobo::entity::AsEntity,
			trait AsEntity {
				fn as_entity(&self) -> ::hobo::entity::Entity;
			}
		},
		syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(_), .. }) => {
			let name = input.ident;
			let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
			(quote! {
				impl #impl_generics #crate_name::entity::AsEntity for #name #ty_generics #where_clause {
					fn as_entity(&self) -> #crate_name::entity::Entity { self.element.as_entity() }
				}
			}).into()
		},
		syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Unnamed(_), .. }) => {
			let name = input.ident;
			let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
			(quote! {
				impl #impl_generics #crate_name::entity::AsEntity for #name #ty_generics #where_clause {
					fn as_entity(&self) -> #crate_name::entity::Entity { self.0.as_entity() }
				}
			}).into()
		},
		_ => unimplemented!(),
	}
}

#[proc_macro_derive(Element)]
pub fn derive_element(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let crate_name = crate_name();
	let as_entity: TokenStream = derive_as_entity(input.clone()).into();
	let input = syn::parse_macro_input!(input as syn::DeriveInput);
	let name = input.ident;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
	(quote! {
		#as_entity
		impl #impl_generics #crate_name::Element for #name #ty_generics #where_clause {}
	}).into()
}

// fn extract_element_type(data: &syn::Data) -> syn::Type {
//     match data {
//         syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(syn::FieldsNamed { named, .. }), .. }) => {
//             let mut res = None;
//             for field in named.iter() {
//                 if let Some(ident) = &field.ident {
//                     if ident == "element" {
//                         res = Some(&field.ty);
//                         break;
//                     }
//                 }
//             }
//             if let Some(x) = res { x.clone() } else { panic!("element not found") }
//         },
//         _ => unimplemented!(),
//     }
// }
