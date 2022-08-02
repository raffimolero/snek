use std::collections::VecDeque;

use ::macroquad::prelude::*;
use ::rand::prelude::*;

const BOARD_SZ: Point<Coord> = Point { x: 24, y: 16 };

fn draw_text_centered(text: &str, x: f32, y: f32, font_size: u16) {
    let TextDimensions { width, height, .. } = measure_text(&text, None, font_size, 1.0);
    draw_text(
        &text,
        (screen_width() - width) / 2.0 + x,
        (screen_height() - height) / 2.0 + y,
        font_size as f32,
        WHITE,
    );
}

enum Tile {
    Snek,
    Head,
    Food,
    Bonk,
}
impl From<Tile> for Color {
    fn from(tile: Tile) -> Self {
        match tile {
            Tile::Snek => color_u8!(128, 255, 64, 255),
            Tile::Head => color_u8!(64, 255, 128, 255),
            Tile::Food => color_u8!(128, 64, 255, 255),
            Tile::Bonk => color_u8!(255, 64, 128, 255),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn opposite(self) -> Self {
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
struct Point<T> {
    x: T,
    y: T,
}
impl Point<Coord> {
    fn random(max: Self) -> Self {
        let mut rng = thread_rng();
        Self {
            x: rng.gen_range(0..max.x),
            y: rng.gen_range(0..max.y),
        }
    }

    fn move_towards(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn wrap(&mut self, max: Self) {
        self.x += max.x;
        self.x %= max.x;
        self.y += max.y;
        self.y %= max.y;
    }
}

struct Snake {
    direction: Direction,
    segments: VecDeque<Point<Coord>>,
}
impl Snake {
    fn new() -> Self {
        let direction = Direction::Right;
        let mut segments = VecDeque::new();
        let mut head = Point {
            x: BOARD_SZ.x / 2,
            y: BOARD_SZ.y / 2,
        };
        for _ in 0..3 {
            segments.push_front(head);
            head.move_towards(direction);
            head.wrap(BOARD_SZ);
        }
        Self {
            direction,
            segments,
        }
    }

    fn len(&self) -> usize {
        self.segments.len()
    }

    fn random_unoccupied_point(&self) -> Point<Coord> {
        loop {
            let food = Point::random(BOARD_SZ);
            if self.segments.iter().all(|segment| *segment != food) {
                return food;
            }
        }
    }
}

enum GameState {
    Playing,
    Dead { collision: Point<Coord> },
}

struct Game {
    snake: Snake,
    food: Point<Coord>,
    state: GameState,
    input_queue: VecDeque<Direction>,
}
impl Game {
    fn new() -> Self {
        let snake = Snake::new();
        let food = snake.random_unoccupied_point();
        let state = GameState::Playing;
        let input_queue = VecDeque::new();
        Self {
            snake,
            food,
            state,
            input_queue,
        }
    }

    fn input(&mut self) {
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

    fn tick(&mut self) {
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

    fn draw(&self, mut draw_tile: impl FnMut(Point<Coord>, Tile)) {
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

type Coord = i8;

#[macroquad::main("Snek")]
async fn main() {
    // display data
    let tile_size = (screen_width() / BOARD_SZ.x as f32)
        .floor()
        .min((screen_height() / BOARD_SZ.y as f32).floor());
    let margin = Point {
        x: (screen_width() - tile_size * BOARD_SZ.x as f32) / 2.0,
        y: (screen_height() - tile_size * BOARD_SZ.y as f32) / 2.0,
    };
    let draw_tile = |Point { x, y }: Point<Coord>, tile: Tile| {
        draw_rectangle(
            x as f32 * tile_size + margin.x,
            y as f32 * tile_size + margin.y,
            tile_size - 2.0,
            tile_size - 2.0,
            Color::from(tile),
        );
    };

    // game state
    let mut game = Game::new();

    let mut acc = 0.0;
    const TICKS_PER_SECOND: f32 = 10.0;
    const SECONDS_PER_TICK: f32 = 1.0 / TICKS_PER_SECOND;
    loop {
        // quit
        if is_key_pressed(KeyCode::Escape) {
            return;
        }

        // take input
        game.input();

        acc += get_frame_time();
        while acc > SECONDS_PER_TICK {
            acc -= SECONDS_PER_TICK;

            // logical tick
            game.tick();
        }

        // draw frame
        game.draw(draw_tile);
        next_frame().await
    }
}
