use macroquad::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(PartialEq, Debug, Copy, Clone)]
struct Cell {
    number: u8,
    is_clicked: bool,
}

fn generate_begginer_board() -> [[Cell; 9]; 9] {
    let mines = 10;

    let mut board = [[Cell {
        number: 0,
        is_clicked: false,
    }; 9]; 9];
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    rand::srand(duration.as_millis() as u64);

    for mut _i in 1..mines {
        let rngx = rand::gen_range(0, board.len() - 1);
        let rngy = rand::gen_range(0, board.len() - 1);

        if board[rngx][rngy].number != 9 {
            board[rngx][rngy] = Cell {
                number: 9,
                is_clicked: false,
            }
        } else {
            _i = 1;
        }
    }

    return board;
}

#[macroquad::main("Game")]
async fn main() {
    loop {
        let board: [[Cell; 9]; 9] = generate_begginer_board();

        let mut count = 0;
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j].number == 9 {
                    count += 1;
                }
            }
        }
        if count != 9 {
            println!("{count}");
        }
    }
    // loop {
    //     let _cell_width = screen_width() / board.len() as f32;
    //     let _cell_height = screen_height() / board[0].len() as f32;

    // clear_background(BLACK);
    // request_new_screen_size(1024.0, 768.0);
    // for i in 0..(&board).len() {
    //     for j in 0..(&board[0]).len() {
    //         draw_rectangle(
    //             _cell_width * i as f32,
    //             _cell_height * j as f32,
    //             _cell_width - 1.0,
    //             _cell_height - 1.0,
    //             DARKGRAY,
    //         );
    //     }
    // }
    // next_frame().await;
    // }
}
