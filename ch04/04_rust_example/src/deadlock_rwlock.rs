use std::sync::{Arc, RwLock};
use std::thread;

fn deadlock_sample() {
    let val = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
        let flag = val.read().unwrap(); // <1>
        if *flag {
            *val.write().unwrap() = false; // <2>
            println!("flag is true");
        }
    });

    t.join().unwrap();
}

fn fixed_deadlock_sample() {
    let val = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
        let flag = *val.read().unwrap(); // <1>
        if flag {
            *val.write().unwrap() = false; // <2>
            println!("flag is true");
        }
    });

    t.join().unwrap();
}

fn deadlock_sample2() {
    let val = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
        let _flag = val.read().unwrap(); // <1>
        *val.write().unwrap() = false; // <2>
        println!("deadlock");
    });

    t.join().unwrap();
}

fn fixed_deadlock_sample2() {
    let val = Arc::new(RwLock::new(true));

    let t = thread::spawn(move || {
        let _ = val.read().unwrap(); // <1>
        *val.write().unwrap() = false; // <2>
        println!("not deadlock");
    });

    t.join().unwrap();
}
