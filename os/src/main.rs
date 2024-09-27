#![no_std] // no standard library linkage
#![no_main] // no Rust-level entry points

use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this functions
pub extern "C" fn _start() -> ! {
    // the linker looks for function called _start by default
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) ->  ! {
    loop {}
}

