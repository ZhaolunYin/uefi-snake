use crate::shapes::{Rectangle, Grid};
use uefi::proto::console::gop::BltPixel;

pub struct Fruit {
    pub rect: Rectangle,
    pub color: BltPixel,
    pub seed: usize,
}

impl Fruit {
    pub fn new(grid: &Grid, mut seed: usize) -> Self {
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
