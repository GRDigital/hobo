use crate::prelude::*;

#[derive(Clone)]
struct Nottable {
	not: bool,
	stream: TokenStream,
}

impl quote::ToTokens for Nottable {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let Self { not, stream } = self;
		(quote! { Nottable { not: #not, data: MediaFeature::#stream } }).to_tokens(tokens)
	}
}

impl Parse for Nottable {
	fn parse(input: ParseStream) -> Result<Self> {
		let not = input.parse::<Token![!]>().is_ok();
		let mut stream = TokenStream::new();
		while !input.peek(Token![&&]) && !input.is_empty() {
			stream.extend(std::iter::once(input.parse::<TokenTree>()?));
		}
		Ok(Self { not, stream })
	}
}

pub struct Query(Punctuated<Nottable, Token![&&]>);
impl Parse for Query {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self(Punctuated::parse_terminated_with(input, Nottable::parse)?))
	}
}

impl quote::ToTokens for Query {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let media = self.0[0].clone();
		let media_not = media.not;
		let media_ident = media.stream;
		let elems = self.0.iter().skip(1).collect::<Vec<_>>();
		(quote! {
			MediaQuery {
				media: Nottable { not: #media_not, data: MediaType::#media_ident },
				features: vec![#(#elems),*],
			}
		}).to_tokens(tokens)
	}
}

pub struct Selector(pub Vec<Query>);
impl Parse for Selector {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut punct = vec![];
		while !input.is_empty() {
			let mut stream = TokenStream::new();
			while input.parse::<Token![,]>().is_err() && !input.is_empty() {
				stream.extend(std::iter::once(input.parse::<TokenTree>()?));
			}
			punct.push(syn::parse_macro_input::parse::<Query>(stream.into())?);
		}
		Ok(Self(punct))
	}
}
