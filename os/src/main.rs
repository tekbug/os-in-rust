#![no_std] // no standard library linkage
#![no_main] // no Rust-level entry points

use core::panic::PanicInfo;

static STORY: &[u8] = b"CHECK MY STORY!";

#[no_mangle] // don't mangle the name of this functions
pub extern "C" fn _start() -> ! {
    // the linker looks for function called _start by default
    // printing out a colored VGA buffer for the text `CHECK MY STORY`
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in STORY.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xa;
        }
    }

    loop {}
}



#[panic_handler]
fn panic(_info: &PanicInfo) ->  ! {
    loop {}
}

