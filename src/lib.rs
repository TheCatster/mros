#![feature(lang_items)]
#![no_std]

use core::panic::PanicInfo;

#[no_mangle]
pub extern fn rust_main() -> ! {
    // ATTENTION: we have a very small stack and no guard page

    let hello = b"Hello!";
    let color_byte = 0x1f; // white foreground, blue background

    let mut hello_colored = [color_byte; 24];
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i*2] = *char_byte;
    }

    // write `Hello World!` to the center of the VGA text buffer
    let buffer_ptr = (0xb8000) as *mut _;
    unsafe { *buffer_ptr = hello_colored };

    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}