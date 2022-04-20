#![no_std]
#![no_main]
#![feature(start)]

extern crate alloc;
extern crate embeddedsw_rs;
use alloc::vec::Vec;
use embeddedsw_rs as xemb;
use xemb::{print, println};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    println!("Allocator Test");
    let mut v = Vec::new();

    for i in 0..100 {
        if (i + 1) % 10 == 0 {
            println!("pushed [{}] elements", i + 1);
        }
        v.push(i as usize);
    }

    for (i, e) in v.iter().enumerate() {
        if *e != i {
            println!("[Error] expected {}, but got {}", i, e);
        }
        if (i + 1) % 10 == 0 {
            println!("poped [{}] elements", i + 1);
        }
    }

    println!("Sucessfully Allocator Test");
    return 0;
}
