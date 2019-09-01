static mut ERROR: i32 = 0;

#[derive(Debug)]
struct SyscallResult(String);

fn some_good_call() -> Result<SyscallResult, String> {
    let msg = String::from("Syscall Success!");
    let res = SyscallResult(msg.clone());
    Ok(res)
}

fn some_bad_call() -> Result<SyscallResult, String> {
    let err_msg = String::from("Permission denied");
    Err(err_msg)
}


fn main() {

    // good call ...
    let res = some_good_call();
    match res {
        Ok(v) => println!("Executed Call: {:?}", v),
        Err(e) => println!("Error executing call: {:?}", e),
    }
    
    // bad call ...
    let res2 = some_bad_call();
    match res2 {
        Ok(v) => println!("Executed Call: {:?}", v),
        Err(e) => {
            println!("Error executing call: {:?}", e);
            unsafe {
                ERROR = 1
            }
        },
    }

    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred!");
        }
    }

}
