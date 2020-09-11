use std::sync::{Arc, Mutex};
use std::thread;

#[allow(dead_code)]
fn basic() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
        // we don't need release lock munually, it's done via Drop trait
    }

    println!("m = {:?}", m);
}

fn multiple_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("counter: {}", *counter.lock().unwrap());
}

fn main() {
    // basic();
    multiple_threads();
}
