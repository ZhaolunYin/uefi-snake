use uefi::proto::console::gop::BltPixel;
use font8x8::unicode::BasicFonts;
use font8x8::BASIC_FONTS;
use crate::game::Difficulty;

// Colors
pub const HEAD_COL: BltPixel = BltPixel::new(0, 255, 0);
pub const TAIL_START_COL: BltPixel = BltPixel::new(255, 255, 0); 
pub const TAIL_END_COL: BltPixel = BltPixel::new(255, 0, 0);
pub const FRUIT_COL: BltPixel = BltPixel::new(255, 0, 255);
pub const TEXT_COL: BltPixel = BltPixel::new(255, 255, 255);
pub const BACKGROUND_COL: BltPixel = BltPixel::new(0, 0, 0);

// Difficulty levels
pub const DIFFICULTIES: [Difficulty; 9] = [
    Difficulty { name: "Snail", tick_rate: 6 },
    Difficulty { name: "Easy", tick_rate: 10 },
    Difficulty { name: "'mid'", tick_rate: 15 },
    Difficulty { name: "Hard", tick_rate: 20 },
    Difficulty { name: "Even Harder", tick_rate: 40 },
    Difficulty { name: "No.", tick_rate: 80 },
    Difficulty { name: "Stop.", tick_rate: 120 },
    Difficulty { name: "Don't You Dare", tick_rate: 200 },
    Difficulty { name: "Why?", tick_rate: 500 },
];

pub const RENDER_FPS: usize = 120;

// font8x8 width and height
pub const FONT: BasicFonts = BASIC_FONTS;
pub const FONT_WIDTH: usize = 8;
pub const FONT_HEIGHT: usize = 8;
