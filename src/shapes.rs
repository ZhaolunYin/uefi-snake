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
        const REF: usize = 1080 * 1920;
        let scale = (((screenwidth * screenheight) + (REF / 2)) / REF).max(1);
        let cell_size = scale * 40;
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
