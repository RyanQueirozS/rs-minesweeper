use macroquad::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(PartialEq, Debug, Copy, Clone)]
struct Cell {
    number: u8,
    is_clicked: bool,
}

fn generate_begginer_board() -> [[Cell; 9]; 9] {
    let mut _count = 0;
    let mut board = [[Cell {
        number: 0,
        is_clicked: false,
    }; 9]; 9];

    {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        rand::srand(duration.as_millis() as u64);
    }

    loop {
        let rngx = rand::gen_range(0, board.len());
        let rngy = rand::gen_range(0, board.len());

        if board[rngx][rngy].number != 9 {
            board[rngx][rngy] = Cell {
                number: 9, // 9 will be the mine number
                is_clicked: false,
            };
            _count += 1;
        }
        if _count == 10 {
            break;
        }
    }

    return board;
}

#[macroquad::main("Minesweeper")]
async fn main() {
    let board: [[Cell; 9]; 9] = generate_begginer_board();

    for i in 0..board.len() {
        for j in 0..board[i].len() {
            print!("{} ", board[i][j].number);
        }
        println!();
    }

    loop {
        request_new_screen_size(1024.0, 768.0);

        let cell_width = screen_width() / board.len() as f32;
        let cell_height = screen_height() / board[0].len() as f32;

        clear_background(BLACK);
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                draw_rectangle(
                    cell_width * i as f32,
                    cell_height * j as f32,
                    cell_width - 1.0,
                    cell_height - 1.0,
                    DARKGRAY,
                );
                // if board[i][j].number == 9 {
                let number = board[i][j].number;
                draw_text(
                    &number.to_string(),
                    cell_width * i as f32,
                    cell_height * j as f32 + cell_height,
                    100.0,
                    BLACK,
                )
                // }
            }
        }
        next_frame().await;
    }
}
