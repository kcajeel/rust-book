use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

// I will try to create a deadlock here
pub fn deadlock() {
    let r1 = Arc::new(Mutex::new(0));
    let r2 = Arc::new(Mutex::new(1));
    let mut handles = vec![];

    for _ in 0..2 {
        let r1 = Arc::clone(&r1);
        let r2 = Arc::clone(&r2);
        let handle = thread::spawn(move || {
            check_resources(r1, r2);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn check_resources(r1: Arc<Mutex<i32>>, r2: Arc<Mutex<i32>>) {
    if let Ok(val1) = r1.lock() {
        println!("Got r1: {}", val1);

        if let Ok(val2) = r2.lock() {
            println!("Got r2: {}", val2); // Mutex is unlocked at the end of this scope
                                          // So a deadlock is actually avoided
        } else {
            thread::sleep(Duration::from_secs(1));
            check_resources(Arc::clone(&r1), Arc::clone(&r2));
        }

    } else {
        thread::sleep(Duration::from_secs(1));
        check_resources(Arc::clone(&r1), Arc::clone(&r2));
    };
}
