css_macros::easy_enum!{background-repeat repeat repeat-x repeat-y no-repeat}
css_macros::easy_enum!{background-attachment scroll fixed local}

#[macro_export]
macro_rules! background_color {
	($r:tt $g:tt $b:tt $a:tt) => {$crate::Property::BackgroundColor($crate::Color::Rgba($r, $g, $b, $a))};
	($r:tt $g:tt $b:tt)       => {$crate::Property::BackgroundColor($crate::Color::Rgba($r, $g, $b, 255))};
	(initial)                 => {$crate::Property::BackgroundColor($crate::Color::Initial)};
	(inherit)                 => {$crate::Property::BackgroundColor($crate::Color::Inherit)};
}

