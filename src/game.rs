pub mod snake;

use self::snake::{Snake, Tile};
use crate::util::{draw_text_centered, Direction, Point};
use std::collections::VecDeque;

use ::rand::prelude::*;
use macroquad::prelude::*;

pub type Coord = i8;
pub const BOARD_SZ: Point<Coord> = Point { x: 24, y: 16 };

impl Point<Coord> {
    pub fn random(max: Self) -> Self {
        let mut rng = thread_rng();
        Self {
            x: rng.gen_range(0..max.x),
            y: rng.gen_range(0..max.y),
        }
    }

    pub fn move_towards(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    pub fn wrap(&mut self, max: Self) {
        self.x += max.x;
        self.x %= max.x;
        self.y += max.y;
        self.y %= max.y;
    }
}

struct GridProportions {
    tile_size: f32,
    margin: Point<f32>,
}
impl GridProportions {
    fn calculate() -> Self {
        let tile_size = {
            let max_w = (screen_width() / BOARD_SZ.x as f32).floor();
            let max_h = (screen_height() / BOARD_SZ.y as f32).floor();
            max_w.min(max_h)
        };
        let margin = Point {
            x: (screen_width() - tile_size * BOARD_SZ.x as f32) / 2.0,
            y: (screen_height() - tile_size * BOARD_SZ.y as f32) / 2.0,
        };
        Self { tile_size, margin }
    }
}

pub enum GameState {
    Playing,
    Dead { collision: Point<Coord> },
}

pub struct Game {
    snake: Snake,
    food: Point<Coord>,
    state: GameState,
    input_queue: VecDeque<Direction>,
}
impl Game {
    pub fn new() -> Self {
        let snake = Snake::new(Direction::Right, 3);
        let food = snake.random_unoccupied_point();
        Self {
            snake,
            food,
            state: GameState::Playing,
            input_queue: VecDeque::new(),
        }
    }

    pub fn take_input(&mut self) {
        if let GameState::Dead { .. } = self.state {
            if is_key_pressed(KeyCode::Enter) {
                *self = Self::new();
            }
        }

        // snek input
        for (key_code, desired_direction) in [
            (KeyCode::Up, Direction::Up),
            (KeyCode::Down, Direction::Down),
            (KeyCode::Left, Direction::Left),
            (KeyCode::Right, Direction::Right),
        ] {
            if is_key_pressed(key_code) {
                self.input_queue.push_back(desired_direction);
            }
        }
    }

    pub fn run_tick(&mut self) {
        // ded
        if let GameState::Dead { .. } = self.state {
            return;
        }

        // move snek
        if let Some(direction) = self.input_queue.pop_front() {
            if direction != self.snake.direction.opposite() {
                self.snake.direction = direction;
            }
        }

        let mut head = *self.snake.segments.front().expect("head disappeared");
        head.move_towards(self.snake.direction);
        head.wrap(BOARD_SZ);
        self.snake.segments.push_front(head);

        // check food collision
        if head == self.food {
            self.food = self.snake.random_unoccupied_point();
        } else {
            self.snake.segments.pop_back();
        }

        // check snek collision
        if let Some(&collision) = self
            .snake
            .segments
            .iter()
            .skip(1)
            .find(|&segment| *segment == head)
        {
            self.state = GameState::Dead { collision };
            return;
        }
    }

    pub fn draw_frame(&self) {
        let GridProportions { tile_size, margin } = GridProportions::calculate();
        let draw_tile = |Point { x, y }: Point<Coord>, tile: Tile| {
            draw_rectangle(
                x as f32 * tile_size + margin.x,
                y as f32 * tile_size + margin.y,
                tile_size - 2.0,
                tile_size - 2.0,
                Color::from(tile),
            );
        };

        // draw snek
        {
            let mut iter = self.snake.segments.iter();
            draw_tile(*iter.next().expect("head just disappeared"), Tile::Head);
            for segment in iter {
                draw_tile(*segment, Tile::Snek);
            }
        }
        // draw food
        draw_tile(self.food, Tile::Food);

        if let GameState::Dead { collision } = self.state {
            draw_tile(collision, Tile::Bonk);

            draw_text_centered("Game Over!", 0.0, 0.0, 100);
            draw_text_centered(&format!("Score: {}", self.snake.len()), 0.0, 50.0, 50);
        }
    }
}
