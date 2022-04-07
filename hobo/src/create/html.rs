use std::borrow::Cow;
use super::*;

impl A {
	#[inline]
	pub fn href<'a>(self, href: impl Into<Cow<'a, str>>) -> Self {
		self.set_href(href);
		self
	}

	#[inline]
	pub fn set_href<'a>(self, href: impl Into<Cow<'a, str>>) {
		self.set_attr(web_str::href(), href);
	}
}

impl Input {
	pub async fn file_data(&self, id: u32) -> ::std::option::Option<Vec<u8>> {
		let file = self.get_cmp::<web_sys::HtmlInputElement>().files()?.get(id)?;
		let arr_buffer: js_sys::ArrayBuffer = wasm_bindgen_futures::JsFuture::from(file.array_buffer()).await.ok()?.dyn_into().ok()?;
		let vec = js_sys::Uint8Array::new(&arr_buffer).to_vec();
		Some(vec)
	}

	#[inline]
	pub fn type_text(self) -> Self {
		self.set_type_text(); self
	}

	#[inline]
	pub fn set_type_text(self) {
		self.set_attr(web_str::r#type(), web_str::text());
	}

	#[inline]
	pub fn type_checkbox(self) -> Self {
		self.set_type_checkbox(); self
	}

	#[inline]
	pub fn set_type_checkbox(self) {
		self.set_attr(web_str::r#type(), web_str::checkbox());
	}

	#[inline]
	pub fn type_date(self) -> Self {
		self.set_type_date(); self
	}

	#[inline]
	pub fn set_type_date(self) {
		self.set_attr(web_str::r#type(), web_str::date());
	}

	#[inline]
	pub fn type_datetime_local(self) -> Self {
		self.set_type_datetime_local(); self
	}

	#[inline]
	pub fn set_type_datetime_local(self) {
		self.set_attr(web_str::r#type(), web_str::datetime_local());
	}

	#[inline]
	pub fn type_time(self) -> Self {
		self.set_type_time(); self
	}

	#[inline]
	pub fn set_type_time(self) {
		self.set_attr(web_str::r#type(), web_str::time());
	}

	#[inline]
	pub fn type_file(self) -> Self {
		self.set_type_file(); self
	}

	#[inline]
	pub fn set_type_file(self) {
		self.set_attr(web_str::r#type(), web_str::file());
	}

	#[inline]
	pub fn type_number(self) -> Self {
		self.set_type_number(); self
	}

	#[inline]
	pub fn set_type_number(self) {
		self.set_attr(web_str::r#type(), web_str::number());
	}

	#[inline]
	pub fn type_password(self) -> Self {
		self.set_type_password(); self
	}

	#[inline]
	pub fn set_type_password(self) {
		self.set_attr(web_str::r#type(), web_str::password());
	}

	#[inline]
	pub fn type_radio(self) -> Self {
		self.set_type_radio(); self
	}

	#[inline]
	pub fn set_type_radio(self) {
		self.set_attr(web_str::r#type(), web_str::radio());
	}

	#[inline]
	pub fn type_range(self) -> Self {
		self.set_type_range(); self
	}

	#[inline]
	pub fn set_type_range(self) {
		self.set_attr(web_str::r#type(), web_str::range());
	}

	#[inline]
	pub fn type_tel(self) -> Self {
		self.set_type_tel(); self
	}

	#[inline]
	pub fn set_type_tel(self) {
		self.set_attr(web_str::r#type(), web_str::tel());
	}

	#[inline]
	pub fn type_url(self) -> Self {
		self.set_type_url(); self
	}

	#[inline]
	pub fn set_type_url(self) {
		self.set_attr(web_str::r#type(), web_str::url());
	}
}

pub trait StringValue {
	fn value(&self) -> String;
	fn set_value(&self, x: &str);
}

impl StringValue for Input {
	#[inline]
	fn value(&self) -> String { self.get_cmp::<web_sys::HtmlInputElement>().value() }

	#[inline]
	fn set_value(&self, x: &str) { self.get_cmp::<web_sys::HtmlInputElement>().set_value(x) }
}

impl StringValue for Textarea {
	#[inline]
	fn value(&self) -> String { self.get_cmp::<web_sys::HtmlTextAreaElement>().value() }

	#[inline]
	fn set_value(&self, x: &str) { self.get_cmp::<web_sys::HtmlTextAreaElement>().set_value(x) }
}

impl Select {
	#[inline]
	pub fn selected_index(&self) -> i32 {
		self.get_cmp::<web_sys::HtmlSelectElement>().selected_index()
	}
}

impl Image {
	#[inline]
	pub fn src(self, url: &str) -> Self {
		self.attr(web_str::src(), url)
	}

	#[inline]
	pub fn set_src(&self, url: &str) {
		self.set_attr(web_str::src(), url)
	}
}
