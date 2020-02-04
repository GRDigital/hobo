#![allow(non_snake_case)]

macro_rules! intern_strings {
	($($name:ident, $s:expr);+$(;)*) => {$(
		#[inline(always)]
		pub fn $name() -> &'static str { wasm_bindgen::intern($s) }
	)+};
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
	src, "src";
	href, "href";
	disabled, "disabled";
	selected, "selected";
	hidden, "hidden";
	beforebegin, "beforebegin";
	afterbegin, "afterbegin";
	beforeend, "beforeend";
	afterend, "afterend";
	checkbox, "checkbox";
	radio, "radio";
	accept, "accept";
	alt, "alt";
	checked, "checked";
	step, "step";
	number, "number";
	multiple, "multiple";
	readonly, "readonly";
	required, "required";
	reversed, "reversed";
	rows, "rows";
	tabindex, "tabindex";
	target, "target";
	width, "width";
	height, "height";
	wrap, "wrap";
	autofocus, "autofocus";
	autoplay, "autoplay";
	r#async, "async";
	autocomplete, "autocomplete";
	download, "download";
	draggable, "draggable";
	dropzone, "dropzone";
	id, "id";

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
	object, "object";
	iframe, "iframe";
	embed, "embed";
	select, "select";
	option, "option";
	nav, "nav";
	filter, "filter";
	svg, "svg";
	feColorMatrix, "feColorMatrix";
	footer, "footer";
	address, "address";
	h1, "h1";
	h2, "h2";
	h3, "h3";
	h4, "h4";
	h5, "h5";
	h6, "h6";
}
