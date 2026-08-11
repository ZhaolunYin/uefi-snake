use alloc::vec::Vec;
use alloc::string::{String, ToString};
use alloc::format;
use uefi::proto::console::text::{Input, Key, ScanCode};
use uefi::boot::ScopedProtocol;
use uefi::{Result, boot};
use core::time::Duration;

use crate::draw::{Buffer, lerp_color, string_dimensions};
use crate::fruit::Fruit;
use crate::shapes::{self, Grid, Rectangle};
use crate::snake::{self, Head, Direction};
use crate::constants::{HEAD_COL, TAIL_START_COL, TAIL_END_COL, FRUIT_COL, TEXT_COL, BACKGROUND_COL, DIFFICULTIES, RENDER_FPS};

pub struct Difficulty<'a> {
    pub name: &'a str,
    pub tick_rate: usize,
}

pub struct Game {
    pub fruit: Fruit,
    pub head: Head,
    pub tail: Vec<Rectangle>,
    pub dir: Direction,

    pub grid: Grid,
    pub buffer: Buffer,
    pub input: ScopedProtocol<Input>,

    frame_n: usize,
    pub tick_rate: usize,
    pub highscore: usize,
}
impl Game {
    pub fn new(grid: Grid, buffer: Buffer, input: ScopedProtocol<Input>) -> Self {
        let head = Head::new(Rectangle {
            x: (grid.width / 2) * grid.cell_size,
            y: (grid.height / 2) * grid.cell_size,
            width: grid.cell_size,
            height: grid.cell_size,
        });

        let tail: Vec<Rectangle> = Vec::new();
        let dir = Direction::Right;
        let fruit = Fruit::new(&grid, uefi::runtime::get_time()
            .unwrap()
            .second() as usize, FRUIT_COL);

        Self {
            fruit,
            head,
            tail,
            dir,
            grid,
            buffer,
            input,
            frame_n: 0,
            tick_rate: 15,
            highscore: 0,
        }
    }
    pub fn select_difficulty(&mut self) -> Result {
        self.buffer.pixels.fill(BACKGROUND_COL);

        let mut lines: Vec<String> = Vec::new();
        lines.push("Select Difficulty:".to_string());
        for i in 0..DIFFICULTIES.len() {
            lines.push(format!("{} {} ({} tick rate)", i + 1, DIFFICULTIES[i].name, DIFFICULTIES[i].tick_rate));
        }
        let (_, height) = string_dimensions("", self.grid.scale * 2);
        for i in 0..lines.len() {
            self.buffer.draw_string(
                self.grid.scale * 2,
                self.grid.scale * 2 + (i * height),
                lines[i].as_str(),
                self.grid.scale * 2,
                TEXT_COL
            );
        }
        self.buffer.blit()?;
        loop {
            while let Some(key) = self.input.read_key()? {
                for i in 1..=lines.len() {
                    if key == Key::Printable(uefi::Char16::try_from((b'0' + (i as u8)) as char).unwrap()) {
                        self.tick_rate = DIFFICULTIES[i - 1].tick_rate;
                        return Ok(())
                    }
                }
            }
        }
    }
    pub fn game_input(&mut self) -> Result {
        let old = self.dir.clone();
        while let Some(key) = self.input.read_key()? {
            match key {
                Key::Special(ScanCode::UP) => self.dir = Direction::Up,
                Key::Printable(c) if c == uefi::Char16::try_from('w').unwrap() => self.dir = Direction::Up,

                Key::Special(ScanCode::LEFT) => self.dir = Direction::Left,
                Key::Printable(c) if c == uefi::Char16::try_from('a').unwrap() => self.dir = Direction::Left,

                Key::Special(ScanCode::DOWN) => self.dir = Direction::Down,
                Key::Printable(c) if c == uefi::Char16::try_from('s').unwrap() => self.dir = Direction::Down,

                Key::Special(ScanCode::RIGHT) => self.dir = Direction::Right,
                Key::Printable(c) if c == uefi::Char16::try_from('d').unwrap() => self.dir = Direction::Right,

                Key::Special(ScanCode::ESCAPE) => self.pause_screen().unwrap(),
                Key::Printable(c) if c == uefi::Char16::try_from(' ').unwrap() => self.pause_screen().unwrap(),

                _ => {}
            }
        }
        if self.dir.opposite(&old) {
            self.dir = old;
        }
        Ok(())
    }

    fn pause_screen(&mut self) -> Result {
        self.draw(false);

        let string = "Game paused. Press ESC or SPACE to resume.";
        let (width, height) = string_dimensions(string, self.grid.scale * 2);
        self.buffer.draw_string(
            (self.grid.width * self.grid.cell_size / 2) - (width / 2),
            (self.grid.height * self.grid.cell_size / 2) - (height / 2),
            string,
            self.grid.scale * 2,
            TEXT_COL
        );
        self.buffer.blit()?;

        loop {
            if let Some(key) = self.input.read_key()? {
                match key {
                    Key::Special(ScanCode::ESCAPE) => {
                        self.buffer.pixels.fill(BACKGROUND_COL);
                        return Ok(()); }
                    Key::Printable(c) if c == uefi::Char16::try_from(' ').unwrap() => {
                        self.buffer.pixels.fill(BACKGROUND_COL);
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn move_snake(&mut self) -> bool {
        snake::move_tail(&mut self.tail, &self.head);
        self.head.move_head(&self.dir, &self.grid, &self.tail)
    }

    pub fn check_fruit(&mut self) {
        if shapes::check_collision(&self.head.rect, &self.fruit.rect) {
            // clone head - new segment gets pushed to back on next frame
            let new = self.head.rect.clone();
            self.tail.push(new);
            self.fruit = Fruit::new(&self.grid, self.fruit.seed, FRUIT_COL);
        }
    }

    pub fn draw(&mut self, normal_frame: bool) {
        if self.frame_n % self.tick_rate.div_ceil(RENDER_FPS) != 0 && normal_frame {
            return;
        }
        // clear
        self.buffer.pixels.fill(BACKGROUND_COL);
        // draw tail
        for i in 0..self.tail.len() {
            self.buffer.draw_rect(&self.tail[i], lerp_color(TAIL_START_COL, TAIL_END_COL, i as f32 / self.tail.len() as f32));
        }
        // draw fruit
        self.buffer.draw_rect(&self.fruit.rect, self.fruit.color);
        // draw head
        self.buffer.draw_rect(&self.head.rect, HEAD_COL);

        if self.tail.len() > self.highscore {
            self.highscore = self.tail.len()
        }

        // draw score
        self.buffer.draw_string(
            self.grid.scale * 2,
            self.grid.scale * 2,
            format!("Score: {} Highscore: {}", self.tail.len(), self.highscore).as_str(),
            self.grid.scale * 2,
            TEXT_COL
        );
        self.buffer.blit().unwrap();
    }

    pub fn gameover(&mut self) -> Result<bool> {
        self.draw(false);

        let string = "Game Over! Press R to Restart, C to Change Difficulty or Q to Quit.";
        let (width, height) = string_dimensions(string, self.grid.scale * 2);
        self.buffer.draw_string(
            (self.grid.width * self.grid.cell_size / 2) - (width / 2),
            (self.grid.height * self.grid.cell_size / 2) - (height / 2),
            string,
            self.grid.scale * 2,
            TEXT_COL
        );
        self.buffer.blit()?;

        loop {
            while let Some(key) = self.input.read_key()? {
                match key {
                    Key::Printable(c) if c == uefi::Char16::try_from('r').unwrap() => return Ok(false),
                    Key::Printable(c) if c == uefi::Char16::try_from('c').unwrap() => {
                        let old = self.tick_rate;
                        self.select_difficulty()?;
                        if self.tick_rate > old {
                            self.highscore = 0;
                        }
                        return Ok(false);
                    }
                    Key::Printable(c) if c == uefi::Char16::try_from('q').unwrap() => return Ok(true),
                    _ => {}
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.buffer.pixels.fill(BACKGROUND_COL);
        // Move head to middle
        self.head.rect.x = (self.grid.width / 2) * self.grid.cell_size;
        self.head.rect.y = (self.grid.height / 2) * self.grid.cell_size;
        // Clear head
        self.tail.clear();
        self.dir = Direction::Right;
        self.fruit = Fruit::new(&self.grid, self.fruit.seed, FRUIT_COL);
    }
    pub fn stall(&mut self) {
        self.frame_n += 1;
        boot::stall(Duration::from_millis((1000 / self.tick_rate) as u64));
    }
}
