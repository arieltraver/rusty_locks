use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("thread 1's i is {}", i);
            thread::sleep(Duration::from_millis(2));
        }
    });
    for j in 1..10 {
        println!("thread 2's i is {}", j);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}
