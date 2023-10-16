use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Arc::new(Mutex::new(0)); // Rc<T> in threads

    let mut handles = vec![];

    for _ in 0..10 {
        let m = Arc::clone(&m);
        let handle = thread::spawn(move || {
            let mut num = m.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for i in handles {
        i.join().unwrap();
    }

    println!("Result: {}", *m.lock().unwrap());
}

