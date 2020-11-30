#![no_std] // standard library depends on host OS and libc, so we disable it
#![no_main] // we dont have a runtime environment to call it so we disable main

mod vga_buffer;
use core::panic::PanicInfo;

#[panic_handler] // used to define the behavior of panic! in #![no_std] app
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}    
}

#[no_mangle] // turns off Rust's name mangling, so that it is easier to link to
pub extern "C" fn _start() -> ! { // this function is adhere to the C calling convention
    println!("Hello World{}", "!");
    panic!("Some panic message!);
    loop {}
}
