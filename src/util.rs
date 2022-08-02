use macroquad::prelude::*;

pub fn draw_text_centered(text: &str, x: f32, y: f32, font_size: u16) {
    let TextDimensions { width, height, .. } = measure_text(&text, None, font_size, 1.0);
    draw_text(
        &text,
        (screen_width() - width) / 2.0 + x,
        (screen_height() - height) / 2.0 + y,
        font_size as f32,
        WHITE,
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn opposite(self) -> Self {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}
