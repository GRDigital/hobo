//# command-macros = { version = "*", features = ["nightly"] }

#![feature(proc_macro_hygiene)]
use command_macros::command as cmd;
use std::thread::sleep;
use std::time::Duration;

fn main() {
	assert!(cmd!(cargo publish).current_dir("crates/hobo_derive").status().unwrap().success());
	sleep(Duration::from_secs(60));
	assert!(cmd!(cargo publish).current_dir("crates/hobo_css_macros").status().unwrap().success());
	sleep(Duration::from_secs(60));
	assert!(cmd!(cargo publish).current_dir("crates/hobo_css").status().unwrap().success());
	sleep(Duration::from_secs(60));
	assert!(cmd!(cargo publish).current_dir("crates/hobo_core").status().unwrap().success());
	sleep(Duration::from_secs(60));
	assert!(cmd!(cargo publish).status().unwrap().success());
}
