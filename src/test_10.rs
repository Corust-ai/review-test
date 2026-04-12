pub fn risky_10(s: &str) -> String {
    let n: usize = s.parse().unwrap();
    let v = vec!["a","b","c"];
    format!("{}{}", v[n], s.len() / (n - n))
}
