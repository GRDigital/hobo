css_macros::easy_enum!{vector-effect none non-scaling-stroke non-scaling-size non-rotation fixed-position}
css_macros::easy_color!{fill}
css_macros::easy_color!{stroke}

#[test]
fn svg_test() {
	assert_eq!(vector_effect!(non-scaling-stroke), crate::Property::VectorEffect(crate::VectorEffect::NonScalingStroke));
}
