use super::font::_ASCII_FONT;

#[derive(Debug, Clone, Copy)]
pub struct FrameBuffer{
    pub _width: usize,
    pub _font_width: usize,
    pub _font_height: usize,

    pub _pos_x: usize,
    pub _pos_y: usize,
    pub _max_x: usize,
    pub _max_y: usize,

    pub _buffer: usize,
}

impl FrameBuffer{
    /// Convert buffer to *mut u32
    pub fn buffer_to_ptr(&self) -> *mut u32{
        self._buffer as *mut u32
    }

    /// Clean all frameBuffer.
    pub fn clean(&mut self){
        let buffer_size: u32 = self._max_x * self._font_width * self._max_y * self._font_height;
        for i in 0..buffer_size{
            unsafe{ self.buffer_to_ptr().add(i as usize).write(0x00000000); }
        }
    }

    /// Set frameBuffer base.
    pub fn set_base(&mut self, buffer: *mut u32){
        unsafe{self._buffer = buffer as usize;}
    }

    /// Scroll up frameBuffer.
    pub fn scroll_up(&mut self){

        let mut count: usize = self._width * ((self._max_y - 1) * self._font_height);

        let mut cur: usize = 0;
        let mut row: usize = self._width * self._font_height;

        // Move the text up one row
        loop{
            unsafe{
                let content: u32 = *self.buffer_to_ptr().add(cur + row);
                self.buffer_to_ptr().add(cur).write(content);
            }
            cur += 1;

            count -= 1;
            if count == 0{
                break;
            }
        }

        // Clean up the last row
        loop{
            unsafe{
                self.buffer_to_ptr().add(cur).write(0x00000000);
            }

            cur += 1;

            row -= 1;
            if row == 0{
                break;
            }
        }
    }
    
    /// Output a single character on frameBuffer.
    pub fn output(&mut self, ch: u8){

        let mut mch: u8 = ch;
        if (mch as i8) <= 0{
            if mch == 0{
                return ;
            }
            mch = b'?';
        }

        if mch == b'\n' || self._pos_x == self._max_x{
            self._pos_x = 0;
            self._pos_y += 1;
        }

        if self._pos_y == self._max_y{
            self._pos_y -= 1;
            self.scroll_up();
        }

        if mch == b'\n'{
            return ;
        }

        let index: usize = (ch as usize) * (self._font_width * self._font_height / 8);
        unsafe{
            let ptr: *mut u8 = ((_ASCII_FONT as *const u8).add(index)) as *mut u8;
            let mut cur: usize = self._pos_x * self._font_width + (self._pos_y * self._font_height) * self._width;
            for j in 0..self._font_height{
                let mut bitmap: i8 = (*ptr.add(j)) as i8;
                for i in 0..self._font_width{
                    let color: i32 = (bitmap >> 7) as i32;
                    self.buffer_to_ptr().add(cur + i).write(color as u32);
                    bitmap <<= 1;
                }
                cur += self._width;
            }
            self._pos_x += 1;
        }
    }
    
    /// Print a string to frameBuffer.
    pub fn print_str(&mut self, s: &str){
        for ch in s.bytes(){
            self.output(ch);
        }
    }

}