#![no_std]
#![no_main]
#![feature(start)]

extern crate embeddedsw_rs;
use embeddedsw_rs as xemb;
use xemb::println;

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    println!("Hellor Rust World!!");
    return 0;
}
