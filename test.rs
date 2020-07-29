//# command-macros = { version = "*", features = ["nightly"] }

#![feature(proc_macro_hygiene)]
use command_macros::command as cmd;

fn main() {
	assert!(cmd!(cargo test).status().unwrap().success());
	assert!(cmd!(wasm-pack test --headless --chrome --firefox).current_dir("lib").status().unwrap().success());
}
