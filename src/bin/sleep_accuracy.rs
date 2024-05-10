use std::time::{Duration, Instant};

fn accuracy(target: Duration) -> Duration {
    let now = Instant::now();
    std::thread::sleep(target);
    let elapsed = now.elapsed();
    elapsed.checked_sub(target).unwrap_or_else(|| target - elapsed)
}

fn main() {
    for _ in 0..10 {
        println!("{:?}", accuracy(Duration::from_millis(50)));
    }
}
