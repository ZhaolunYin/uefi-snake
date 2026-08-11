use alloc::vec::Vec;

use crate::shapes::{self, Rectangle, Point, Grid};

#[derive(Clone)]
#[derive(PartialEq)]
pub enum Direction { Up, Down, Left, Right }

impl Direction {
    pub fn opposite(&self, other: &Direction) -> bool {
        (*self == Direction::Up && *other == Direction::Down) ||
            (*self == Direction::Down && *other == Direction::Up) ||
            (*self == Direction::Right && *other == Direction::Left) ||
            (*self == Direction::Left && *other == Direction::Right)
    }
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

    pub fn move_head(&mut self, d: &Direction, grid: &Grid, tail: &Vec<Rectangle>) -> bool {
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
            if shapes::check_collision(&self.rect, &segment) { return false; }
        }
        true
    }
}

pub fn move_tail(tail: &mut Vec<Rectangle>, head: &Head) {
    if !tail.is_empty() {
        for i in (1..tail.len()).rev() {
            tail[i].x = tail[i - 1].x;
            tail[i].y = tail[i - 1].y;
        }
        tail[0].x = head.rect.x;
        tail[0].y = head.rect.y;
    }
}
