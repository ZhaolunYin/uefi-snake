use alloc::vec;
use alloc::vec::Vec;
use uefi::proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput};
use font8x8::{UnicodeFonts, BASIC_FONTS};

use crate::shapes::{Rectangle, Point};

pub struct Buffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<BltPixel>
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![BltPixel::new(0,0,0); width * height]
        }
    }

    pub fn pixel(&mut self, p: Point) -> Option<&mut BltPixel> {
        if p.x >= self.width || p.y >= self.height {
            return None;
        }
        self.pixels.get_mut(p.y * self.width + p.x)
    }

    pub fn blit(&self, gop: &mut GraphicsOutput) -> uefi::Result {
        gop.blt(BltOp::BufferToVideo {
            buffer: &self.pixels,
            src: BltRegion::Full,
            dest: (0, 0),
            dims: (self.width, self.height)
        })
    }

    pub fn draw_rect(&mut self, rect: &Rectangle, color: BltPixel) {
        for py in rect.y..rect.y + rect.height {
            for px in rect.x..rect.x + rect.width {
                if let Some(pixel) = self.pixel(Point {x: px, y: py}) {
                    *pixel = color;
                }
            }
        }
    }
    fn draw_char(&mut self, x: usize, y: usize, c: char, size: usize) {
        if let Some(glyph) = BASIC_FONTS.get(c) {
            for (row, bits) in glyph.iter().enumerate() {
                for col in 0..8 {
                    if bits & (1 << col) != 0 {
                        // Draw a size×size block for this font pixel
                        for dy in 0..size {
                            for dx in 0..size {
                                if let Some(pixel) = self.pixel(Point {
                                    x: x + col * size + dx,
                                    y: y + row * size + dy,
                                }) {
                                    *pixel = BltPixel::new(255, 255, 255);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pub fn draw_string(&mut self, x: usize, y: usize, text: &str, size: usize) {
        for (i, c) in text.chars().enumerate() {
            self.draw_char(x + (i * 8 * size), y, c, size);
        }
    }

}
