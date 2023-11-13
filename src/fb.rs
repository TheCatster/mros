use crate::font::_ASCII_FONT;

pub const FONT_WIDTH: usize = 2;
pub const FONT_HEIGHT: usize = 2;

pub struct FrameBuffer
{
    pub _width: usize,
    pub _pos_x: usize,
    pub _pos_y: usize,
    pub _max_x: usize,
    pub _max_y: usize,
    pub _buffer: *mut u32,
}

impl FrameBuffer
{
    pub fn new(&mut self, fb: *mut u32, width: usize, height: usize)
    {
        let num: usize = width * height;
        /* Clean up the scree */
        for i in 0..num
        {
            unsafe{fb.add(i as usize).write(0x0);}
        }

        self._buffer = fb;
        self._width = width;
        self._pos_x = 0;
        self._pos_y = 0;
        self._max_x = width / FONT_WIDTH;
        self._max_y = height / FONT_HEIGHT;
    }

    pub fn scrollup(&mut self)
    {
        let mut cur: usize = 0;
        let mut count: usize = self._width * ((self._max_y - 1) * FONT_HEIGHT);
        let mut row: usize = self._width * FONT_HEIGHT;

        loop
        {
            unsafe
            {
                let content: u32 = *self._buffer.add(cur + row);
                self._buffer.add(cur).write(content);
            }
            cur += 1;

            count -= 1;
            if count == 0
            {
                break;
            }
        }

        loop
        {
            unsafe
            {
                self._buffer.add(cur).write(0x0);
                cur += 1;

                row -= 1;
                if row == 0
                {
                    break;
                }
            }
        }
    }

    pub fn output(&mut self, ch: u8)
    {
        if ch == b'\n' || self._pos_x == self._max_x
        {
            self._pos_x = 0;
            self._pos_y += 1;
        }

        if self._pos_y == self._max_y
        {
            self._pos_y -= 1;
            self.scrollup();
        }

        if ch == b'\n'
        {
            return;
        }

        let index: usize = (ch as usize) * (FONT_WIDTH * FONT_HEIGHT / 8);
        unsafe
        {
            let ptr: *mut u8 = ((_ASCII_FONT as *const u8).add(index)) as *mut u8;
            let mut cur: usize = self._pos_x * FONT_WIDTH + (self._pos_y * FONT_HEIGHT) * self._width;
            for j in 0..FONT_HEIGHT
            {
                let mut bitmap: i8 = ptr.add(j) as i8;
                for i in 0..FONT_WIDTH
                {
                    let color: i8 = bitmap >> 7;
                    self._buffer.add(cur + (i as usize)).write(color as u32);
                    bitmap <<= 1;
                }
                cur += self._width;
            }
            self._pos_x += 1;
        }
    }
}