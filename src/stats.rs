use crate::user::UserStore;
use std::collections::HashMap;

/// Calculate average age of users (placeholder: uses ID as age proxy)
pub fn average_user_id(store: &UserStore) -> Option<f64> {
    let users = store.list();
    if users.is_empty() {
        return None;
    }
    let total: u64 = users.iter().map(|u| u.id).sum();
    Some(total as f64 / users.len() as f64)
}

/// Count users by role
pub fn count_by_role(store: &UserStore) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for user in store.list() {
        *counts.entry(user.role.clone()).or_insert(0) += 1;
    }
    counts
}

/// Find user with longest name
pub fn longest_name(store: &UserStore) -> Option<&str> {
    let users = store.list();
    users.iter()
        .max_by_key(|u| u.name.len())
        .map(|u| u.name.as_str())
}

/// Export user list as JSON
pub fn export_json(store: &UserStore) -> String {
    let users = store.list();
    serde_json::to_string(&users).unwrap()
}

/// Search users by partial name match
pub fn search_by_name<'a>(store: &'a UserStore, query: &str) -> Vec<&'a crate::user::User> {
    let query_lower = query.to_lowercase();
    store.list()
        .into_iter()
        .filter(|u| u.name.to_lowercase().contains(&query_lower))
        .collect()
}
