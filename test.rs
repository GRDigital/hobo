//# command-macros = { version = "*", features = ["nightly"] }

#![feature(proc_macro_hygiene)]
use command_macros::command as cmd;

fn main() {
	assert!(cmd!(cargo test).status().unwrap().success());
	assert!(cmd!(cargo test -p hobo_css).status().unwrap().success());
	assert!(cmd!(wasm-pack test --headless --chrome --firefox).status().unwrap().success());
}
