use std::thread::spawn;

fn main() {
    my_func11();
    my_func12();
}

fn my_func11() {
    spawn(hello).join();

    let h = || println!("Hello, world!");
    spawn(h).join();
}

fn hello() {
    println!("Hello, world!");
}

fn my_func12() {
    let v = 10;
    let f = move || v * 2;

    let result = spawn(f).join();
    println!("result = {:?}", result);

    match spawn(|| panic!("panic!")).join() {
        Ok(_) => println!("success"),
        Err(e) => {
            println!("failed: {:?}", e);
            let s = e.downcast_ref::<&str>();
            println!("failed: {:?}", s);
        }
    }
}
