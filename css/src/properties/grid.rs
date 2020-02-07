use crate::prelude::*;

css_macros::easy_enum! {row-gap normal @}
css_macros::easy_enum! {column-gap normal @}
css_macros::easy_enum! {grid-column-start auto #}
css_macros::easy_enum! {grid-column-end auto #}
css_macros::easy_enum! {grid-row-start auto #}
css_macros::easy_enum! {grid-row-end auto #}

/*
css::grid!(
	column,
	// [*-gap] | [grid-template-*] >> [grid-auto-*]
	columns (32 px) | (1 fr) (50 px) repeat(5, (1 fr)) >> 1 fr,
	rows >> 1 fr,
)
*/

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GridAutoFlow {
	Inherit,
	Initial,
	Unset,
	Revert,
	Row,
	Column,
	RowDense,
	ColumnDense,
}

#[rustfmt::skip]
impl ToString for GridAutoFlow {
	fn to_string(&self) -> String {
		match self {
			Self::Inherit     => "grid-auto-flow:inherit;".to_owned(),
			Self::Initial     => "grid-auto-flow:initial;".to_owned(),
			Self::Unset       => "grid-auto-flow:unset;".to_owned(),
			Self::Revert      => "grid-auto-flow:revert;".to_owned(),
			Self::Row         => "grid-auto-flow:row;".to_owned(),
			Self::Column      => "grid-auto-flow:column;".to_owned(),
			Self::RowDense    => "grid-auto-flow:row dense;".to_owned(),
			Self::ColumnDense => "grid-auto-flow:column dense;".to_owned(),
		}
	}
}

#[macro_export]
macro_rules! grid_auto_flow {
	(inherit)      => {$crate::Property::GridAutoFlow($crate::GridAutoFlow::Inherit)};
	(initial)      => {$crate::Property::GridAutoFlow($crate::GridAutoFlow::Initial)};
	(unset)        => {$crate::Property::GridAutoFlow($crate::GridAutoFlow::Unset)};
	(revert)       => {$crate::Property::GridAutoFlow($crate::GridAutoFlow::Revert)};
	(row)          => {$crate::Property::GridAutoFlow($crate::GridAutoFlow::Row)};
	(column)       => {$crate::Property::GridAutoFlow($crate::GridAutoFlow::Column)};
	(row dense)    => {$crate::Property::GridAutoFlow($crate::GridAutoFlow::RowDense)};
	(column dense) => {$crate::Property::GridAutoFlow($crate::GridAutoFlow::ColumnDense)};
}

// what follows is not a great bunch of structs and macros, but it covers basic grid usage
// the actual grid syntax is absolutely bananas and nothing short of a full grammar parser would suffice

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GridRepeatNumber {
	AutoFit,
	AutoFill,
	Some(u32),
}

#[rustfmt::skip]
impl ToString for GridRepeatNumber {
	fn to_string(&self) -> String {
		match self {
			Self::AutoFit  => "auto-fit".to_owned(),
			Self::AutoFill => "auto-fill".to_owned(),
			Self::Some(x)  => x.to_string(),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GridRepeat {
	pub number: GridRepeatNumber,
	pub values: Vec<Unit>,
}

#[rustfmt::skip]
impl ToString for GridRepeat {
	fn to_string(&self) -> String {
		format!("repeat({}, {})", self.number.to_string(), self.values.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(" "))
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum GridTemplate {
	Inherit,
	Initial,
	Unset,
	Revert,
	None,
	Some(Vec<GridTemplateValue>),
}

#[rustfmt::skip]
impl ToString for GridTemplate {
	fn to_string(&self) -> String {
		match self {
			Self::Inherit     => "inherit".to_owned(),
			Self::Initial     => "initial".to_owned(),
			Self::Unset       => "unset".to_owned(),
			Self::Revert      => "revert".to_owned(),
			Self::None        => "none".to_owned(),
			Self::Some(x)     => x.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(" "),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum GridTemplateValue {
	Repeat(GridRepeat),
	Unit(Unit),
}

#[rustfmt::skip]
impl ToString for GridTemplateValue {
	fn to_string(&self) -> String {
		match self {
			Self::Unit(x) => x.to_string(),
			Self::Repeat(x) => x.to_string(),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum GridAuto {
	Inherit,
	Initial,
	Unset,
	Revert,
	Auto,
	Some(Vec<Unit>),
}

#[rustfmt::skip]
impl ToString for GridAuto {
	fn to_string(&self) -> String {
		match self {
			Self::Inherit => "inherit".to_owned(),
			Self::Initial => "initial".to_owned(),
			Self::Unset   => "unset".to_owned(),
			Self::Revert  => "revert".to_owned(),
			Self::Auto    => "auto".to_owned(),
			Self::Some(x) => x.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(" "),
		}
	}
}

#[macro_export]
macro_rules! __repeat_number {
	(auto-fit) => {$crate::GridRepeatNumber::AutoFit};
	(auto-fill) => {$crate::GridRepeatNumber::AutoFill};
	($x:expr) => {$crate::GridRepeatNumber::Some($x)};
}

#[macro_export]
macro_rules! __nested_unit {
	(($($tt:tt)+)) => {$crate::unit!($($tt)+)};
}

#[macro_export]
macro_rules! __repeat {
	($($repeat_number:tt)-+, $($v:tt)+) => {
		$crate::GridRepeat {
			number: $crate::__repeat_number!($($repeat_number)-+),
			values: vec![$($crate::__nested_unit!($v)),+],
		}
	}
}

#[macro_export]
macro_rules! __grid_template_value {
	(repeat($($tt:tt)+)) => {$crate::GridTemplateValue::Repeat($crate::__repeat!($($tt)+))};
	(($($tt:tt)+)) => {$crate::GridTemplateValue::Unit($crate::unit!($($tt)+))};
}

#[macro_export]
macro_rules! __grid_template {
	(
		kind = $kind:ident,
		acc = $acc:expr,
		current = ($($current:tt)+),
		rest = ($(,)*),
	) => {{
		$acc.push($crate::__grid_template_value!($($current)+));
		let acc = $acc;
		$crate::paste::item!{$crate::Property::[<GridTemplate $kind>]($crate::GridTemplate::Some(acc))}
	}};
	(
		kind = $kind:ident,
		acc = $acc:expr,
		current = ($($current:tt)+),
		rest = (, $($rest:tt)*),
	) => {{
		$acc.push($crate::__grid_template_value!($($current)+));
		$crate::__grid_template!(
			kind = $kind,
			acc = $acc,
			current = (),
			rest = ($($rest)*),
		)
	}};
	(
		kind = $kind:ident,
		acc = $acc:expr,
		current = ($($current:tt)*),
		rest = ($token:tt $($rest:tt)*),
	) => {
		$crate::__grid_template!(
			kind = $kind,
			acc = $acc,
			current = ($($current)* $token),
			rest = ($($rest)*),
		)
	};
}

#[macro_export]
macro_rules! grid_template_columns {
	($($tt:tt)+) => {{
		let mut acc = Vec::new();
		$crate::__grid_template!(
			kind = Columns,
			acc = acc,
			current = (),
			rest = ($($tt)+),
		)
	}};
}

#[macro_export]
macro_rules! grid_template_rows {
	($($tt:tt)+) => {{
		let mut acc = Vec::new();
		$crate::__grid_template!(
			kind = Rows,
			acc = acc,
			current = (),
			rest = ($($tt)+),
		)
	}};
}
