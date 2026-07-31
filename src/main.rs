#![no_main]
#![no_std]

#[global_allocator]
static ALLOCATOR: uefi::allocator::Allocator = uefi::allocator::Allocator;

mod draw;
mod fruit;
mod shapes;
mod snake;

extern crate alloc;
use alloc::vec::Vec;
use alloc::format;
use core::time::Duration;
use uefi::prelude::*;
use uefi::proto::console::gop::{BltPixel, GraphicsOutput};
use uefi::proto::console::text::{Input, Key, ScanCode};
use uefi::{Result, boot};

use crate::fruit::Fruit;
use crate::shapes::{Rectangle, Grid};
use crate::snake::{Body, Direction, Head};
use crate::draw::Buffer;

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

        snake::move_tail(&mut tail, &head);
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

        buffer.draw_string(grid.scale * 2, grid.scale * 2, format!("Score: {}", tail.len()).as_str(), grid.scale * 2);

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
