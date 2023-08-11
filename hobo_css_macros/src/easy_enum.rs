use crate::{prelude::*, HyphenatedName};

#[derive(Debug)]
struct Input {
	property: HyphenatedName,
	values: Vec<Value>,
	prefixed: bool,
}

impl Parse for Input {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut values = vec![
			Value::EnumVariant(HyphenatedName("initial".to_owned())),
			Value::EnumVariant(HyphenatedName("inherit".to_owned())),
			Value::EnumVariant(HyphenatedName("unset".to_owned())),
		];
		let prefixed = input.parse::<syn::Token!(-)>().is_ok() && input.parse::<syn::Token!(*)>().is_ok() && input.parse::<syn::Token!(-)>().is_ok();
		let property = input.parse()?;

		while let Ok(value) = input.parse() {
			values.push(value);
		}

		Ok(Self { property, values, prefixed })
	}
}

#[derive(Debug, Clone)]
enum Value {
	EnumVariant(HyphenatedName),
	Unit,
	String,
	Number,
	Float,
	Raw,
}

impl Parse for Value {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.peek(syn::token::Bracket) {
			syn::custom_keyword!(unit);
			syn::custom_keyword!(string);
			syn::custom_keyword!(number);
			syn::custom_keyword!(float);
			syn::custom_keyword!(raw);

			let content;
			syn::bracketed!(content in input);
			if content.parse::<unit>().is_ok() {
				return Ok(Self::Unit);
			} else if content.parse::<string>().is_ok() {
				return Ok(Self::String);
			} else if content.parse::<number>().is_ok() {
				return Ok(Self::Number);
			} else if content.parse::<float>().is_ok() {
				return Ok(Self::Float);
			} else if content.parse::<raw>().is_ok() {
				return Ok(Self::Raw);
			}
		} else if let Ok(x) = input.parse::<HyphenatedName>() {
			return Ok(Self::EnumVariant(x));
		}

		Err(input.error("unexpected tokens"))
	}
}

pub fn easy_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as Input);

	let property_snek = proc_macro2::Ident::new(&input.property.0.to_snek_case(), Span::call_site());
	let property_camel = proc_macro2::Ident::new(&input.property.0.to_upper_camel_case(), Span::call_site());

	let test_fn_name = quote::format_ident!("{}_initial_inherit_unset", property_snek);
	let result_initial = format!("{}:initial;", input.property.0);
	let result_inherit = format!("{}:inherit;", input.property.0);
	let result_unset = format!("{}:unset;", input.property.0);

	let enum_members = input.values.iter().map(|value| match value {
		Value::EnumVariant(value) => {
			let value_camel = proc_macro2::Ident::new(&value.0.to_upper_camel_case(), Span::call_site());
			quote! {#value_camel,}
		},
		Value::Unit => quote! {Some(crate::units::Unit),},
		Value::String => quote! {String(String),},
		Value::Raw => quote! {Raw(String),},
		Value::Number => quote! {Number(i32),},
		Value::Float => quote! {Number(crate::units::F32),},
	});

	let display_lines = input.values.iter().map(|value| {
		let prop_name = &input.property.0;
		match value {
			Value::EnumVariant(value) => {
				let value_camel = proc_macro2::Ident::new(&value.0.to_upper_camel_case(), Span::call_site());
				let prop_value = &value.0;
				let css_string = if input.prefixed { format!("{prop_name}:{prop_value};-webkit-{prop_name}:{prop_value};-moz-{prop_name}:{prop_value};") } else { format!("{prop_name}:{prop_value};") };
				quote! {Self::#value_camel => write!(f, #css_string),}
			},
			formatted => {
				let css_format_string = if input.prefixed { format!("{prop_name}:{{x}};-webkit-{prop_name}:{{x}};-moz-{prop_name}:{{x}};") } else { format!("{prop_name}:{{x}};") };
				match formatted {
					Value::Unit   => quote! {Self::Some(x)   => write!(f, #css_format_string),},
					Value::String => quote! {Self::String(x) => write!(f, #css_format_string),},
					Value::Raw    => quote! {Self::Raw(x)    => write!(f, #css_format_string),},
					Value::Number |
					Value::Float  => quote! {Self::Number(x) => write!(f, #css_format_string),},
					_ => unreachable!(),
				}
			}
		}
	});

	let macro_values = input.values.iter().map(|value| match value {
		Value::EnumVariant(value) => {
			let value_camel = proc_macro2::Ident::new(&value.0.to_upper_camel_case(), Span::call_site());
			let value_tt: TokenStream = syn::parse_str(&value.0).unwrap();
			quote! {(#value_tt) => { $crate::Property::#property_camel($crate::#property_camel::#value_camel) };}
		},
		Value::Unit => quote! {($($val:tt)+) => { $crate::Property::#property_camel($crate::#property_camel::Some($crate::unit!($($val)+))) };},
		Value::String => quote! {($str:expr) => { $crate::Property::#property_camel($crate::#property_camel::String($str.into())) };},
		Value::Raw => quote! {($str:expr) => { $crate::Property::#property_camel($crate::#property_camel::Raw($str.into())) };},
		Value::Number => quote! {($num:expr) => { $crate::Property::#property_camel($crate::#property_camel::Number($num)) };},
		Value::Float => quote! {($num:expr) => { $crate::Property::#property_camel($crate::#property_camel::Number(unsafe { $crate::units::F32::new_unchecked($num as _) })) };},
	});

	// css::display::flex

	let fn_values = input.values.iter().map(|value| match value {
		Value::EnumVariant(value) => {
			let value_camel = proc_macro2::Ident::new(&value.0.to_upper_camel_case(), Span::call_site());
			let value_snek = proc_macro2::Ident::new_raw(match &value.0.to_snek_case() as &str {
				"super" => "super_",
				"crate" => "crate_",
				"self" => "self_",
				"Self" => "Self_",
				x => x,
			}, Span::call_site());
			quote! {pub use Property::#value_camel as #value_snek;}
		},
		Value::Unit => quote! {
			#[inline] pub fn zero() -> Property { Property::Some(crate::Unit::Zero) }
			#[inline] pub fn px(  x: impl ::num_traits::cast::AsPrimitive<f32>) -> Property { Property::Some(crate::Unit::px(x)) }
			#[inline] pub fn pct( x: impl ::num_traits::cast::AsPrimitive<f32>) -> Property { Property::Some(crate::Unit::pct(x)) }
			#[inline] pub fn em(  x: impl ::num_traits::cast::AsPrimitive<f32>) -> Property { Property::Some(crate::Unit::em(x)) }
			#[inline] pub fn rem( x: impl ::num_traits::cast::AsPrimitive<f32>) -> Property { Property::Some(crate::Unit::rem(x)) }
			#[inline] pub fn vw(  x: impl ::num_traits::cast::AsPrimitive<f32>) -> Property { Property::Some(crate::Unit::vw(x)) }
			#[inline] pub fn vh(  x: impl ::num_traits::cast::AsPrimitive<f32>) -> Property { Property::Some(crate::Unit::vh(x)) }
			#[inline] pub fn vmin(x: impl ::num_traits::cast::AsPrimitive<f32>) -> Property { Property::Some(crate::Unit::vmin(x)) }
			#[inline] pub fn vmax(x: impl ::num_traits::cast::AsPrimitive<f32>) -> Property { Property::Some(crate::Unit::vmax(x)) }

			// this should probably be separate from Unit
			#[inline] pub fn fr(  x: impl ::num_traits::cast::AsPrimitive<f32>) -> Property { Property::Some(crate::Unit::fr(x)) }
			#[inline] pub fn dur( x: impl ::num_traits::cast::AsPrimitive<f32>) -> Property { Property::Some(crate::Unit::dur(x)) }

			// for calc
			#[inline] pub fn unit(x: crate::Unit) -> Property { Property::Some(x) }
		},
		Value::String => quote! {#[inline] pub fn str(x: impl ::std::convert::Into<String>) -> Property { Property::String(::std::convert::Into::into(x)) }},
		Value::Raw => quote! {#[inline] pub fn raw(x: impl ::std::convert::Into<String>) -> Property { Property::Raw(::std::convert::Into::into(x)) }},
		Value::Number => quote! {#[inline] pub fn val(x: impl ::num_traits::cast::AsPrimitive<i32>) -> Property { Property::Number(::num_traits::cast::AsPrimitive::<i32>::as_(x)) }},
		Value::Float => quote! {#[inline] pub fn val(x: impl ::num_traits::cast::AsPrimitive<f32>) -> Property { Property::Number(unsafe { crate::units::F32::new_unchecked(::num_traits::cast::AsPrimitive::<f32>::as_(x)) }) }},
	});

	let res = quote!(
		pub mod #property_snek {
			#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
			pub enum Property {
				#(#enum_members)*
			}

			impl ::std::fmt::Display for Property {
				fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
					match self {
						#(#display_lines)*
					}
				}
			}

			#(#fn_values)*
		}

		pub type #property_camel = #property_snek::Property;

		#[macro_export]
		macro_rules! #property_snek {
			#(#macro_values)*
		}

		#[test]
		fn #test_fn_name() {
			assert_eq!(#property_snek!(initial).to_string(), #result_initial);
			assert_eq!(#property_snek!(inherit).to_string(), #result_inherit);
			assert_eq!(#property_snek!(unset).to_string(), #result_unset);
		}
	);

	// if input.prefixed {
	//     println!("{}", res.to_string());
	// }

	res.into()
}

pub fn easy_join(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	#[derive(Debug)]
	struct Input {
		name: syn::Ident,
		props: Vec<syn::Ident>,
		vals: Vec<Value>,
	}

	impl syn::parse::Parse for Input {
		fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
			let name = input.parse()?;
			input.parse::<syn::Token![,]>()?;

			let content; syn::parenthesized!(content in input);
			let props = syn::punctuated::Punctuated::<syn::Ident, syn::Token![,]>::parse_terminated(&content)?;

			input.parse::<syn::Token![,]>()?;

			let content; syn::parenthesized!(content in input);
			let vals = syn::punctuated::Punctuated::<Value, syn::Token![,]>::parse_terminated(&content)?.into_iter()
				.chain(["initial", "inherit", "unset"].into_iter().map(|s| Value::EnumVariant(HyphenatedName(s.to_owned()))));

			Ok(Input { name, props: props.into_iter().collect(), vals: vals.collect() })
		}
	}

	let Input { name, props, vals } = syn::parse_macro_input!(input as Input);

	let items = vals.into_iter().map(|val| match val {
		Value::EnumVariant(val) => {
			let value_snek = proc_macro2::Ident::new_raw(match &val.0.to_snek_case() as &str {
				"super" => "super_",
				"crate" => "crate_",
				"self" => "self_",
				"Self" => "Self_",
				x => x,
			}, Span::call_site());
			quote! { pub const #value_snek: (#(super::#props::Property),*) = (#(super::#props::#value_snek),*); }
		},
		Value::Unit => {
			let funs = ["px", "pct", "em", "rem", "vh", "vw", "vmin", "vmax", "fr", "dur"].iter().map(|fname| {
				let fname = proc_macro2::Ident::new(fname, Span::call_site());
				quote! {#[inline] pub fn #fname(x: impl ::num_traits::cast::AsPrimitive<f32>) -> (#(super::#props::Property),*) { (#(super::#props::#fname(x)),*) }}
			});
			quote! {
				#[inline] pub fn zero() -> (#(super::#props::Property),*) { (#(super::#props::zero()),*) }
				#(#funs)*
				#[inline] pub fn unit(x: crate::Unit) -> (#(super::#props::Property),*) { (#(super::#props::unit(x.clone())),*) }
			}
		},
		Value::String => quote! {#[inline] pub fn str(x: impl ::std::convert::Into<String>) -> (#(super::#props::Property),*) { let x = x.into(); (#(super::#props::str(x.clone())),*) }},
		Value::Raw => quote! {#[inline] pub fn raw(x: impl ::std::convert::Into<String>) -> (#(super::#props::Property),*) { let x = x.into(); (#(super::#props::raw(x.clone())),*) }},
		Value::Number => quote! {#[inline] pub fn val(x: impl ::num_traits::cast::AsPrimitive<i32>) -> (#(super::#props::Property),*) { (#(super::#props::val(x)),*) }},
		Value::Float => quote! {#[inline] pub fn val(x: impl ::num_traits::cast::AsPrimitive<f32>) -> (#(super::#props::Property),*) { (#(super::#props::val(x)),*) }},
	});

	let res = quote! {
		#[allow(non_upper_case_globals)]
		pub mod #name {
			#(#items)*
		}
	};

	// dbg!(res.to_string());

	return res.into()
}

pub fn easy_color(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let property = input.into_iter().map(|x| x.to_string()).collect::<String>();

	let property_snek = proc_macro2::Ident::new(&property.to_snek_case(), Span::call_site());
	let property_camel = proc_macro2::Ident::new(&property.to_upper_camel_case(), Span::call_site());

	let test_fn_name = quote::format_ident!("{}_initial_inherit_unset", property_snek);
	let result_initial = format!("{}:initial;", property.to_kebab_case());
	let result_inherit = format!("{}:inherit;", property.to_kebab_case());
	let result_unset = format!("{}:unset;", property.to_kebab_case());

	// TODO: macroless
	let res = quote!(
		#[macro_export]
		macro_rules! #property_snek {
			(initial)       => {$crate::Property::#property_camel($crate::ColorValue::Initial)};
			(inherit)       => {$crate::Property::#property_camel($crate::ColorValue::Inherit)};
			(unset)         => {$crate::Property::#property_camel($crate::ColorValue::Unset)};
			(gray $c:expr)  => {$crate::Property::#property_camel($crate::ColorValue::Rgba($crate::Color { r: $c, g: $c, b: $c, a: 0xFF }))};
			(rgb $rgb:expr) => {$crate::Property::#property_camel($crate::ColorValue::Rgba(($rgb << 8 | 0xFF).into()))};
			($rgba:expr)    => {$crate::Property::#property_camel($crate::ColorValue::Rgba($rgba.into()))};
		}

		#[test]
		fn #test_fn_name() {
			assert_eq!(#property_snek!(initial).to_string(), #result_initial);
			assert_eq!(#property_snek!(inherit).to_string(), #result_inherit);
			assert_eq!(#property_snek!(unset).to_string(), #result_unset);
		}
	);

	res.into()
}
