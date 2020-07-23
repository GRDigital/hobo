use crate::prelude::*;

css_macros::easy_enum! {vector-effect none non-scaling-stroke non-scaling-size non-rotation fixed-position}
css_macros::easy_color! {fill}
css_macros::easy_color! {stroke}
css_macros::unit_value_macro! {stroke_width StrokeWidth}

#[test]
fn svg_test() {
	assert_eq!(vector_effect!(non-scaling-stroke), crate::Property::VectorEffect(crate::VectorEffect::NonScalingStroke));
}
