// T1 round 2: expect on Result that can fail.
pub fn parse_int(s: &str) -> i32 {
    s.parse::<i32>().expect("must be int")
}
