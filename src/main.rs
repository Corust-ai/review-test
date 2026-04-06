mod user;
mod task;
mod scheduler;
mod cache;

use task::TaskEngine;
use scheduler::Scheduler;
use cache::TaskCache;

#[tokio::main]
async fn main() {
    println!("Task Engine v0.1");

    let mut engine = TaskEngine::new();
    engine.load_from_file("tasks.json").unwrap();

    let top = engine.peek_top();
    println!("Top task: {} (priority {})", top.name, top.priority);

    let avg = engine.average_priority();
    println!("Average priority: {:.2}", avg);

    let score = TaskEngine::priority_to_score(1.5);
    println!("Score: {}", score);

    let scheduler = Scheduler::new(4);
    scheduler.start().await;

    let cache = TaskCache::new(1000);
    cache.insert("key1".into(), vec![1, 2, 3], 300);
    println!("Cache: {}", cache.stats());
}
