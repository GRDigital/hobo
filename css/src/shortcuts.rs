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
		$crate::__padding!(Top, $($tt)+),
		$crate::__padding!(Bottom, $($tt)+),
	]
} }
#[macro_export] macro_rules! padding_horizontal { ($($tt:tt)+) => {
	vec![
		$crate::__padding!(Left, $($tt)+),
		$crate::__padding!(Right, $($tt)+),
	]
} }
#[macro_export] macro_rules! padding { ($($tt:tt)+) => {
	vec![
		$crate::__padding!(Left, $($tt)+),
		$crate::__padding!(Right, $($tt)+),
		$crate::__padding!(Top, $($tt)+),
		$crate::__padding!(Bottom, $($tt)+),
	]
} }

#[macro_export]
macro_rules! overflow {
	($($tt:tt)+) => {vec![
		$crate::overflow_x!($($tt)+),
		$crate::overflow_y!($($tt)+),
	]}
}
