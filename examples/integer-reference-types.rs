use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    let _ignore = 0;
    let a = 10;                         // an integer on the stack
    let b = Box::new(20);               // an integer on the heap
    let c = Rc::new(30);                // a boxed integer wrapped behind a reference counter
    let d = Arc::new(Mutex::new(40));   // an integer protected by a mutual exclusion lock, wrapped in an atomic reference counter

    println!("a: {}, b: {}, c: {}, d: {:?}", a, b, c, d);
}
