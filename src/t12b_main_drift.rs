// T12B main drift — added to base AFTER PR opened.
// Should NEVER appear in PR's review (not part of base..head diff).
pub fn drift_function() {
    let unused_var: i32 = 999;
    println!("drift");
}
