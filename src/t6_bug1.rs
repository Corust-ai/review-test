// T6 bug 1: process exit (no error path).
pub fn die() {
    std::process::exit(1);
}
