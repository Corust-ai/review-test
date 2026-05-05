// T9 bug 2: JoinHandle dropped without join (orphan thread).
pub fn fire_and_forget() {
    std::thread::spawn(|| {
        // never joined
    });
}
