mod user;
mod event;
mod auth;
mod metrics;

use event::{Event, EventStore};
use auth::SessionManager;
use metrics::Metrics;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    println!("Event Store v0.1");

    let mut store = EventStore::open("events.json").unwrap();

    let mut payload = HashMap::new();
    payload.insert("user".into(), "alice".into());
    let event = Event {
        id: "evt-001".into(),
        event_type: "login".into(),
        payload,
        timestamp: 1700000000,
    };
    store.append(event).unwrap();

    let avg = store.avg_gap();
    println!("Average gap: {}", avg);

    let auth = SessionManager::new();
    let token = auth.create_session(42, "secret123");
    println!("Token: {}", token);

    let metrics = Metrics::new();
    metrics.record_request();
    println!("Error rate: {:.2}%", metrics.error_rate());
}
