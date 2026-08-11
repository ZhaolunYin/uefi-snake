#![no_main]
#![no_std]

#[global_allocator]
static ALLOCATOR: uefi::allocator::Allocator = uefi::allocator::Allocator;

mod constants;
mod draw;
mod fruit;
mod game;
mod shapes;
mod snake;

extern crate alloc;
use uefi::prelude::*;
use uefi::proto::console::gop::{GraphicsOutput};
use uefi::proto::console::text::{Input};
use uefi::{Result, boot};
use crate::draw::Buffer;
use crate::game::Game;
use crate::shapes::{Grid};

fn game_loop() -> Result {
    boot::set_watchdog_timer(0, 0x10000, None)?;
    uefi::println!("looking up GraphicsOutput");
    let handle = boot::get_handle_for_protocol::<GraphicsOutput>()?;
    let gop = boot::open_protocol_exclusive::<GraphicsOutput>(handle)?;
    uefi::println!("looking up Input");
    let handle = boot::get_handle_for_protocol::<Input>()?;
    let input = boot::open_protocol_exclusive::<Input>(handle)?;

    let (width, height) = gop.current_mode_info().resolution();
    uefi::println!("resolution: {width}x{height}");

    let grid = Grid::new(width, height);
    uefi::println!("cell size: {}", grid.cell_size);
    let buffer = Buffer::new(width, height, gop);
    let mut game = Game::new(grid, buffer, input);
    game.select_difficulty()?;

    loop {
        loop {
            game.game_input()?;
            if !game.move_snake() {
                break;
            }
            game.check_fruit();
            game.draw(true);

            game.stall();
        }
        if game.gameover()? {
            return Ok(());
        }
        game.clear();
    }
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    game_loop().unwrap();
    Status::SUCCESS
}
