#![no_main]
#![no_std]

#[global_allocator]
static ALLOCATOR: uefi::allocator::Allocator = uefi::allocator::Allocator;

mod shapes;
mod snake;

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use core::time::Duration;
use uefi::prelude::*;
use uefi::proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput};
use uefi::proto::console::text::{Input, Key, ScanCode};
use uefi::{Result, boot};

use crate::shapes::{Point, Rectangle, Grid};
use crate::snake::{Body, Direction, Head};

struct Fruit {
    rect: Rectangle,
    color: BltPixel,
    seed: usize,
}

impl Fruit {
    fn new(grid: &Grid, mut seed: usize) -> Self {
        seed = seed
            .wrapping_mul(1664525)
            .wrapping_add(1013904223);
        let x = (seed % grid.width) * grid.cell_size;
        seed = seed
            .wrapping_mul(1664525)
            .wrapping_add(1013904223);
        let y = (seed % grid.height) * grid.cell_size;
        Self {
            rect: Rectangle {
                x: x,
                y: y,
                width: grid.cell_size,
                height: grid.cell_size,
            },
            color: BltPixel::new(255, 0, 0),
            seed,
        }
    }
}

struct Buffer {
    width: usize,
    height: usize,
    pixels: Vec<BltPixel>
}

impl Buffer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![BltPixel::new(0,0,0); width * height]
        }
    }

    fn pixel(&mut self, p: Point) -> Option<&mut BltPixel> {
        if p.x >= self.width || p.y >= self.height {
            return None;
        }
        self.pixels.get_mut(p.y * self.width + p.x)
    }

    fn blit(&self, gop: &mut GraphicsOutput) -> uefi::Result {
        gop.blt(BltOp::BufferToVideo {
            buffer: &self.pixels,
            src: BltRegion::Full,
            dest: (0, 0),
            dims: (self.width, self.height)
        })
    }

    fn draw_rect(&mut self, rect: &Rectangle, color: BltPixel) {
        for py in rect.y..rect.y + rect.height {
            for px in rect.x..rect.x + rect.width {
                if let Some(pixel) = self.pixel(Point {x: px, y: py}) {
                    *pixel = color;
                }
            }
        }
    }
}


fn move_tail(tail: &mut Vec<Body>, head: &Head) {
    if !tail.is_empty() {
        for i in (1..tail.len()).rev() {
            tail[i].rect.x = tail[i - 1].rect.x;
            tail[i].rect.y = tail[i - 1].rect.y;
        }
        tail[0].rect.x = head.rect.x;
        tail[0].rect.y = head.rect.y;
    }
}


fn game_loop() -> Result {
    uefi::println!("looking up GraphicsOutput");
    let handle = boot::get_handle_for_protocol::<GraphicsOutput>()?;
    let mut gop = boot::open_protocol_exclusive::<GraphicsOutput>(handle)?;
    uefi::println!("looking up Input");
    let handle = boot::get_handle_for_protocol::<Input>()?;
    let mut input = boot::open_protocol_exclusive::<Input>(handle)?;

    let (width, height) = gop.current_mode_info().resolution();
    uefi::println!("resolution: {width}x{height}");

    let grid = Grid::new(width, height);
    uefi::println!("cell size: {}", grid.cell_size);

    let mut head = Head::new(Rectangle {
        x: (grid.width / 2) * grid.cell_size,
        y: (grid.height / 2) * grid.cell_size,
        width: grid.cell_size,
        height: grid.cell_size,
    });

    let mut tail: Vec<Body> = Vec::new();
    let mut dir = Direction::Right;
    let mut fruit = Fruit::new(&grid, uefi::runtime::get_time()
        .unwrap()
        .second() as usize);

    let mut buffer = Buffer::new(width, height);

    loop {
        if let Some(key) = input.read_key()? {
            match key {
                Key::Special(ScanCode::UP) => dir = Direction::Up,
                Key::Special(ScanCode::DOWN) => dir = Direction::Down,
                Key::Special(ScanCode::LEFT) => dir = Direction::Left,
                Key::Special(ScanCode::RIGHT) => dir = Direction::Right,
                _ => {}
            }
        }

        move_tail(&mut tail, &head);
        if !head.move_head(&dir, &grid, &tail) {
            break;
        }

        if shapes::check_collision(&head.rect, &fruit.rect) {
            let new = Body {
                rect: head.rect.clone(),
                lastpos: head.lastpos.clone()
            };
            tail.push(new);

            fruit = Fruit::new(&grid, fruit.seed);
        }

        buffer.draw_rect(&fruit.rect, fruit.color);
        for segment in &tail {
            buffer.draw_rect(&segment.rect, BltPixel::new(255, 255, 0))
        }
        buffer.draw_rect(&head.rect, BltPixel::new(0, 255, 0));

        buffer.blit(&mut gop)?;
        buffer.pixels.fill(BltPixel::new(0, 0, 0));
        boot::stall(Duration::from_millis(100));
    };
    Ok(())
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    game_loop().unwrap();
    Status::SUCCESS
}
