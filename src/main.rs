mod user;
mod auth;
mod stats;

fn main() {
    let mut store = user::UserStore::new();
    store.add("Alice".into(), "alice@example.com".into(), "admin".into());
    store.add("Bob".into(), "bob@example.com".into(), "user".into());

    // Auth
    let mut auth = auth::AuthService::new("my-secret-key-123".to_string());
    let token = auth.login(1, "password123", "admin");
    println!("Token: {}", token);

    match auth.verify(&token) {
        Ok(uid) => println!("Verified user: {}", uid),
        Err(e) => println!("Auth error: {}", e),
    }

    // Stats
    println!("Average ID: {}", stats::average_user_id(&store));
    println!("By role: {:?}", stats::count_by_role(&store));
    println!("Longest name: {}", stats::longest_name(&store));
    println!("JSON: {}", stats::export_json(&store));
    println!("Search 'ali': {:?}", stats::search_by_name(&store, "ali"));
}
