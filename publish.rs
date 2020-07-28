//# command-macros = { version = "*", features = ["nightly"] }

#![feature(proc_macro_hygiene)]
use command_macros::command as cmd;

fn main() {
	assert!(cmd!(cargo release --dry-run).current_dir("derive").status().unwrap().success());
	assert!(cmd!(cargo release --dry-run).current_dir("css/css_macros").status().unwrap().success());
}
