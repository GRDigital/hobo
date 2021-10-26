use crate::prelude::*;

crate::macros::easy_enum! {row-gap normal [unit]}
crate::macros::easy_enum! {column-gap normal [unit]}

/*
css::grid!(
	column,
	// [*-gap] | [grid-template-*] >> [grid-auto-*]
	columns (32 px) | (1 fr) (50 px) repeat(5, (1 fr)) >> 1 fr,
	rows >> 1 fr,
)
*/

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GridSpan {
	Auto,
	Initial,
	Inherit,
	Unset,
	Span(i32),
	Absolute(i32),
}

impl std::fmt::Display for GridSpan {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Auto => "auto".fmt(f),
			Self::Initial => "initial".fmt(f),
			Self::Inherit => "inherit".fmt(f),
			Self::Unset => "unset".fmt(f),
			Self::Span(x) => write!(f, "span {}", x),
			Self::Absolute(x) => x.fmt(f),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GridAutoFlow {
	Inherit,
	Initial,
	Unset,
	Row,
	Column,
	RowDense,
	ColumnDense,
}

#[rustfmt::skip]
impl std::fmt::Display for GridAutoFlow {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Inherit     => "grid-auto-flow:inherit;".fmt(f),
			Self::Initial     => "grid-auto-flow:initial;".fmt(f),
			Self::Unset       => "grid-auto-flow:unset;".fmt(f),
			Self::Row         => "grid-auto-flow:row;".fmt(f),
			Self::Column      => "grid-auto-flow:column;".fmt(f),
			Self::RowDense    => "grid-auto-flow:row dense;".fmt(f),
			Self::ColumnDense => "grid-auto-flow:column dense;".fmt(f),
		}
	}
}

#[macro_export]
macro_rules! grid_auto_flow {
	(inherit)      => {$crate::Property::GridAutoFlow($crate::GridAutoFlow::Inherit)};
	(initial)      => {$crate::Property::GridAutoFlow($crate::GridAutoFlow::Initial)};
	(unset)        => {$crate::Property::GridAutoFlow($crate::GridAutoFlow::Unset)};
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
impl std::fmt::Display for GridRepeatNumber {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::AutoFit  => "auto-fit".fmt(f),
			Self::AutoFill => "auto-fill".fmt(f),
			Self::Some(x)  => x.fmt(f),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GridRepeat {
	pub number: GridRepeatNumber,
	pub values: Vec<Unit>,
}

#[rustfmt::skip]
impl std::fmt::Display for GridRepeat {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "repeat({},", self.number)?;
		for value in &self.values {
			write!(f, " {}", value)?;
		}
		")".fmt(f)
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum GridTemplate {
	Inherit,
	Initial,
	Unset,
	None,
	Some(Vec<GridTemplateValue>),
}

#[rustfmt::skip]
impl std::fmt::Display for GridTemplate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Inherit     => "inherit".fmt(f),
			Self::Initial     => "initial".fmt(f),
			Self::Unset       => "unset".fmt(f),
			Self::None        => "none".fmt(f),
			Self::Some(values)     => {
				if let Some((first, rest)) = values.split_first() {
					write!(f, "{}", first)?;
					for value in rest {
						write!(f, " {}", value)?;
					}
				}
				Ok(())
			},
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum GridTemplateValue {
	Repeat(GridRepeat),
	Unit(Unit),
}

#[rustfmt::skip]
impl std::fmt::Display for GridTemplateValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Unit(x) => x.fmt(f),
			Self::Repeat(x) => x.fmt(f),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum GridAuto {
	Inherit,
	Initial,
	Unset,
	Auto,
	Some(Vec<Unit>),
}

#[rustfmt::skip]
impl std::fmt::Display for GridAuto {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Inherit => "inherit".fmt(f),
			Self::Initial => "initial".fmt(f),
			Self::Unset   => "unset".fmt(f),
			Self::Auto    => "auto".fmt(f),
			Self::Some(units) => {
				if let Some((first, rest)) = units.split_first() {
					write!(f, "{}", first)?;
					for unit in rest {
						write!(f, " {}", unit)?;
					}
				}
				Ok(())
			},
		}
	}
}

#[doc(hidden)]
#[macro_export]
macro_rules! __repeat_number {
	(auto-fit) => {$crate::GridRepeatNumber::AutoFit};
	(auto-fill) => {$crate::GridRepeatNumber::AutoFill};
	($x:expr) => {$crate::GridRepeatNumber::Some($x)};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __nested_unit {
	(($($tt:tt)+)) => {$crate::unit!($($tt)+)};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __repeat {
	($($repeat_number:tt)-+, $($v:tt)+) => {
		$crate::GridRepeat {
			number: $crate::__repeat_number!($($repeat_number)-+),
			values: vec![$($crate::__nested_unit!($v)),+],
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __grid_template_value {
	(repeat($($tt:tt)+)) => {$crate::GridTemplateValue::Repeat($crate::__repeat!($($tt)+))};
	(($($tt:tt)+)) => {$crate::GridTemplateValue::Unit($crate::unit!($($tt)+))};
}

#[doc(hidden)]
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
