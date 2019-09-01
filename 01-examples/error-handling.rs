static mut ERROR: i32 = 0;

fn some_bad_syscall() -> i32 {
    1
}

fn main() {
    // system-call ...
    let res = some_bad_syscall();
    unsafe {
        ERROR = res
    }
    
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred!");
        }
    }
}
