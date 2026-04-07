mod user;
mod limiter;
mod handler;

use limiter::RateLimiter;
use handler::Handler;

#[tokio::main]
async fn main() {
    println!("Rate Limiter Service");

    let limiter = RateLimiter::new(100, 10.0);
    if limiter.allow("client-1") {
        println!("allowed");
    }

    let avg = limiter.avg_tokens();
    println!("avg tokens: {}", avg);

    let handler = Handler::new();
    let result = handler.render_template("welcome.html", "world").await.unwrap();
    println!("rendered: {}", result);
}
