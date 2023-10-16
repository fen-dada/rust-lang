use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3, 4];

    let handle2 = thread::spawn(move || {
        //can't infer when the vector will be dropped
        println!("Here's a vector {:?}", v);
    });

    //drop(v);

    handle2.join().unwrap();
}

