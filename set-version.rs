//# regex = "*"

use regex::Regex;

fn main() {
	let new_version = std::env::args().skip(1).next().unwrap();
	for manifest in &["Cargo.toml", "crates/hobo_derive/Cargo.toml", "crates/hobo_css/Cargo.toml", "crates/hobo_css_macros/Cargo.toml", "crates/hobo_core/Cargo.toml"] {
		let mut x = std::fs::read_to_string(manifest).unwrap();
		x = Regex::new(r#"\nversion = "[0-9\.]+""#     ).unwrap().replace(    &x, &format!("\nversion = \"{}\"",                                                       new_version) as &str).to_string();
		x = Regex::new(r#"\nhobo_core = \{.+\}"#       ).unwrap().replace_all(&x, &format!("\nhobo_core = {{ path = \"crates/hobo_core\", version = \"={}\" }}",         new_version) as &str).to_string();
		x = Regex::new(r#"\nhobo_derive = \{.+\}"#     ).unwrap().replace_all(&x, &format!("\nhobo_derive = {{ path = \"../hobo_derive\", version = \"={}\" }}",         new_version) as &str).to_string();
		x = Regex::new(r#"\nhobo_css = \{.+\}"#        ).unwrap().replace_all(&x, &format!("\nhobo_css = {{ path = \"../hobo_css\", version = \"={}\" }}",               new_version) as &str).to_string();
		x = Regex::new(r#"\nhobo_css_macros = \{.+\}"# ).unwrap().replace_all(&x, &format!("\nhobo_css_macros = {{ path = \"../hobo_css_macros\", version = \"={}\" }}", new_version) as &str).to_string();
		std::fs::write(manifest, x).unwrap();
	}
}
