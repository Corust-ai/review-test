// T14 drift A: process::exit hides destructors. NOT PR author.
pub fn shutdown() {
    std::process::exit(0);
}
