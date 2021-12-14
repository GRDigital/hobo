//# regex = "*"

use regex::Regex;

fn main() {
	let new_version = std::env::args().skip(1).next().unwrap();
	for manifest in &["hobo_derive/Cargo.toml", "hobo_css/Cargo.toml", "hobo_css_macros/Cargo.toml", "hobo/Cargo.toml"] {
		let mut x = std::fs::read_to_string(manifest).unwrap();
		x = Regex::new(r#"\nversion = ".+""#          ).unwrap().replace(    &x, &format!("\nversion = \"{}\"",                                                         new_version) as &str).to_string();
		x = Regex::new(r#"\nhobo_derive = \{.+\}"#    ).unwrap().replace_all(&x, &format!("\nhobo_derive = {{ path = \"../hobo_derive\", version = \"={}\" }}",         new_version) as &str).to_string();
		x = Regex::new(r#"\nhobo_css = \{.+\}"#       ).unwrap().replace_all(&x, &format!("\nhobo_css = {{ path = \"../hobo_css\", version = \"={}\" }}",               new_version) as &str).to_string();
		x = Regex::new(r#"\nhobo_css_macros = \{.+\}"#).unwrap().replace_all(&x, &format!("\nhobo_css_macros = {{ path = \"../hobo_css_macros\", version = \"={}\" }}", new_version) as &str).to_string();
		std::fs::write(manifest, x).unwrap();
	}
}
