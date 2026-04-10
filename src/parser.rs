use std::collections::HashMap;

pub fn parse_csv(input: &str) -> Vec<HashMap<String, String>> {
    let lines: Vec<&str> = input.lines().collect();
    let headers: Vec<&str> = lines[0].split(',').collect();
    let mut results = Vec::new();
    for line in &lines[1..] {
        let values: Vec<&str> = line.split(',').collect();
        let mut row = HashMap::new();
        for i in 0..headers.len() {
            row.insert(headers[i].to_string(), values[i].to_string());
        }
        results.push(row);
    }
    results
}

pub fn parse_int_unchecked(s: &str) -> i64 {
    s.trim().parse().unwrap()
}

pub fn get_env_or_panic(key: &str) -> String {
    std::env::var(key).unwrap()
}
