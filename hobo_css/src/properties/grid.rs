use crate::prelude::*;

crate::macros::easy_enum! {row-gap normal [unit]}
crate::macros::easy_enum! {column-gap normal [unit]}
crate::macros::easy_enum! {grid-gap normal [unit]}
crate::macros::easy_join! {gap, (row_gap, column_gap), (normal, [unit])}

/*
css::grid!(
	column,
	// [*-gap] | [grid-template-*] >> [grid-auto-*]
	columns (32 px) | (1 fr) (50 px) repeat(5, (1 fr)) >> 1 fr,
	rows >> 1 fr,
)
*/

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
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
			Self::Span(x) => write!(f, "span {x}"),
			Self::Absolute(x) => x.fmt(f),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum grid_auto_flow {
	inherit,
	initial,
	unset,
	row,
	column,
	row_dense,
	column_dense,
}

#[rustfmt::skip]
impl std::fmt::Display for grid_auto_flow {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::inherit     => "grid-auto-flow:inherit;".fmt(f),
			Self::initial     => "grid-auto-flow:initial;".fmt(f),
			Self::unset       => "grid-auto-flow:unset;".fmt(f),
			Self::row         => "grid-auto-flow:row;".fmt(f),
			Self::column      => "grid-auto-flow:column;".fmt(f),
			Self::row_dense    => "grid-auto-flow:row dense;".fmt(f),
			Self::column_dense => "grid-auto-flow:column dense;".fmt(f),
		}
	}
}

// what follows is not a great bunch of structs and macros, but it covers basic grid usage
// the actual grid syntax is absolutely bananas and nothing short of a full grammar parser would suffice

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct GridRepeat {
	pub number: GridRepeatNumber,
	pub values: Vec<Unit>,
}

#[rustfmt::skip]
impl std::fmt::Display for GridRepeat {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "repeat({},", self.number)?;
		for value in &self.values {
			write!(f, " {value}")?;
		}
		")".fmt(f)
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
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
					write!(f, "{first}")?;
					for value in rest {
						write!(f, " {value}")?;
					}
				}
				Ok(())
			},
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
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
					write!(f, "{first}")?;
					for unit in rest {
						write!(f, " {unit}")?;
					}
				}
				Ok(())
			},
		}
	}
}
