//# heck = "*"

use heck::*;

fn main() {
	let property_name = std::env::args().nth(1).unwrap();
	let mut options = std::env::args().skip(2).collect::<Vec<String>>();
	options.push("initial".to_owned());
	options.push("inherit".to_owned());
	// dbg!(property_name.to_snek_case());
	// dbg!(options);

	let enum_members: String = options.iter().map(|value| format!(
		"    #[strum(to_string = \"{property}:{value};\")] {camel_value},",
		property = property_name,
		value = value,
		camel_value = value.to_camel_case(),
	)).collect::<Vec<_>>().join("\n");

	let macro_values: String = options.iter().map(|value| format!(
		"    ({value}) => {{ $crate::Property::{camel_property}($crate::{camel_property}::{camel_value}) }};",
		value = value,
		camel_property = property_name.to_camel_case(),
		camel_value = value.to_camel_case(),
	)).collect::<Vec<_>>().join("\n");

	println!(r#"
#[derive(Debug, PartialEq, Eq, Hash, smart_default::SmartDefault, Clone, Copy, strum_macros::Display)]
pub enum {camel_property} {{
    #[default]
{enum_members}
}}

#[macro_export]
macro_rules! {snek_property} {{
{macro_values}
}}"#,
	camel_property = property_name.to_camel_case(),
	snek_property = property_name.to_snek_case(),
	enum_members = enum_members,
	macro_values = macro_values,
);
}
