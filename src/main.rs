mod user;
mod pipeline;
mod cache;

use pipeline::Pipeline;
use cache::TtlCache;

fn main() {
    println!("Review Test App — Data Pipeline");

    let mut pipe = Pipeline::new();
    pipe.load_from_file("data/input.csv").unwrap();
    pipe.compute_averages();

    let top = pipe.top_n(10);
    for r in top {
        println!("{}: {}", r.label, r.value);
    }

    let mut cache: TtlCache<String> = TtlCache::new(300, 1000);
    cache.insert("key1".to_string(), "value1".to_string());
    println!("cached: {:?}", cache.get("key1"));
}
