#![no_std]
#![no_main]
#![feature(start)]

extern crate embeddedsw_rs;
use core::mem::MaybeUninit;
use cstr_core::CStr;
use embeddedsw_rs as xemb;
use embeddedsw_rs::{
    raw,
    xscugic::{self, XScuGic, XScuGicConfig},
};
use xemb::{print, println};

const INTC_DEVICE_ID: u16 = 0;
const INTC_DEVICE_INT_ID: u16 = 0x0e;

static mut INTERRUPT_PROCESSED: bool = false;

#[panic_handler]
fn panic(Info: &core::panic::PanicInfo<'_>) -> ! {
    println!("{}", Info);
    loop {}
}

#[no_mangle]
#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    println!("Interrupt Test");

    // unsafe {
    //     raw::Xil_AssertSetCallback(Some(assert_print));
    // };

    // lookup config
    let xconfig = match XScuGicConfig::lookup_config(INTC_DEVICE_ID) {
        Ok(xconfig) => {
            println!("[Info] lookup config ...[ok]");
            xconfig
        }
        Err(()) => {
            println!("[Info] lookup config ...[fail]");
            return 0;
        }
    };
    let mut xscugic = MaybeUninit::<XScuGic>::uninit();

    // cfg initialize
    match unsafe { XScuGic::cfg_initialize(&mut xscugic, &xconfig, xconfig.get_cpu_base_addr()) } {
        Ok(()) => {
            println!("[Info] cfg initialize ...[ok]");
        }
        Err(e) => {
            println!("[Info] cfg initialize ...[fail]");
            println!("Error: {}", e);
            return 0;
        }
    };
    let mut xscugic = unsafe { xscugic.assume_init() };

    // setupt interrupt system
    xscugic.exception_register_handler();
    unsafe { xscugic::xil_exception_enable() };

    // connect handler
    match xscugic.connect(INTC_DEVICE_INT_ID as u32, Some(device_handler)) {
        Ok(()) => {
            println!("[Info] connect handler ...[ok]");
        }
        Err(e) => {
            println!("[Info] connect handler ...[fail]");
            println!("[Error]: {}", e);
            return 0;
        }
    }

    xscugic.enable(INTC_DEVICE_INT_ID as u32);

    // software interrupt
    let status = unsafe {
        raw::XScuGic_SoftwareIntr(
            &mut xscugic as *mut XScuGic as *mut _,
            INTC_DEVICE_INT_ID as u32,
            raw::XSCUGIC_SPI_CPU0_MASK,
        )
    };
    if status != 0 {
        println!("[Info] software interrupt ...[fail]");
        return 0;
    } else {
        println!("[Info] software interrupt ...[ok]");
    }

    for i in 0..1000 {
        if (i % 100) == 0 {
            println!("[Info] busy loop now");
        }
    }
    loop {
        if unsafe { INTERRUPT_PROCESSED } {
            break;
        }
    }

    println!("Sucessfully Interrupt Test");
    return 0;
}

// extern "C" fn assert_print(file: *const raw::char8, line: i32) {
//     let mut fname = unsafe { CStr::from_ptr(file) };
//     println!("Assert: File Name: {:?}", fname.as_ref());
//     println!("Line Number: {}", line);
// }

extern "C" fn device_handler() {
    println!("[Info] called device handler");
    unsafe {
        INTERRUPT_PROCESSED = true;
    }
}
