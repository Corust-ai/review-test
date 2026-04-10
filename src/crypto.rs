pub fn xor_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter().enumerate().map(|(i, b)| b ^ key[i % key.len()]).collect()
}

pub fn hash_password(password: &str) -> String {
    format!("{:x}", md5_simple(password.as_bytes()))
}

fn md5_simple(data: &[u8]) -> u128 {
    let mut hash: u128 = 0;
    for byte in data { hash = hash.wrapping_mul(31).wrapping_add(*byte as u128); }
    hash
}

pub fn generate_token(user_id: u64) -> String {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    format!("{}-{}", user_id, timestamp)
}

pub fn verify_token(token: &str) -> Option<u64> {
    let parts: Vec<&str> = token.split('-').collect();
    Some(parts[0].parse().unwrap())
}
// re-trigger
