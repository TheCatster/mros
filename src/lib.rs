#![feature(lang_items)]
#![no_std]

use core::panic::PanicInfo;
mod font;
mod fb;
use fb::FrameBuffer;

#[no_mangle]
pub extern "C" fn rust_main(){
    let buffer_ptr: *mut u32 = (0xb8000 + 1988) as *mut u32;
    let mut frameBuffer: FrameBuffer = FrameBuffer{_width: 800, _pos_x: 0, _pos_y: 0, 
        _max_x: 800 / fb::FONT_WIDTH, 
        _max_y: 600 / fb::FONT_WIDTH, 
        _buffer: buffer_ptr};
    
    /* Test output */
    let hello = b"Hello World!";
    for(i, ch) in hello.into_iter().enumerate()
    {
        frameBuffer.output(*ch);
    }

    loop{}
}

#[lang = "eh_personality"] 
#[no_mangle] 
pub extern fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}