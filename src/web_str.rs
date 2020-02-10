#![allow(non_snake_case)]

macro_rules! intern_strings_line {
	($name:ident, $s:expr) => {
		#[inline(always)]
		pub fn $name() -> &'static str { wasm_bindgen::intern($s) }
	};
	($name:ident) => {
		#[inline(always)]
		pub fn $name() -> &'static str { wasm_bindgen::intern(stringify!($name)) }
	};
}

macro_rules! intern_strings {
	(
		current = ()
		rest = ($(;)*)
	) => {};
	(
		current = ($($current:tt)*)
		rest = (; $($rest:tt)*)
	) => {
		intern_strings_line! {$($current)*}
		intern_strings! {
			current = ()
			rest = ($($rest)*)
		}
	};
	(
		current = ($($current:tt)*)
		rest = ($tt:tt $($rest:tt)*)
	) => {
		intern_strings! {
			current = ($($current)* $tt)
			rest = ($($rest)*)
		}
	};
	($($tt:tt)+) => {
		intern_strings! {
			current = ()
			rest = ($($tt)+)
		}
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

	// events
	click; contextmenu; dblclick; mousedown; mouseenter;
	mouseleave; mousemove; mouseover; mouseout; mouseup;
	change; keydown; keyup; scroll;

	// elements
	div; span; input; a; img; p;
	textarea; script; object; iframe; embed;
	select; option; nav; filter; svg;
	feColorMatrix; footer; address;
	h1; h2; h3; h4; h5; h6;
}
