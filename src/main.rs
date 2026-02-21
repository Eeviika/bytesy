#![cfg_attr(
    not(any(debug_assertions, test)),
    deny(clippy::unwrap_used, clippy::expect_used)
)]

use macroquad::prelude::*;

mod dialog;
mod entities;
mod game;
mod rendering;
mod rooms;

#[macroquad::main("Bytesy")]
async fn main() {
    loop {
        clear_background(WHITE);
        draw_text("Hello, world!", 20.0, 20.0, 30.0, BLACK);
        next_frame().await;
    }
}
