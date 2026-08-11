use alloc::vec;
use alloc::vec::Vec;
use uefi::proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput};
use uefi::boot::ScopedProtocol;
use font8x8::UnicodeFonts;

use crate::shapes::{Rectangle, Point};
use crate::constants::{FONT, FONT_WIDTH, FONT_HEIGHT};

pub struct Buffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<BltPixel>,
    pub gop: ScopedProtocol<GraphicsOutput>,
}

impl Buffer {
    pub fn new(width: usize, height: usize, gop: ScopedProtocol<GraphicsOutput>) -> Self {
        Self {
            width,
            height,
            pixels: vec![BltPixel::new(0,0,0); width * height],
            gop,
        }
    }

    pub fn pixel(&mut self, p: Point) -> Option<&mut BltPixel> {
        if p.x >= self.width || p.y >= self.height {
            return None;
        }
        self.pixels.get_mut(p.y * self.width + p.x)
    }

    pub fn blit(&mut self) -> uefi::Result {
        self.gop.blt(BltOp::BufferToVideo {
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
    fn draw_char(&mut self, x: usize, y: usize, c: char, size: usize, color: BltPixel) {
        if let Some(glyph) = FONT.get(c) {
            for (row, bits) in glyph.iter().enumerate() {
                for col in 0..FONT_WIDTH {
                    if bits & (1 << col) != 0 {
                        // Draw a size×size block for this font pixel
                        for dy in 0..size {
                            for dx in 0..size {
                                if let Some(pixel) = self.pixel(Point {
                                    x: x + col * size + dx,
                                    y: y + row * size + dy,
                                }) {
                                    *pixel = color;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pub fn draw_string(&mut self, x: usize, y: usize, text: &str, size: usize, color: BltPixel) {
        for (i, c) in text.chars().enumerate() {
            self.draw_char(x + (i * FONT_WIDTH * size), y, c, size, color);
        }
    }
}
pub fn lerp_color(a: BltPixel, b: BltPixel, t: f32) -> BltPixel {
    let t = t.clamp(0.0, 1.0);
    let r: u8 = (a.red as f32 * (1.0 - t) + b.red as f32 * t) as u8;
    let g: u8 = (a.green as f32 * (1.0- t) + b.green as f32 * t) as u8;
    let b: u8 = (a.blue as f32 * (1.0 - t) + b.blue as f32 * t) as u8;
    BltPixel::new(r, g, b)
}
pub fn string_dimensions(s: &str, scale: usize) -> (usize, usize) {
    let width = s.chars().count() * scale * FONT_WIDTH;
    let height = FONT_HEIGHT * scale;
    (width, height)
}
