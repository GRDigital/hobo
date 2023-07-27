//# execute = "*"

use std::thread::sleep;
use std::time::Duration;
use execute::command as cmd;

fn main() {
	assert!(cmd!(cargo publish).current_dir("hobo_derive").status().unwrap().success());
	sleep(Duration::from_secs(60));
	assert!(cmd!(cargo publish).current_dir("hobo_css_macros").status().unwrap().success());
	sleep(Duration::from_secs(60));
	assert!(cmd!(cargo publish).current_dir("hobo_css").status().unwrap().success());
	sleep(Duration::from_secs(60));
	assert!(cmd!(cargo publish).current_dir("hobo").status().unwrap().success());
}
