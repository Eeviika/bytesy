use macroquad::prelude::*;

#[macroquad::main("Hello World")]
async fn main() {
    loop {
        clear_background(WHITE);
        draw_text("Hello, world!", 20.0, 20.0, 30.0, BLACK);
        next_frame().await;
    }
}
