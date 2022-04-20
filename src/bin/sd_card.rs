#![no_std]
#![no_main]
#![feature(start)]

extern crate embeddedsw_rs;
use core::mem::MaybeUninit;
use embeddedsw_rs as xemb;
use xemb::{
    ff::{FileAccessMode::*, FileMountOption::*, *},
    print, println,
};

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    println!("SD Card Test");

    //-------------------------------------------------------------------------
    // Mount Logical Drive
    //-------------------------------------------------------------------------
    let path = "0:/\0";
    let opt = Immediately;
    let mut fatfs = MaybeUninit::<FatFs>::uninit();
    match unsafe { FatFs::mount(&mut fatfs, path, opt) } {
        Ok(_) => {
            println!("mount sd card... [ok]");
        }
        Err(fresult) => {
            println!("mount sd card: {:?}", fresult);
            return 0;
        }
    };
    let mut fatfs = unsafe { fatfs.assume_init() };

    //-------------------------------------------------------------------------
    // Open the test.dat file
    //-------------------------------------------------------------------------
    let fname = "test.dat\0";
    let mode = Read;
    let mut fil = MaybeUninit::<Fil>::uninit();
    match unsafe { Fil::open(&mut fil, fname, mode) } {
        Ok(_) => {
            println!("open file... [ok]");
        }
        Err(fresult) => {
            println!("open file: {:?}", fresult);
            return 0;
        }
    };
    let mut fil = unsafe { fil.assume_init() };

    //-------------------------------------------------------------------------
    // Read contents in the test.dat
    //-------------------------------------------------------------------------
    let mut buff = [2; 124];
    let n = 10;
    if let Err(fresult) = fil.read(&mut buff, n) {
        println!("read file: {:?}", fresult);
        return 0;
    } else {
        println!("read file... [ok]");
        // print the 10 bytes that are read by read() function
        for i in 0..n {
            println!("{}", buff[i] as char);
        }
    }

    //-------------------------------------------------------------------------
    // Close the test.dat
    //-------------------------------------------------------------------------
    if let Err(fresult) = fil.close() {
        println!("close file: {:?}", fresult);
        return 0;
    } else {
        println!("close file... [ok]");
    }

    let path = "0:/";
    fatfs.unmount(path);

    println!("Scucessfully Read SD Card Test");
    return 0;
}
