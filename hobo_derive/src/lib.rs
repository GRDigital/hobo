use proc_quote::quote;

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

		impl #impl_generics ::hobo::Container for #name #ty_generics #where_clause {
			fn children(&self) -> &::std::vec::Vec<Box<dyn ::hobo::Element>> { self.element.children() }
			fn children_mut(&mut self) -> &mut ::std::vec::Vec<Box<dyn ::hobo::Element>> { self.element.children_mut() }
		}
	};

	impls.into()
}

#[proc_macro_derive(Slot)]
pub fn derive_slot(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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

		impl #impl_generics ::hobo::Container for #name #ty_generics #where_clause {
			fn children(&self) -> &::std::vec::Vec<Box<dyn ::hobo::Element>> { self.element.children() }
			fn children_mut(&mut self) -> &mut ::std::vec::Vec<Box<dyn ::hobo::Element>> { self.element.children_mut() }
		}

		impl<T: ::hobo::Basic + 'static> ::hobo::Replaceable<T> for #name #ty_generics #where_clause {
			fn replace_element(&self, element: T) { self.element.replace_element(element) }
		}
	};

	impls.into()
}
