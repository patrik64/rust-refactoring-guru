// fn change(global_state: &mut u32) {
//     *global_state += 1;
// }

// fn main() {
//     let mut global_state = 0u32;

//     change(&mut global_state);

//     println!("Final state: {}", global_state);
// }

// Taken from: https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
//
// Rust doesn't really allow a singleton pattern without `unsafe` because it
// doesn't have a safe mutable global state.
//
// `lazy-static` allows declaring a static variable with lazy initialization
// at first access. It is actually implemented via `unsafe` with `static mut`
// manipulation, however, it keeps your code clear of `unsafe` blocks.
//
// `Mutex` provides safe access to a single object.

// use lazy_static::lazy_static;
// use std::sync::Mutex;

// lazy_static! {
//     static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
// }

// fn do_a_call() {
//     ARRAY.lock().unwrap().push(1);
// }

// fn main() {
//     do_a_call();
//     do_a_call();
//     do_a_call();

//     println!("Called {}", ARRAY.lock().unwrap().len());
// }

// ructc 1.63
// https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
//
// Starting with Rust 1.63, it can be easier to work with global mutable
// singletons, although it's still preferable to avoid global variables in most
// cases.
//
// Now that `Mutex::new` is `const`, you can use global static `Mutex` locks
// without needing lazy initialization.

use std::sync::Mutex;

static ARRAY: Mutex<Vec<i32>> = Mutex::new(Vec::new());

fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

fn main() {
    do_a_call();
    do_a_call();
    do_a_call();

    let array = ARRAY.lock().unwrap();
    println!("Called {} times: {:?}", array.len(), array);
    drop(array);

    *ARRAY.lock().unwrap() = vec![3, 4, 5];

    println!("New singleton object: {:?}", ARRAY.lock().unwrap());
}
