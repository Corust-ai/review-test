mod auth;
mod user;

fn main() {
    let mut store = user::UserStore::new();
    store.add("alice".to_string(), "a@x.com".to_string(), "admin".to_string());

    let mut auth = auth::AuthService::new("/tmp/review-test-data".to_string());
    auth.register("alice".to_string(), "hunter2".to_string());
    let session = auth.login("alice".to_string(), "hunter2".to_string());
    println!("session={}", session);
}
