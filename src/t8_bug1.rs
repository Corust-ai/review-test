// T8 bug 1: channel send unwrap (panics on disconnect).
use std::sync::mpsc::Sender;
pub fn send_or_die(tx: Sender<i32>) {
    tx.send(0).unwrap();
}
