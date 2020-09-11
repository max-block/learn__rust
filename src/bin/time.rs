use std::time::SystemTime;

fn current_time() {
    let now = SystemTime::now();
    println!("{:?}", now);
    // SystemTime { tv_sec: 1599461835, tv_nsec: 433910000 }
}

fn main() {
    current_time();
}
