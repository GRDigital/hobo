mod flexbox;

#[macro_export] macro_rules! margin_vertical { ($($tt:tt)+) => {
	vec![
		$crate::__margin!(Top, $($tt)+),
		$crate::__margin!(Bottom, $($tt)+),
	]
} }
#[macro_export] macro_rules! margin_horizontal { ($($tt:tt)+) => {
	vec![
		$crate::__margin!(Left, $($tt)+),
		$crate::__margin!(Right, $($tt)+),
	]
} }
#[macro_export] macro_rules! margin { ($($tt:tt)+) => {
	vec![
		$crate::__margin!(Left, $($tt)+),
		$crate::__margin!(Right, $($tt)+),
		$crate::__margin!(Top, $($tt)+),
		$crate::__margin!(Bottom, $($tt)+),
	]
} }

#[macro_export] macro_rules! padding_vertical { ($($tt:tt)+) => {
	vec![
		$crate::padding_top!($($tt)+),
		$crate::padding_bottom!($($tt)+),
	]
} }
#[macro_export] macro_rules! padding_horizontal { ($($tt:tt)+) => {
	vec![
		$crate::padding_left!($($tt)+),
		$crate::padding_right!($($tt)+),
	]
} }
#[macro_export] macro_rules! padding { ($($tt:tt)+) => {
	vec![
		$crate::padding_left!($($tt)+),
		$crate::padding_right!($($tt)+),
		$crate::padding_top!($($tt)+),
		$crate::padding_bottom!($($tt)+),
	]
} }

#[macro_export]
macro_rules! overflow {
	($($tt:tt)+) => {vec![
		$crate::overflow_x!($($tt)+),
		$crate::overflow_y!($($tt)+),
	]}
}

#[macro_export]
macro_rules! size {
	($($tt:tt)+) => {vec![
		$crate::width!($($tt)+),
		$crate::height!($($tt)+),
	]}
}

#[macro_export]
macro_rules! gap {
	($($tt:tt)+) => {vec![
		$crate::row_gap!($($tt)+),
		$crate::column_gap!($($tt)+),
	]}
}
