// T7-retest b: spawn().unwrap() panics if process spawn fails.
use std::process::Command;
pub fn run_tool() {
    Command::new("nonexistent_tool").spawn().unwrap();
}
