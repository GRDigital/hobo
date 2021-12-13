use hobo_css as css;

#[test]
fn initial_inherit_unset() {
	assert_eq!(css::bottom!(initial).to_string(), "bottom:initial;");
	assert_eq!(css::bottom!(inherit).to_string(), "bottom:inherit;");
	assert_eq!(css::bottom!(unset).to_string(), "bottom:unset;");

	assert_eq!(css::left!(initial).to_string(), "left:initial;");
	assert_eq!(css::left!(inherit).to_string(), "left:inherit;");
	assert_eq!(css::left!(unset).to_string(), "left:unset;");

	assert_eq!(css::right!(initial).to_string(), "right:initial;");
	assert_eq!(css::right!(inherit).to_string(), "right:inherit;");
	assert_eq!(css::right!(unset).to_string(), "right:unset;");

	assert_eq!(css::top!(initial).to_string(), "top:initial;");
	assert_eq!(css::top!(inherit).to_string(), "top:inherit;");
	assert_eq!(css::top!(unset).to_string(), "top:unset;");

	assert_eq!(css::grid_auto_flow!(initial).to_string(), "grid-auto-flow:initial;");
	assert_eq!(css::grid_auto_flow!(inherit).to_string(), "grid-auto-flow:inherit;");
	assert_eq!(css::grid_auto_flow!(unset).to_string(), "grid-auto-flow:unset;");

	// assert_eq!(css::grid_template_columns!(initial).to_string(), "grid-template-columns:initial;");
	// assert_eq!(css::grid_template_columns!(inherit).to_string(), "grid-template-columns:inherit;");
	// assert_eq!(css::grid_template_columns!(unset).to_string(), "grid-template-columns:unset;");
	//
	// assert_eq!(css::grid_template_rows!(initial).to_string(), "grid-template-rows:initial;");
	// assert_eq!(css::grid_template_rows!(inherit).to_string(), "grid-template-rows:inherit;");
	// assert_eq!(css::grid_template_rows!(unset).to_string(), "grid-template-rows:unset;");

	assert_eq!(css::margin_bottom!(initial).to_string(), "margin-bottom:initial;");
	assert_eq!(css::margin_bottom!(inherit).to_string(), "margin-bottom:inherit;");
	assert_eq!(css::margin_bottom!(unset).to_string(), "margin-bottom:unset;");

	assert_eq!(css::margin_left!(initial).to_string(), "margin-left:initial;");
	assert_eq!(css::margin_left!(inherit).to_string(), "margin-left:inherit;");
	assert_eq!(css::margin_left!(unset).to_string(), "margin-left:unset;");

	assert_eq!(css::margin_right!(initial).to_string(), "margin-right:initial;");
	assert_eq!(css::margin_right!(inherit).to_string(), "margin-right:inherit;");
	assert_eq!(css::margin_right!(unset).to_string(), "margin-right:unset;");

	assert_eq!(css::margin_top!(initial).to_string(), "margin-top:initial;");
	assert_eq!(css::margin_top!(inherit).to_string(), "margin-top:inherit;");
	assert_eq!(css::margin_top!(unset).to_string(), "margin-top:unset;");

	assert_eq!(css::max_height!(initial).to_string(), "max-height:initial;");
	assert_eq!(css::max_height!(inherit).to_string(), "max-height:inherit;");
	assert_eq!(css::max_height!(unset).to_string(), "max-height:unset;");

	assert_eq!(css::max_width!(initial).to_string(), "max-width:initial;");
	assert_eq!(css::max_width!(inherit).to_string(), "max-width:inherit;");
	assert_eq!(css::max_width!(unset).to_string(), "max-width:unset;");

	assert_eq!(css::min_height!(initial).to_string(), "min-height:initial;");
	assert_eq!(css::min_height!(inherit).to_string(), "min-height:inherit;");
	assert_eq!(css::min_height!(unset).to_string(), "min-height:unset;");

	assert_eq!(css::min_width!(initial).to_string(), "min-width:initial;");
	assert_eq!(css::min_width!(inherit).to_string(), "min-width:inherit;");
	assert_eq!(css::min_width!(unset).to_string(), "min-width:unset;");

	assert_eq!(css::height!(initial).to_string(), "height:initial;");
	assert_eq!(css::height!(inherit).to_string(), "height:inherit;");
	assert_eq!(css::height!(unset).to_string(), "height:unset;");

	assert_eq!(css::width!(initial).to_string(), "width:initial;");
	assert_eq!(css::width!(inherit).to_string(), "width:inherit;");
	assert_eq!(css::width!(unset).to_string(), "width:unset;");

	assert_eq!(css::z_index!(initial).to_string(), "z-index:initial;");
	assert_eq!(css::z_index!(inherit).to_string(), "z-index:inherit;");
	assert_eq!(css::z_index!(unset).to_string(), "z-index:unset;");
}
