#![no_std] // standard library depends on host OS and libc, so we disable it
#![no_main] // we dont have a runtime environment to call it so we disable main

use core::panic::PanicInfo;

#[panic_handler] // used to define the behavior of panic! in #![no_std] app
fn panic(_info: &PanicInfo) -> ! {
    loop {}    
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // turns off Rust's name mangling, so that it is easier to link to
pub extern "C" fn _start() -> ! { // this function is adhere to the C calling convention
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}
