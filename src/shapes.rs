#[derive(Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

#[derive(Clone)]
pub struct Rectangle {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

pub struct Grid {
    pub cell_size: usize,
    pub scale: usize,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(screenwidth: usize, screenheight: usize) -> Self{
        let scale: f32 = match screenheight {
            0..=1079 => 1.0,
            1080..=2159 => 1.5,
            _ => 2.0,
        };
        let cell_size = (scale * 40.0) as usize;
        let scale: usize = match scale {
            1.0 => 1,
            _ => 2,
        };
        Self {
            cell_size,
            scale,
            width: screenwidth / cell_size,
            height: screenheight / cell_size,
        }
    }
}

pub fn check_collision(a: &Rectangle, b: &Rectangle) -> bool {
    a.x < b.x + b.width && a.x + a.width > b.x &&
        a.y < b.y + b.height && a.y + a.height > b.y
}
