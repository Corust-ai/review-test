// Bug 03: unchecked index access — panics when slice is empty.
pub fn first_element(values: &[i32]) -> i32 {
    values[0]
}
