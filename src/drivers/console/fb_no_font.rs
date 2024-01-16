#![allow(dead_code)]

use core::slice::from_raw_parts_mut;

#[derive(Debug, Clone, Copy)]
pub struct FrameBufferNoFont{
    pub _width: usize,
    pub _height: usize,

    pub _pos_x: usize,
    pub _pos_y: usize,

    pub _buffer: usize,
}

#[repr(C)]
pub struct CharByte{
    _char: u8,
    _color: u8,
}

impl CharByte{
    pub fn default(ch: u8) -> Self{
        Self{ _char: ch, _color: 0x0f, }
    }

    pub fn set(ch: u8, color: u8) -> Self{
        Self{ _char: ch, _color: color }
    }

    pub fn set_color(&mut self, color: u8){
        self._color = color;
    }
}

impl FrameBufferNoFont{

    /// Clear frame buffer.
    pub fn clear(&mut self){
        let buffer_size: usize = self._width * self._height;
        let buffer = unsafe{from_raw_parts_mut(self._buffer as *mut CharByte, buffer_size)};
        for i in 0..buffer_size{
            buffer[i] = CharByte::default(0);
        }
    }

    /// Print a character to frame buffer.
    pub fn output(&mut self, ch: u8){
        if ch == b'\n' || self._pos_x == self._width{
            self._pos_x += 1;
            self._pos_y = 0;
        }

        if ch == b'\n'{
            return ;
        }

        let buffer_size: usize = self._width * self._height;
        let buffer = unsafe{from_raw_parts_mut(self._buffer as *mut CharByte, buffer_size)};

        let pos: usize = self._pos_x * self._width + self._pos_y;
        buffer[pos] = CharByte::default(ch);

        self._pos_y += 1;
    }

    /// Print a string to frame buffer.
    pub fn print_str(&mut self, s: &str){
        for ch in s.bytes(){
            self.output(ch);
        }
    }
}