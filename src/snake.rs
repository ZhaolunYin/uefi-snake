use alloc::vec::Vec;
use uefi::proto::console::gop::BltPixel;

use crate::shapes::{self, Rectangle, Point, Grid};

pub enum Direction { Up, Down, Left, Right }

pub struct Body {
    pub rect: Rectangle,
    pub lastpos: Point,
}

pub struct Head {
    pub rect: Rectangle,
    pub lastpos: Point
}

impl Head {
    pub fn new(rect: Rectangle) -> Self {
        let lastpos = Point {x: rect.x, y: rect.y};
        Self {
            rect,
            lastpos
        }
    }

    pub fn move_head(&mut self, d: &Direction, grid: &Grid, tail: &Vec<Body>) -> bool {
        self.lastpos = Point {x: self.rect.x, y: self.rect.y};
        match d {
            Direction::Up => {
                if self.rect.y == 0 { return false; }
                self.rect.y -= grid.cell_size;
            }
            Direction::Down => {
                if self.rect.y + grid.cell_size >= grid.height * grid.cell_size { return false; }
                self.rect.y += grid.cell_size;
            }
            Direction::Left => {
                if self.rect.x == 0 { return false; }
                self.rect.x -= grid.cell_size;
            }
            Direction::Right => {
                if self.rect.x + grid.cell_size >= grid.width * grid.cell_size { return false; }
                self.rect.x += grid.cell_size;
            }
        }
        for segment in tail {
            if shapes::check_collision(&self.rect, &segment.rect) { return false; }
        }
        true
    }
}

pub fn move_tail(tail: &mut Vec<Body>, head: &Head) {
    if !tail.is_empty() {
        for i in (1..tail.len()).rev() {
            tail[i].rect.x = tail[i - 1].rect.x;
            tail[i].rect.y = tail[i - 1].rect.y;
        }
        tail[0].rect.x = head.rect.x;
        tail[0].rect.y = head.rect.y;
    }
}
