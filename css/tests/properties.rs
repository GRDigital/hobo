use hobo_css as css;

#[test]
fn initial_inherit_unset() {
	assert_eq!(css::align_content!(initial).to_string(), "align-content:initial;");
	assert_eq!(css::align_content!(inherit).to_string(), "align-content:inherit;");
	assert_eq!(css::align_content!(unset).to_string(), "align-content:unset;");

	assert_eq!(css::align_items!(initial).to_string(), "align-items:initial;");
	assert_eq!(css::align_items!(inherit).to_string(), "align-items:inherit;");
	assert_eq!(css::align_items!(unset).to_string(), "align-items:unset;");

	assert_eq!(css::align_self!(initial).to_string(), "align-self:initial;");
	assert_eq!(css::align_self!(inherit).to_string(), "align-self:inherit;");
	assert_eq!(css::align_self!(unset).to_string(), "align-self:unset;");

	assert_eq!(css::appearance!(initial).to_string(), "appearance:initial;-webkit-appearance:initial;-moz-appearance:initial;");
	assert_eq!(css::appearance!(inherit).to_string(), "appearance:inherit;-webkit-appearance:inherit;-moz-appearance:inherit;");
	assert_eq!(css::appearance!(unset).to_string(), "appearance:unset;-webkit-appearance:unset;-moz-appearance:unset;");

	assert_eq!(css::background_blend_mode!(initial).to_string(), "background-blend-mode:initial;");
	assert_eq!(css::background_blend_mode!(inherit).to_string(), "background-blend-mode:inherit;");
	assert_eq!(css::background_blend_mode!(unset).to_string(), "background-blend-mode:unset;");

	assert_eq!(css::background_position_x!(initial).to_string(), "background-position-x:initial;");
	assert_eq!(css::background_position_x!(inherit).to_string(), "background-position-x:inherit;");
	assert_eq!(css::background_position_x!(unset).to_string(), "background-position-x:unset;");

	assert_eq!(css::background_position_y!(initial).to_string(), "background-position-y:initial;");
	assert_eq!(css::background_position_y!(inherit).to_string(), "background-position-y:inherit;");
	assert_eq!(css::background_position_y!(unset).to_string(), "background-position-y:unset;");

	assert_eq!(css::border_bottom_color!(initial).to_string(), "border-bottom-color:initial;");
	assert_eq!(css::border_bottom_color!(inherit).to_string(), "border-bottom-color:inherit;");
	assert_eq!(css::border_bottom_color!(unset).to_string(), "border-bottom-color:unset;");

	assert_eq!(css::border_bottom_left_radius!(initial).to_string(), "border-bottom-left-radius:initial;");
	assert_eq!(css::border_bottom_left_radius!(inherit).to_string(), "border-bottom-left-radius:inherit;");
	assert_eq!(css::border_bottom_left_radius!(unset).to_string(), "border-bottom-left-radius:unset;");

	assert_eq!(css::border_bottom_right_radius!(initial).to_string(), "border-bottom-right-radius:initial;");
	assert_eq!(css::border_bottom_right_radius!(inherit).to_string(), "border-bottom-right-radius:inherit;");
	assert_eq!(css::border_bottom_right_radius!(unset).to_string(), "border-bottom-right-radius:unset;");

	assert_eq!(css::border_bottom_style!(initial).to_string(), "border-bottom-style:initial;");
	assert_eq!(css::border_bottom_style!(inherit).to_string(), "border-bottom-style:inherit;");
	assert_eq!(css::border_bottom_style!(unset).to_string(), "border-bottom-style:unset;");

	assert_eq!(css::border_bottom_width!(initial).to_string(), "border-bottom-width:initial;");
	assert_eq!(css::border_bottom_width!(inherit).to_string(), "border-bottom-width:inherit;");
	assert_eq!(css::border_bottom_width!(unset).to_string(), "border-bottom-width:unset;");

	assert_eq!(css::border_collapse!(initial).to_string(), "border-collapse:initial;");
	assert_eq!(css::border_collapse!(inherit).to_string(), "border-collapse:inherit;");
	assert_eq!(css::border_collapse!(unset).to_string(), "border-collapse:unset;");

	assert_eq!(css::border_image_outset!(initial).to_string(), "border-image-outset:initial;");
	assert_eq!(css::border_image_outset!(inherit).to_string(), "border-image-outset:inherit;");
	assert_eq!(css::border_image_outset!(unset).to_string(), "border-image-outset:unset;");

	assert_eq!(css::border_image_repeat!(initial).to_string(), "border-image-repeat:initial;");
	assert_eq!(css::border_image_repeat!(inherit).to_string(), "border-image-repeat:inherit;");
	assert_eq!(css::border_image_repeat!(unset).to_string(), "border-image-repeat:unset;");

	assert_eq!(css::border_image_slice!(initial).to_string(), "border-image-slice:initial;");
	assert_eq!(css::border_image_slice!(inherit).to_string(), "border-image-slice:inherit;");
	assert_eq!(css::border_image_slice!(unset).to_string(), "border-image-slice:unset;");

	assert_eq!(css::border_image_width!(initial).to_string(), "border-image-width:initial;");
	assert_eq!(css::border_image_width!(inherit).to_string(), "border-image-width:inherit;");
	assert_eq!(css::border_image_width!(unset).to_string(), "border-image-width:unset;");

	assert_eq!(css::border_left_color!(initial).to_string(), "border-left-color:initial;");
	assert_eq!(css::border_left_color!(inherit).to_string(), "border-left-color:inherit;");
	assert_eq!(css::border_left_color!(unset).to_string(), "border-left-color:unset;");

	assert_eq!(css::border_left_style!(initial).to_string(), "border-left-style:initial;");
	assert_eq!(css::border_left_style!(inherit).to_string(), "border-left-style:inherit;");
	assert_eq!(css::border_left_style!(unset).to_string(), "border-left-style:unset;");

	assert_eq!(css::border_left_width!(initial).to_string(), "border-left-width:initial;");
	assert_eq!(css::border_left_width!(inherit).to_string(), "border-left-width:inherit;");
	assert_eq!(css::border_left_width!(unset).to_string(), "border-left-width:unset;");

	assert_eq!(css::border_right_color!(initial).to_string(), "border-right-color:initial;");
	assert_eq!(css::border_right_color!(inherit).to_string(), "border-right-color:inherit;");
	assert_eq!(css::border_right_color!(unset).to_string(), "border-right-color:unset;");

	assert_eq!(css::border_right_style!(initial).to_string(), "border-right-style:initial;");
	assert_eq!(css::border_right_style!(inherit).to_string(), "border-right-style:inherit;");
	assert_eq!(css::border_right_style!(unset).to_string(), "border-right-style:unset;");

	assert_eq!(css::border_right_width!(initial).to_string(), "border-right-width:initial;");
	assert_eq!(css::border_right_width!(inherit).to_string(), "border-right-width:inherit;");
	assert_eq!(css::border_right_width!(unset).to_string(), "border-right-width:unset;");

	assert_eq!(css::border_top_color!(initial).to_string(), "border-top-color:initial;");
	assert_eq!(css::border_top_color!(inherit).to_string(), "border-top-color:inherit;");
	assert_eq!(css::border_top_color!(unset).to_string(), "border-top-color:unset;");

	assert_eq!(css::border_top_left_radius!(initial).to_string(), "border-top-left-radius:initial;");
	assert_eq!(css::border_top_left_radius!(inherit).to_string(), "border-top-left-radius:inherit;");
	assert_eq!(css::border_top_left_radius!(unset).to_string(), "border-top-left-radius:unset;");

	assert_eq!(css::border_top_right_radius!(initial).to_string(), "border-top-right-radius:initial;");
	assert_eq!(css::border_top_right_radius!(inherit).to_string(), "border-top-right-radius:inherit;");
	assert_eq!(css::border_top_right_radius!(unset).to_string(), "border-top-right-radius:unset;");

	assert_eq!(css::border_top_style!(initial).to_string(), "border-top-style:initial;");
	assert_eq!(css::border_top_style!(inherit).to_string(), "border-top-style:inherit;");
	assert_eq!(css::border_top_style!(unset).to_string(), "border-top-style:unset;");

	assert_eq!(css::border_top_width!(initial).to_string(), "border-top-width:initial;");
	assert_eq!(css::border_top_width!(inherit).to_string(), "border-top-width:inherit;");
	assert_eq!(css::border_top_width!(unset).to_string(), "border-top-width:unset;");

	assert_eq!(css::bottom!(initial).to_string(), "bottom:initial;");
	assert_eq!(css::bottom!(inherit).to_string(), "bottom:inherit;");
	assert_eq!(css::bottom!(unset).to_string(), "bottom:unset;");

	assert_eq!(css::box_decoration_break!(initial).to_string(), "box-decoration-break:initial;");
	assert_eq!(css::box_decoration_break!(inherit).to_string(), "box-decoration-break:inherit;");
	assert_eq!(css::box_decoration_break!(unset).to_string(), "box-decoration-break:unset;");

	assert_eq!(css::box_sizing!(initial).to_string(), "box-sizing:initial;");
	assert_eq!(css::box_sizing!(inherit).to_string(), "box-sizing:inherit;");
	assert_eq!(css::box_sizing!(unset).to_string(), "box-sizing:unset;");

	assert_eq!(css::break_after!(initial).to_string(), "break-after:initial;");
	assert_eq!(css::break_after!(inherit).to_string(), "break-after:inherit;");
	assert_eq!(css::break_after!(unset).to_string(), "break-after:unset;");

	assert_eq!(css::break_before!(initial).to_string(), "break-before:initial;");
	assert_eq!(css::break_before!(inherit).to_string(), "break-before:inherit;");
	assert_eq!(css::break_before!(unset).to_string(), "break-before:unset;");

	assert_eq!(css::break_inside!(initial).to_string(), "break-inside:initial;");
	assert_eq!(css::break_inside!(inherit).to_string(), "break-inside:inherit;");
	assert_eq!(css::break_inside!(unset).to_string(), "break-inside:unset;");

	assert_eq!(css::caption_side!(initial).to_string(), "caption-side:initial;");
	assert_eq!(css::caption_side!(inherit).to_string(), "caption-side:inherit;");
	assert_eq!(css::caption_side!(unset).to_string(), "caption-side:unset;");

	assert_eq!(css::color!(initial).to_string(), "color:initial;");
	assert_eq!(css::color!(inherit).to_string(), "color:inherit;");
	assert_eq!(css::color!(unset).to_string(), "color:unset;");

	assert_eq!(css::column_gap!(initial).to_string(), "column-gap:initial;");
	assert_eq!(css::column_gap!(inherit).to_string(), "column-gap:inherit;");
	assert_eq!(css::column_gap!(unset).to_string(), "column-gap:unset;");

	assert_eq!(css::content!(initial).to_string(), "content:initial;");
	assert_eq!(css::content!(inherit).to_string(), "content:inherit;");
	assert_eq!(css::content!(unset).to_string(), "content:unset;");

	assert_eq!(css::cursor!(initial).to_string(), "cursor:initial;");
	assert_eq!(css::cursor!(inherit).to_string(), "cursor:inherit;");
	assert_eq!(css::cursor!(unset).to_string(), "cursor:unset;");

	assert_eq!(css::direction!(initial).to_string(), "direction:initial;");
	assert_eq!(css::direction!(inherit).to_string(), "direction:inherit;");
	assert_eq!(css::direction!(unset).to_string(), "direction:unset;");

	assert_eq!(css::display!(initial).to_string(), "display:initial;");
	assert_eq!(css::display!(inherit).to_string(), "display:inherit;");
	assert_eq!(css::display!(unset).to_string(), "display:unset;");

	assert_eq!(css::empty_cells!(initial).to_string(), "empty-cells:initial;");
	assert_eq!(css::empty_cells!(inherit).to_string(), "empty-cells:inherit;");
	assert_eq!(css::empty_cells!(unset).to_string(), "empty-cells:unset;");

	assert_eq!(css::fill!(initial).to_string(), "fill:initial;");
	assert_eq!(css::fill!(inherit).to_string(), "fill:inherit;");
	assert_eq!(css::fill!(unset).to_string(), "fill:unset;");

	assert_eq!(css::flex_basis!(initial).to_string(), "flex-basis:initial;");
	assert_eq!(css::flex_basis!(inherit).to_string(), "flex-basis:inherit;");
	assert_eq!(css::flex_basis!(unset).to_string(), "flex-basis:unset;");

	assert_eq!(css::flex_direction!(initial).to_string(), "flex-direction:initial;");
	assert_eq!(css::flex_direction!(inherit).to_string(), "flex-direction:inherit;");
	assert_eq!(css::flex_direction!(unset).to_string(), "flex-direction:unset;");

	assert_eq!(css::flex_grow!(initial).to_string(), "flex-grow:initial;");
	assert_eq!(css::flex_grow!(inherit).to_string(), "flex-grow:inherit;");
	assert_eq!(css::flex_grow!(unset).to_string(), "flex-grow:unset;");

	assert_eq!(css::flex_shrink!(initial).to_string(), "flex-shrink:initial;");
	assert_eq!(css::flex_shrink!(inherit).to_string(), "flex-shrink:inherit;");
	assert_eq!(css::flex_shrink!(unset).to_string(), "flex-shrink:unset;");

	assert_eq!(css::flex_wrap!(initial).to_string(), "flex-wrap:initial;");
	assert_eq!(css::flex_wrap!(inherit).to_string(), "flex-wrap:inherit;");
	assert_eq!(css::flex_wrap!(unset).to_string(), "flex-wrap:unset;");

	assert_eq!(css::font_family!(initial).to_string(), "font-family:initial;");
	assert_eq!(css::font_family!(inherit).to_string(), "font-family:inherit;");
	assert_eq!(css::font_family!(unset).to_string(), "font-family:unset;");

	assert_eq!(css::font_kerning!(initial).to_string(), "font-kerning:initial;");
	assert_eq!(css::font_kerning!(inherit).to_string(), "font-kerning:inherit;");
	assert_eq!(css::font_kerning!(unset).to_string(), "font-kerning:unset;");

	assert_eq!(css::font_size!(initial).to_string(), "font-size:initial;");
	assert_eq!(css::font_size!(inherit).to_string(), "font-size:inherit;");
	assert_eq!(css::font_size!(unset).to_string(), "font-size:unset;");

	assert_eq!(css::font_stretch!(initial).to_string(), "font-stretch:initial;");
	assert_eq!(css::font_stretch!(inherit).to_string(), "font-stretch:inherit;");
	assert_eq!(css::font_stretch!(unset).to_string(), "font-stretch:unset;");

	assert_eq!(css::font_style!(initial).to_string(), "font-style:initial;");
	assert_eq!(css::font_style!(inherit).to_string(), "font-style:inherit;");
	assert_eq!(css::font_style!(unset).to_string(), "font-style:unset;");

	assert_eq!(css::font_variant!(initial).to_string(), "font-variant:initial;");
	assert_eq!(css::font_variant!(inherit).to_string(), "font-variant:inherit;");
	assert_eq!(css::font_variant!(unset).to_string(), "font-variant:unset;");

	assert_eq!(css::font_weight!(initial).to_string(), "font-weight:initial;");
	assert_eq!(css::font_weight!(inherit).to_string(), "font-weight:inherit;");
	assert_eq!(css::font_weight!(unset).to_string(), "font-weight:unset;");

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
	//
	assert_eq!(css::hanging_punctuation!(initial).to_string(), "hanging-punctuation:initial;");
	assert_eq!(css::hanging_punctuation!(inherit).to_string(), "hanging-punctuation:inherit;");
	assert_eq!(css::hanging_punctuation!(unset).to_string(), "hanging-punctuation:unset;");

	assert_eq!(css::height!(initial).to_string(), "height:initial;");
	assert_eq!(css::height!(inherit).to_string(), "height:inherit;");
	assert_eq!(css::height!(unset).to_string(), "height:unset;");

	assert_eq!(css::hyphens!(initial).to_string(), "hyphens:initial;");
	assert_eq!(css::hyphens!(inherit).to_string(), "hyphens:inherit;");
	assert_eq!(css::hyphens!(unset).to_string(), "hyphens:unset;");

	assert_eq!(css::isolation!(initial).to_string(), "isolation:initial;");
	assert_eq!(css::isolation!(inherit).to_string(), "isolation:inherit;");
	assert_eq!(css::isolation!(unset).to_string(), "isolation:unset;");

	assert_eq!(css::justify_content!(initial).to_string(), "justify-content:initial;");
	assert_eq!(css::justify_content!(inherit).to_string(), "justify-content:inherit;");
	assert_eq!(css::justify_content!(unset).to_string(), "justify-content:unset;");

	assert_eq!(css::justify_items!(initial).to_string(), "justify-items:initial;");
	assert_eq!(css::justify_items!(inherit).to_string(), "justify-items:inherit;");
	assert_eq!(css::justify_items!(unset).to_string(), "justify-items:unset;");

	assert_eq!(css::justify_self!(initial).to_string(), "justify-self:initial;");
	assert_eq!(css::justify_self!(inherit).to_string(), "justify-self:inherit;");
	assert_eq!(css::justify_self!(unset).to_string(), "justify-self:unset;");

	assert_eq!(css::left!(initial).to_string(), "left:initial;");
	assert_eq!(css::left!(inherit).to_string(), "left:inherit;");
	assert_eq!(css::left!(unset).to_string(), "left:unset;");

	assert_eq!(css::letter_spacing!(initial).to_string(), "letter-spacing:initial;");
	assert_eq!(css::letter_spacing!(inherit).to_string(), "letter-spacing:inherit;");
	assert_eq!(css::letter_spacing!(unset).to_string(), "letter-spacing:unset;");

	assert_eq!(css::line_height!(initial).to_string(), "line-height:initial;");
	assert_eq!(css::line_height!(inherit).to_string(), "line-height:inherit;");
	assert_eq!(css::line_height!(unset).to_string(), "line-height:unset;");

	assert_eq!(css::list_style_image!(initial).to_string(), "list-style-image:initial;");
	assert_eq!(css::list_style_image!(inherit).to_string(), "list-style-image:inherit;");
	assert_eq!(css::list_style_image!(unset).to_string(), "list-style-image:unset;");

	assert_eq!(css::list_style_position!(initial).to_string(), "list-style-position:initial;");
	assert_eq!(css::list_style_position!(inherit).to_string(), "list-style-position:inherit;");
	assert_eq!(css::list_style_position!(unset).to_string(), "list-style-position:unset;");

	assert_eq!(css::list_style_type!(initial).to_string(), "list-style-type:initial;");
	assert_eq!(css::list_style_type!(inherit).to_string(), "list-style-type:inherit;");
	assert_eq!(css::list_style_type!(unset).to_string(), "list-style-type:unset;");

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

	assert_eq!(css::mix_blend_mode!(initial).to_string(), "mix-blend-mode:initial;");
	assert_eq!(css::mix_blend_mode!(inherit).to_string(), "mix-blend-mode:inherit;");
	assert_eq!(css::mix_blend_mode!(unset).to_string(), "mix-blend-mode:unset;");

	assert_eq!(css::object_fit!(initial).to_string(), "object-fit:initial;");
	assert_eq!(css::object_fit!(inherit).to_string(), "object-fit:inherit;");
	assert_eq!(css::object_fit!(unset).to_string(), "object-fit:unset;");

	assert_eq!(css::opacity!(initial).to_string(), "opacity:initial;");
	assert_eq!(css::opacity!(inherit).to_string(), "opacity:inherit;");
	assert_eq!(css::opacity!(unset).to_string(), "opacity:unset;");

	assert_eq!(css::order!(initial).to_string(), "order:initial;");
	assert_eq!(css::order!(inherit).to_string(), "order:inherit;");
	assert_eq!(css::order!(unset).to_string(), "order:unset;");

	assert_eq!(css::outline_color!(initial).to_string(), "outline-color:initial;");
	assert_eq!(css::outline_color!(inherit).to_string(), "outline-color:inherit;");
	assert_eq!(css::outline_color!(unset).to_string(), "outline-color:unset;");

	assert_eq!(css::outline_offset!(initial).to_string(), "outline-offset:initial;");
	assert_eq!(css::outline_offset!(inherit).to_string(), "outline-offset:inherit;");
	assert_eq!(css::outline_offset!(unset).to_string(), "outline-offset:unset;");

	assert_eq!(css::outline_style!(initial).to_string(), "outline-style:initial;");
	assert_eq!(css::outline_style!(inherit).to_string(), "outline-style:inherit;");
	assert_eq!(css::outline_style!(unset).to_string(), "outline-style:unset;");

	assert_eq!(css::outline_width!(initial).to_string(), "outline-width:initial;");
	assert_eq!(css::outline_width!(inherit).to_string(), "outline-width:inherit;");
	assert_eq!(css::outline_width!(unset).to_string(), "outline-width:unset;");

	assert_eq!(css::overflow_wrap!(initial).to_string(), "overflow-wrap:initial;");
	assert_eq!(css::overflow_wrap!(inherit).to_string(), "overflow-wrap:inherit;");
	assert_eq!(css::overflow_wrap!(unset).to_string(), "overflow-wrap:unset;");

	assert_eq!(css::overflow_x!(initial).to_string(), "overflow-x:initial;");
	assert_eq!(css::overflow_x!(inherit).to_string(), "overflow-x:inherit;");
	assert_eq!(css::overflow_x!(unset).to_string(), "overflow-x:unset;");

	assert_eq!(css::overflow_y!(initial).to_string(), "overflow-y:initial;");
	assert_eq!(css::overflow_y!(inherit).to_string(), "overflow-y:inherit;");
	assert_eq!(css::overflow_y!(unset).to_string(), "overflow-y:unset;");

	assert_eq!(css::padding_bottom!(initial).to_string(), "padding-bottom:initial;");
	assert_eq!(css::padding_bottom!(inherit).to_string(), "padding-bottom:inherit;");
	assert_eq!(css::padding_bottom!(unset).to_string(), "padding-bottom:unset;");

	assert_eq!(css::padding_left!(initial).to_string(), "padding-left:initial;");
	assert_eq!(css::padding_left!(inherit).to_string(), "padding-left:inherit;");
	assert_eq!(css::padding_left!(unset).to_string(), "padding-left:unset;");

	assert_eq!(css::padding_right!(initial).to_string(), "padding-right:initial;");
	assert_eq!(css::padding_right!(inherit).to_string(), "padding-right:inherit;");
	assert_eq!(css::padding_right!(unset).to_string(), "padding-right:unset;");

	assert_eq!(css::padding_top!(initial).to_string(), "padding-top:initial;");
	assert_eq!(css::padding_top!(inherit).to_string(), "padding-top:inherit;");
	assert_eq!(css::padding_top!(unset).to_string(), "padding-top:unset;");

	assert_eq!(css::perspective!(initial).to_string(), "perspective:initial;");
	assert_eq!(css::perspective!(inherit).to_string(), "perspective:inherit;");
	assert_eq!(css::perspective!(unset).to_string(), "perspective:unset;");

	assert_eq!(css::pointer_events!(initial).to_string(), "pointer-events:initial;");
	assert_eq!(css::pointer_events!(inherit).to_string(), "pointer-events:inherit;");
	assert_eq!(css::pointer_events!(unset).to_string(), "pointer-events:unset;");

	assert_eq!(css::position!(initial).to_string(), "position:initial;");
	assert_eq!(css::position!(inherit).to_string(), "position:inherit;");
	assert_eq!(css::position!(unset).to_string(), "position:unset;");

	assert_eq!(css::resize!(initial).to_string(), "resize:initial;");
	assert_eq!(css::resize!(inherit).to_string(), "resize:inherit;");
	assert_eq!(css::resize!(unset).to_string(), "resize:unset;");

	assert_eq!(css::right!(initial).to_string(), "right:initial;");
	assert_eq!(css::right!(inherit).to_string(), "right:inherit;");
	assert_eq!(css::right!(unset).to_string(), "right:unset;");

	assert_eq!(css::row_gap!(initial).to_string(), "row-gap:initial;");
	assert_eq!(css::row_gap!(inherit).to_string(), "row-gap:inherit;");
	assert_eq!(css::row_gap!(unset).to_string(), "row-gap:unset;");

	assert_eq!(css::scroll_behavior!(initial).to_string(), "scroll-behavior:initial;");
	assert_eq!(css::scroll_behavior!(inherit).to_string(), "scroll-behavior:inherit;");
	assert_eq!(css::scroll_behavior!(unset).to_string(), "scroll-behavior:unset;");

	assert_eq!(css::stroke!(initial).to_string(), "stroke:initial;");
	assert_eq!(css::stroke!(inherit).to_string(), "stroke:inherit;");
	assert_eq!(css::stroke!(unset).to_string(), "stroke:unset;");

	assert_eq!(css::stroke_width!(initial).to_string(), "stroke-width:initial;");
	assert_eq!(css::stroke_width!(inherit).to_string(), "stroke-width:inherit;");
	assert_eq!(css::stroke_width!(unset).to_string(), "stroke-width:unset;");

	assert_eq!(css::tab_size!(initial).to_string(), "tab-size:initial;");
	assert_eq!(css::tab_size!(inherit).to_string(), "tab-size:inherit;");
	assert_eq!(css::tab_size!(unset).to_string(), "tab-size:unset;");

	assert_eq!(css::table_layout!(initial).to_string(), "table-layout:initial;");
	assert_eq!(css::table_layout!(inherit).to_string(), "table-layout:inherit;");
	assert_eq!(css::table_layout!(unset).to_string(), "table-layout:unset;");

	assert_eq!(css::text_align!(initial).to_string(), "text-align:initial;");
	assert_eq!(css::text_align!(inherit).to_string(), "text-align:inherit;");
	assert_eq!(css::text_align!(unset).to_string(), "text-align:unset;");

	assert_eq!(css::text_align_last!(initial).to_string(), "text-align-last:initial;");
	assert_eq!(css::text_align_last!(inherit).to_string(), "text-align-last:inherit;");
	assert_eq!(css::text_align_last!(unset).to_string(), "text-align-last:unset;");

	assert_eq!(css::text_decoration_color!(initial).to_string(), "text-decoration-color:initial;");
	assert_eq!(css::text_decoration_color!(inherit).to_string(), "text-decoration-color:inherit;");
	assert_eq!(css::text_decoration_color!(unset).to_string(), "text-decoration-color:unset;");

	assert_eq!(css::text_decoration_line!(initial).to_string(), "text-decoration-line:initial;");
	assert_eq!(css::text_decoration_line!(inherit).to_string(), "text-decoration-line:inherit;");
	assert_eq!(css::text_decoration_line!(unset).to_string(), "text-decoration-line:unset;");

	assert_eq!(css::text_decoration_style!(initial).to_string(), "text-decoration-style:initial;");
	assert_eq!(css::text_decoration_style!(inherit).to_string(), "text-decoration-style:inherit;");
	assert_eq!(css::text_decoration_style!(unset).to_string(), "text-decoration-style:unset;");

	assert_eq!(css::text_indent!(initial).to_string(), "text-indent:initial;");
	assert_eq!(css::text_indent!(inherit).to_string(), "text-indent:inherit;");
	assert_eq!(css::text_indent!(unset).to_string(), "text-indent:unset;");

	assert_eq!(css::text_justify!(initial).to_string(), "text-justify:initial;");
	assert_eq!(css::text_justify!(inherit).to_string(), "text-justify:inherit;");
	assert_eq!(css::text_justify!(unset).to_string(), "text-justify:unset;");

	assert_eq!(css::text_overflow!(initial).to_string(), "text-overflow:initial;");
	assert_eq!(css::text_overflow!(inherit).to_string(), "text-overflow:inherit;");
	assert_eq!(css::text_overflow!(unset).to_string(), "text-overflow:unset;");

	assert_eq!(css::text_rendering!(initial).to_string(), "text-rendering:initial;");
	assert_eq!(css::text_rendering!(inherit).to_string(), "text-rendering:inherit;");
	assert_eq!(css::text_rendering!(unset).to_string(), "text-rendering:unset;");

	assert_eq!(css::text_transform!(initial).to_string(), "text-transform:initial;");
	assert_eq!(css::text_transform!(inherit).to_string(), "text-transform:inherit;");
	assert_eq!(css::text_transform!(unset).to_string(), "text-transform:unset;");

	assert_eq!(css::top!(initial).to_string(), "top:initial;");
	assert_eq!(css::top!(inherit).to_string(), "top:inherit;");
	assert_eq!(css::top!(unset).to_string(), "top:unset;");

	assert_eq!(css::transform_style!(initial).to_string(), "transform-style:initial;");
	assert_eq!(css::transform_style!(inherit).to_string(), "transform-style:inherit;");
	assert_eq!(css::transform_style!(unset).to_string(), "transform-style:unset;");

	assert_eq!(css::unicode_bidi!(initial).to_string(), "unicode-bidi:initial;");
	assert_eq!(css::unicode_bidi!(inherit).to_string(), "unicode-bidi:inherit;");
	assert_eq!(css::unicode_bidi!(unset).to_string(), "unicode-bidi:unset;");

	assert_eq!(css::user_select!(initial).to_string(), "user-select:initial;");
	assert_eq!(css::user_select!(inherit).to_string(), "user-select:inherit;");
	assert_eq!(css::user_select!(unset).to_string(), "user-select:unset;");

	assert_eq!(css::vector_effect!(initial).to_string(), "vector-effect:initial;");
	assert_eq!(css::vector_effect!(inherit).to_string(), "vector-effect:inherit;");
	assert_eq!(css::vector_effect!(unset).to_string(), "vector-effect:unset;");

	assert_eq!(css::vertical_align!(initial).to_string(), "vertical-align:initial;");
	assert_eq!(css::vertical_align!(inherit).to_string(), "vertical-align:inherit;");
	assert_eq!(css::vertical_align!(unset).to_string(), "vertical-align:unset;");

	assert_eq!(css::visibility!(initial).to_string(), "visibility:initial;");
	assert_eq!(css::visibility!(inherit).to_string(), "visibility:inherit;");
	assert_eq!(css::visibility!(unset).to_string(), "visibility:unset;");

	assert_eq!(css::white_space!(initial).to_string(), "white-space:initial;");
	assert_eq!(css::white_space!(inherit).to_string(), "white-space:inherit;");
	assert_eq!(css::white_space!(unset).to_string(), "white-space:unset;");

	assert_eq!(css::width!(initial).to_string(), "width:initial;");
	assert_eq!(css::width!(inherit).to_string(), "width:inherit;");
	assert_eq!(css::width!(unset).to_string(), "width:unset;");

	assert_eq!(css::word_break!(initial).to_string(), "word-break:initial;");
	assert_eq!(css::word_break!(inherit).to_string(), "word-break:inherit;");
	assert_eq!(css::word_break!(unset).to_string(), "word-break:unset;");

	assert_eq!(css::word_spacing!(initial).to_string(), "word-spacing:initial;");
	assert_eq!(css::word_spacing!(inherit).to_string(), "word-spacing:inherit;");
	assert_eq!(css::word_spacing!(unset).to_string(), "word-spacing:unset;");

	assert_eq!(css::word_wrap!(initial).to_string(), "word-wrap:initial;");
	assert_eq!(css::word_wrap!(inherit).to_string(), "word-wrap:inherit;");
	assert_eq!(css::word_wrap!(unset).to_string(), "word-wrap:unset;");

	assert_eq!(css::writing_mode!(initial).to_string(), "writing-mode:initial;");
	assert_eq!(css::writing_mode!(inherit).to_string(), "writing-mode:inherit;");
	assert_eq!(css::writing_mode!(unset).to_string(), "writing-mode:unset;");

	assert_eq!(css::z_index!(initial).to_string(), "z-index:initial;");
	assert_eq!(css::z_index!(inherit).to_string(), "z-index:inherit;");
	assert_eq!(css::z_index!(unset).to_string(), "z-index:unset;");
}
