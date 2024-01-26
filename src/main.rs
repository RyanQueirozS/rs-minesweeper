use std::i32;

use macroquad::prelude::*;

const BG_COLOR: Color = Color {
    r: 190.0,
    g: 190.0,
    b: 190.0,
    a: 190.0,
};

const _FG_COLOR: Color = Color {
    r: 255.0,
    g: 255.0,
    b: 255.0,
    a: 255.0,
};

struct Cell {
    _number: i32,
    _is_clicked: bool,
}

fn generate_board() -> [[Cell; 99]; 99] {
    let _board: [[Cell; 99]; 99] = todo!();
    return _board;
}

#[macroquad::main("Game")]
async fn main() {
    let _board: [[Cell; 99]; 99] = generate_board();
    loop {
        clear_background(BG_COLOR);
        next_frame().await;
    }
}
