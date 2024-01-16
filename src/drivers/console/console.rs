extern crate multiboot2;
use multiboot2::{Tag, TagType, FramebufferTag, FramebufferType};

use super::fb::FrameBuffer;

//use lazy_static::lazy_static;

use core::fmt;
use spin::Mutex;

// println macro
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

lazy_static!{
    // seems that we can't set global frame buffer before find fb
    // so here we set buffer to 0xb8000 first, and adjust to new
    // buffer address after find_fb.
    pub static ref STDOUT: Mutex<FrameBuffer> = Mutex::new(FrameBuffer{
        _width: 800,
        _font_width: 16,
        _font_height: 8,
        _pos_x: 0,
        _pos_y: 0,
        _max_x: 800 / 16,
        _max_y: 600 / 8,
        _buffer: 0xb8000 as usize,
    });
}

unsafe impl Send for STDOUT {}
unsafe impl Sync for STDOUT {}

#[repr(C)]
pub struct MultibootInfo{
    pub total_size: u32,
    pub pad: u32,
}

/// Find frame buffer from multiboot structure.
pub fn find_fb(info: *mut MultibootInfo)->Option<*mut u8>{
    unsafe{
        let mut curr: Tag = *(info.add(1) as *const Tag);
        while curr.typ() != TagType::End{

            if curr.typ() == TagType::Framebuffer{
                let mut fb: &FramebufferTag = curr.cast_tag::<FramebufferTag>();
                let buf_type = fb.buffer_type();
                match buf_type{
                    Ok(_) => {
                        if fb.bpp() == 32 && fb.width() == 800 && fb.height() == 600 && fb.pitch() == 3200{
                            return Some(fb.address() as *mut u8);
                        }
                    }
                    Err(_) => {
                        return None;
                    }
                }

            }
            let next: *const Tag = &curr;
            let next_offset = ((curr.size + 7) &!7) as usize;
            curr = *((next as *const u8).add(next_offset) as *const Tag);
        }
        return None;
    }
}

/// Override format write for frameBuffer.
impl fmt::Write for FrameBuffer{
    fn write_str(&mut self, s: &str)->fmt::Result{
        self.print_str(s);
        Ok(())
    }
}

/// Print function, provide for println! macro.
pub fn _print(args: fmt::Arguments){
    use core::fmt::Write;
    STDOUT.lock().write_fmt(args).unwrap();
}