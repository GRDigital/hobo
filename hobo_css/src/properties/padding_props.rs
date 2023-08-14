crate::macros::unit_value_macro! {padding_left}
crate::macros::unit_value_macro! {padding_right}
crate::macros::unit_value_macro! {padding_top}
crate::macros::unit_value_macro! {padding_bottom}
crate::macros::easy_join!(padding_horizontal, (padding_left, padding_right), ([unit]));
crate::macros::easy_join!(padding_vertical, (padding_top, padding_bottom), ([unit]));
crate::macros::easy_join!(padding, (padding_horizontal, padding_vertical), ([unit]));
