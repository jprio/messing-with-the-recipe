use region::{protect_with_handle, Protection};

fn main() {
    let addr: *mut u8 = say_hi as *mut _;
    unsafe {
        println!("before");
        let _guard = protect_with_handle(addr, 1, Protection::READ_WRITE_EXECUTE).unwrap();
        println!("after");
        *addr = 0xc3;
    }
    say_hi()
}

fn say_hi() {
    println!("Hi");
}
