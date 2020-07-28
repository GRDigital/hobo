//# command-macros = { version = "*", features = ["nightly"] }
//# anyhow = "*"

#![feature(proc_macro_hygiene)]
use command_macros::command as cmd;

use std::fs::File;

fn main() -> anyhow::Result<()> {
	// let main_toml: toml::Value = toml::from_slice(&std::fs::read("Cargo.toml")?)?;
	// assert_eq!(toml::ser::to_string(&main_toml)?, std::fs::read_to_string("Cargo.toml")?);
	assert!(cmd!(cargo release --dry-run).current_dir("derive").status()?.success());
	// assert!(cmd!(cargo test -p hobo_css).status()?.success());
	// assert!(cmd!(wasm-pack test --headless --chrome --firefox).status()?.success());
	Ok(())
}
