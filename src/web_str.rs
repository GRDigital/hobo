macro_rules! intern_strings {
	($($name:ident, $s:expr);+$(;)*) => {$(
		#[inline(always)]
		pub fn $name() -> &'static str { wasm_bindgen::intern($s) }
	)+}
}

intern_strings! {
	class, "class";
	r#type, "type";
	range, "range";
	button, "button";
	min, "min";
	max, "max";
	value, "value";
	style, "style";
	placeholder, "placeholder";

	// events
	click, "click";
	contextmenu, "contextmenu";
	dblclick, "dblclick";
	mousedown, "mousedown";
	mouseenter, "mouseenter";
	mouseleave, "mouseleave";
	mousemove, "mousemove";
	mouseover, "mouseover";
	mouseout, "mouseout";
	mouseup, "mouseup";
	change, "change";
	keydown, "keydown";
	keyup, "keyup";

	// elements
	div, "div";
	span, "span";
	input, "input";
	a, "a";
	img, "img";
	textarea, "textarea";
	script, "script";
}
