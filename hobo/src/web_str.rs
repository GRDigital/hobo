#![allow(non_snake_case)]

//! interned strings
//!
//! all of these functions return interned strings    
//! to find out more about string interning, refer to [wasm-bindgen documentation](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/fn.intern.html)

#[doc(inline)] pub use crate::create::strings::*;

// interning disabled in debug mode to help track memory leaks
macro_rules! intern_strings {
	() => {};
	($name:ident, $s:expr; $($rest:tt)*) => {
		pub fn $name() -> &'static str {
			#[cfg(debug_assertions)]
			{$s}

			#[cfg(not(debug_assertions))]
			{wasm_bindgen::intern($s)}
		}
		intern_strings! {$($rest)*}
	};
	($name:ident; $($rest:tt)*) => {
		pub fn $name() -> &'static str {
			#[cfg(debug_assertions)]
			{stringify!($name)}

			#[cfg(not(debug_assertions))]
			{wasm_bindgen::intern(stringify!($name))}
		}
		intern_strings! {$($rest)*}
	};
}

intern_strings! {
	class;
	r#type, "type";
	range;
	button;
	min; max; value; step;
	style;
	placeholder;
	src;
	href;
	disabled; selected;
	hidden;
	beforebegin; afterbegin; beforeend; afterend;
	checkbox; radio;
	accept;
	alt;
	checked;
	number;
	multiple;
	readonly;
	required;
	reversed;
	rows;
	tabindex;
	target;
	width; height;
	wrap;
	autofocus; autoplay;
	r#async, "async";
	autocomplete;
	download;
	draggable;
	dropzone;
	id;
	password;
	text;
	_blank; rel; noopener;
	viewBox; d; cx; cy; r; x; y; x1; x2; y1; y2;
	file;
	loading; lazy;

	// events
	click; contextmenu; dblclick; mousedown; mouseenter;
	mouseleave; mousemove; mouseover; mouseout; mouseup;
	change; keydown; keyup; scroll; resize; blur; focus;
	touchstart; touchend; touchmove; touchcancel; wheel;
	load; canplay;
}
