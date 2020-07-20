//# command-macros = { version = "*", features = ["nightly"] }

#![feature(proc_macro_hygiene)]
use command_macros::command as cmd;

fn main() {
	cmd!(cargo test).status().unwrap();
	cmd!(cargo test -p hobo-css).status().unwrap();
	cmd!(wasm-pack test --headless --chrome --firefox).status().unwrap();
}
