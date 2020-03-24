use proc_quote::quote;

#[proc_macro_derive(Slot)]
pub fn derive_element(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as syn::DeriveInput);
	let name = input.ident;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	proc_macro::TokenStream::from(quote! {
		impl #impl_generics ::hobo::Element for #name #ty_generics #where_clause {
			fn element(&self) -> ::std::borrow::Cow<'_, ::hobo::web_sys::Element> { self.slot.element() }
		}
	})
}

#[proc_macro_derive(Component)]
pub fn derive_component(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as syn::DeriveInput);
	let name = input.ident;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let impls = quote! {
		impl #impl_generics ::hobo::Element for #name #ty_generics #where_clause {
			fn element(&self) -> ::std::borrow::Cow<'_, ::hobo::web_sys::Element> { self.element.element() }
		}

		impl #impl_generics ::hobo::EventTarget for #name #ty_generics #where_clause {
			fn event_handlers(&self) -> ::std::cell::RefMut<::std::vec::Vec<::hobo::EventHandler>> { self.element.event_handlers() }
		}

		impl #impl_generics #name #ty_generics #where_clause {
			pub fn attach_child(&mut self, child: impl ::hobo::Element + 'static) { self.element.attach_child(child) }
		}
	};

	impls.into()
}
