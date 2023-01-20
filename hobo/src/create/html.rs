use std::borrow::Cow;
use super::*;

pub trait BasicAttrs: AsElement {
	#[inline] fn set_name<'a>(&self, x: impl Into<Cow<'a, str>>) { self.set_attr(web_str::name(), x); }
	#[must_use] #[inline] fn name<'a>(self, x: impl Into<Cow<'a, str>>) -> Self where Self: Sized { self.set_name(x); self }

	#[inline] fn set_id<'a>(&self, x: impl Into<Cow<'a, str>>) { self.set_attr(web_str::id(), x); }
	#[must_use] #[inline] fn id<'a>(self, x: impl Into<Cow<'a, str>>) -> Self where Self: Sized { self.set_id(x); self }
}

impl<T: AsElement> BasicAttrs for T {}

impl A {
	#[must_use] #[inline] pub fn href<'a>(self, href: impl Into<Cow<'a, str>>) -> Self { self.set_href(href); self }
	#[inline] pub fn set_href<'a>(self, href: impl Into<Cow<'a, str>>) { self.set_attr(web_str::href(), href); }
}

impl Input {
	pub async fn file_data(&self, id: u32) -> ::std::option::Option<Vec<u8>> {
		let file = self.get_cmp::<web_sys::HtmlInputElement>().files()?.get(id)?;
		let arr_buffer: js_sys::ArrayBuffer = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await.ok()?.dyn_into().ok()?;
		let vec = js_sys::Uint8Array::new(&arr_buffer).to_vec();
		Some(vec)
	}

	#[must_use] #[inline] pub fn type_text(self) -> Self { self.set_type_text(); self }
	#[inline] pub fn set_type_text(&self) { self.set_attr(web_str::r#type(), web_str::text()); }

	#[must_use] #[inline] pub fn type_checkbox(self) -> Self { self.set_type_checkbox(); self }
	#[inline] pub fn set_type_checkbox(&self) { self.set_attr(web_str::r#type(), web_str::checkbox()); }

	#[must_use] #[inline] pub fn type_date(self) -> Self { self.set_type_date(); self }
	#[inline] pub fn set_type_date(&self) { self.set_attr(web_str::r#type(), web_str::date()); }

	#[must_use] #[inline] pub fn type_datetime_local(self) -> Self { self.set_type_datetime_local(); self }
	#[inline] pub fn set_type_datetime_local(&self) { self.set_attr(web_str::r#type(), web_str::datetime_local()); }

	#[must_use] #[inline] pub fn type_time(self) -> Self { self.set_type_time(); self }
	#[inline] pub fn set_type_time(&self) { self.set_attr(web_str::r#type(), web_str::time()); }

	#[must_use] #[inline] pub fn type_file(self) -> Self { self.set_type_file(); self }
	#[inline] pub fn set_type_file(&self) { self.set_attr(web_str::r#type(), web_str::file()); }

	#[must_use] #[inline] pub fn type_number(self) -> Self { self.set_type_number(); self }
	#[inline] pub fn set_type_number(&self) { self.set_attr(web_str::r#type(), web_str::number()); }

	#[must_use] #[inline] pub fn type_password(self) -> Self { self.set_type_password(); self }
	#[inline] pub fn set_type_password(&self) { self.set_attr(web_str::r#type(), web_str::password()); }

	#[must_use] #[inline] pub fn type_radio(self) -> Self { self.set_type_radio(); self }
	#[inline] pub fn set_type_radio(&self) { self.set_attr(web_str::r#type(), web_str::radio()); }

	#[must_use] #[inline] pub fn type_range(self) -> Self { self.set_type_range(); self }
	#[inline] pub fn set_type_range(&self) { self.set_attr(web_str::r#type(), web_str::range()); }

	#[must_use] #[inline] pub fn type_tel(self) -> Self { self.set_type_tel(); self }
	#[inline] pub fn set_type_tel(&self) { self.set_attr(web_str::r#type(), web_str::tel()); }

	#[must_use] #[inline] pub fn type_url(self) -> Self { self.set_type_url(); self }
	#[inline] pub fn set_type_url(&self) { self.set_attr(web_str::r#type(), web_str::url()); }

	#[must_use] #[inline] pub fn placeholder<'a>(self, placeholder: impl Into<Cow<'a, str>>) -> Self { self.set_placeholder(placeholder); self }
	#[inline] pub fn set_placeholder<'a>(&self, placeholder: impl Into<Cow<'a, str>>) { self.set_attr(web_str::placeholder(), placeholder); }

	#[inline] pub fn get_checked(&self) -> bool { self.get_cmp::<web_sys::HtmlInputElement>().checked() }
}

pub trait StringValue: AsElement {
	fn value_attr<'a>(self, value: impl Into<Cow<'a, str>>) -> Self where Self: Sized { self.set_value_attr(value); self }
	fn set_value_attr<'a>(&self, value: impl Into<Cow<'a, str>>) { self.set_attr(web_str::value(), value) }
	fn get_value(&self) -> String;
	fn set_value(&self, x: &str);
}

impl StringValue for Input {
	#[inline] fn get_value(&self) -> String { self.get_cmp::<web_sys::HtmlInputElement>().value() }
	#[inline] fn set_value(&self, x: &str) { self.get_cmp::<web_sys::HtmlInputElement>().set_value(x) }
}

impl StringValue for Textarea {
	#[inline] fn get_value(&self) -> String { self.get_cmp::<web_sys::HtmlTextAreaElement>().value() }
	#[inline] fn set_value(&self, x: &str) { self.get_cmp::<web_sys::HtmlTextAreaElement>().set_value(x) }
}

impl Select {
	#[inline] pub fn selected_index(&self) -> i32 { self.get_cmp::<web_sys::HtmlSelectElement>().selected_index() }
}

impl Img {
	#[inline] pub fn src<'a>(self, url: impl Into<Cow<'a, str>>) -> Self { self.attr(web_str::src(), url) }
	#[inline] pub fn set_src<'a>(&self, url: impl Into<Cow<'a, str>>) { self.set_attr(web_str::src(), url) }
}

impl Script {
	#[inline] pub fn src<'a>(self, url: impl Into<Cow<'a, str>>) -> Self { self.attr(web_str::src(), url) }
	#[inline] pub fn set_src<'a>(&self, url: impl Into<Cow<'a, str>>) { self.set_attr(web_str::src(), url) }
}

impl Label {
	#[inline] pub fn set_for_ctrl<'a>(&self, x: impl Into<Cow<'a, str>>) { self.set_attr(web_str::r#for(), x); }
	#[inline] pub fn for_ctrl<'a>(self, x: impl Into<Cow<'a, str>>) -> Self where Self: Sized { self.set_for_ctrl(x); self }
}

impl Form {
	#[inline] pub fn radio_value<'a>(&self, x: impl Into<Cow<'a, str>>) -> std::option::Option<String> {
		self.get_cmp::<web_sys::HtmlFormElement>()
			.elements()
			.get_with_name(&x.into())
			.and_then(|x| x.dyn_into::<web_sys::RadioNodeList>().ok())
			.map(|x| x.value())
	}
}
