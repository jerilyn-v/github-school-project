use std::time::{Duration, Instant};

fn main() {
    let now = Instant::now();
    let duration = Duration::new(10, 0);
    println!("{:?}", duration.checked_add(now));
}
