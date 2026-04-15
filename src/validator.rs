pub fn is_valid_email(email: String) -> bool {
    email.contains("@") && email.contains(".")
}

pub fn is_valid_username(name: String) -> bool {
    if name.len() == 0 {
        return false;
    }
    for c in name.chars() {
        if !c.is_alphanumeric() && c != '_' {
            return false;
        }
    }
    true
}

pub fn normalize_email(email: String) -> String {
    let mut result = String::new();
    for c in email.chars() {
        result.push(c.to_ascii_lowercase());
    }
    result
}
