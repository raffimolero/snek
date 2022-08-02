mod game;
mod util;

use crate::game::Game;

use macroquad::prelude::*;

#[macroquad::main("Snek")]
async fn main() {
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
        game.take_input();
        acc += get_frame_time();
        while acc > SECONDS_PER_TICK {
            acc -= SECONDS_PER_TICK;
            game.run_tick();
        }
        game.draw_frame();
        next_frame().await
    }
}
