use std::ffi::c_int;

use region::{protect_with_handle, Protection};

extern "C" {
    fn open(pathname: *const u8, flags: c_int) -> c_int;
}
#[ctor::ctor]
fn init() {
    println!("===== init=====");
    let addr: *mut u8 = open as *mut _;
    println!("{:#?}", addr);
    unsafe {
        println!("before");
        let _guard = protect_with_handle(addr, 1, Protection::READ_WRITE_EXECUTE).unwrap();
        println!("after");
        *addr = 0xc3;
    }
}
