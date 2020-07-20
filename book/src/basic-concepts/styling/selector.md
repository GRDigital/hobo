# Selector

hobo selectors mirror css selectors with minor changes:

* descendant selectors like `div a` become `div >> a` because Rust doesn't have semantic whitespaces.
	* selectors like `div.active` work the same (except have to be written like `div."active"` or `div ."active"`)
* pseudo-classes use `_` instead of `-` and must always use single colon syntax, e.g. `:active` or `:last_child`
	* there's an escape hatch in `:raw("-webkit-prop".to_string())` for browser-specific or other weird things
* pseudo-elements use `_` instead of `-` and must always use double colon syntax, e.g. `::after` or `::first_line`
* `.&` will be replaced at runtime with the name of a class, which will be generated from the rules in the style it belongs to
	* in other words, it's similar to `&` in SASS or `styled-components`
