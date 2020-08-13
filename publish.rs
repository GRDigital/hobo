//# command-macros = { version = "*", features = ["nightly"] }

#![feature(proc_macro_hygiene)]
use command_macros::command as cmd;
use std::thread::sleep;
use std::time::Duration;

fn main() {
	assert!(cmd!(cargo publish).current_dir("derive").status().unwrap().success());
	assert!(cmd!(cargo publish).current_dir("css/macros").status().unwrap().success());
	sleep(Duration::from_secs(60));
	assert!(cmd!(cargo publish).current_dir("css").status().unwrap().success());
	sleep(Duration::from_secs(60));
	assert!(cmd!(cargo publish).current_dir("lib").status().unwrap().success());
}
