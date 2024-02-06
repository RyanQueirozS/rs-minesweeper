use macroquad::prelude::*;
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
    usize, vec,
};

#[derive(PartialEq, Debug, Copy, Clone)]
struct Cell {
    number: u8,
    is_clicked: bool,
    is_flaged: bool,
}

fn generate_begginer_board() -> Vec<Vec<Cell>> {
    let mut board = vec![vec![Cell {
        number: 0,
        is_clicked: false,
        is_flaged: false,
    }]];

    {
        board.remove(0);
        let mut row = vec![Cell {
            number: 0,
            is_clicked: false,
            is_flaged: false,
        }];
        let cell = Cell {
            number: 0,
            is_clicked: false,
            is_flaged: false,
        };

        for _ in 1..9 {
            // it is initialized with one already
            row.push(cell);
        }
        for _ in 0..9 {
            board.push(row.clone());
        }
    }

    {
        rand::srand(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as u64,
        );
    }
    let mut count = 0;
    loop {
        let x = rand::gen_range(0, board.len());
        let y = rand::gen_range(0, board[x].len());

        if board[x][y].number != 9 {
            board[x][y] = Cell {
                number: 9, // 9 will be the mine number
                is_clicked: false,
                is_flaged: false,
            };
            count += 1;
        }
        if count == 10 {
            break;
        }
    }

    for ix in 0..board.len() {
        for jx in 0..board[ix].len() {
            if board[ix][jx].number != 9 {
                count = 0;
                for x in 0..=2 {
                    for y in 0..=2 {
                        let new_x = (ix as isize + x as isize - 1) as usize;
                        let new_y = (jx as isize + y as isize - 1) as usize;

                        if new_x < board.len()
                            && new_y < board[ix].len()
                            && board[new_x][new_y].number == 9
                        {
                            count += 1;
                        }
                    }
                }
                board[ix][jx] = Cell {
                    number: count,
                    is_clicked: false,
                    is_flaged: false,
                };
            }
        }
    }

    return board;
}

fn generate_intermediate_board() -> Vec<Vec<Cell>> {
    let mut board = vec![vec![Cell {
        number: 0,
        is_clicked: false,
        is_flaged: false,
    }]];

    {
        board.remove(0);
        let mut row = vec![Cell {
            number: 0,
            is_clicked: false,
            is_flaged: false,
        }];
        let cell = Cell {
            number: 0,
            is_clicked: false,
            is_flaged: false,
        };

        for _ in 1..16 {
            // it is initialized with one already
            row.push(cell);
        }
        for _ in 0..16 {
            board.push(row.clone());
        }
    }

    {
        rand::srand(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as u64,
        );
    }
    let mut count = 0;
    loop {
        let x = rand::gen_range(0, board.len());
        let y = rand::gen_range(0, board[x].len());

        if board[x][y].number != 9 {
            board[x][y] = Cell {
                number: 9, // 9 will be the mine number
                is_clicked: false,
                is_flaged: false,
            };
            count += 1;
        }
        if count == 40 {
            // 40 mines
            break;
        }
    }

    for ix in 0..board.len() {
        for jx in 0..board[ix].len() {
            if board[ix][jx].number != 9 {
                count = 0;
                for x in 0..=2 {
                    for y in 0..=2 {
                        let new_x = (ix as isize + x as isize - 1) as usize;
                        let new_y = (jx as isize + y as isize - 1) as usize;

                        if new_x < board.len()
                            && new_y < board[ix].len()
                            && board[new_x][new_y].number == 9
                        {
                            count += 1;
                        }
                    }
                }
                board[ix][jx] = Cell {
                    number: count,
                    is_clicked: false,
                    is_flaged: false,
                };
            }
        }
    }

    return board;
}
fn generate_hard_board() -> Vec<Vec<Cell>> {
    let mut board = vec![vec![Cell {
        number: 0,
        is_clicked: false,
        is_flaged: false,
    }]];

    {
        board.remove(0);
        let mut row = vec![Cell {
            number: 0,
            is_clicked: false,
            is_flaged: false,
        }];
        let cell = Cell {
            number: 0,
            is_clicked: false,
            is_flaged: false,
        };

        for _ in 1..16 {
            // it is initialized with one already
            row.push(cell);
        }
        for _ in 0..30 {
            board.push(row.clone());
        }
    }

    {
        rand::srand(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as u64,
        );
    }
    let mut count = 0;
    loop {
        let x = rand::gen_range(0, board.len());
        let y = rand::gen_range(0, board[x].len());

        if board[x][y].number != 9 {
            board[x][y] = Cell {
                number: 9, // 9 will be the mine number
                is_clicked: false,
                is_flaged: false,
            };
            count += 1;
        }
        if count == 99 {
            //  99 mines
            break;
        }
    }

    for ix in 0..board.len() {
        for jx in 0..board[ix].len() {
            if board[ix][jx].number != 9 {
                count = 0;
                for x in 0..=2 {
                    for y in 0..=2 {
                        let new_x = (ix as isize + x as isize - 1) as usize;
                        let new_y = (jx as isize + y as isize - 1) as usize;

                        if new_x < board.len()
                            && new_y < board[ix].len()
                            && board[new_x][new_y].number == 9
                        {
                            count += 1;
                        }
                    }
                }
                board[ix][jx] = Cell {
                    number: count,
                    is_clicked: false,
                    is_flaged: false,
                };
            }
        }
    }

    return board;
}

async fn draw_board(board: &mut Vec<Vec<Cell>>) {
    let cell_width = screen_width() / board.len() as f32;
    let cell_height = screen_height() / board[0].len() as f32;

    let bomb = load_texture("./src/assets/bomb.png").await.unwrap();
    let flag = load_texture("./src/assets/flag.png").await.unwrap();
    bomb.set_filter(FilterMode::Nearest);
    flag.set_filter(FilterMode::Nearest);
    let text_params = DrawTextureParams {
        dest_size: Some(vec2(cell_width - 10.0, cell_height - 10.0)),
        ..Default::default()
    };

    for x in 0..board.len() {
        for y in 0..board[x].len() {
            draw_rectangle(
                cell_width * x as f32,
                cell_height * y as f32,
                cell_width - 1.0,
                cell_height - 1.0,
                DARKGRAY,
            );

            let number = board[x][y].number;
            if board[x][y].is_clicked {
                if board[x][y].number == 9 {
                    // i don't get why i some times need to invert xand y
                    draw_texture_ex(
                        &bomb,
                        cell_width * x as f32,
                        cell_height * y as f32,
                        WHITE,
                        text_params.clone(),
                    );
                } else {
                    let color: Color;
                    match number {
                        1 => color = DARKGREEN,
                        2 => color = DARKBLUE,
                        3 => color = VIOLET,
                        4 => color = PINK,
                        5 => color = MAROON,
                        6 => color = BROWN,
                        7 => color = YELLOW,
                        8 => color = RED,
                        _ => color = BLACK,
                    }
                    draw_rectangle(
                        cell_width * x as f32,
                        cell_height * y as f32,
                        cell_width - 1.0,
                        cell_height - 1.0,
                        GRAY,
                    );
                    if number != 0 {
                        draw_text(
                            &number.to_string(),
                            cell_width * x as f32,
                            cell_height * y as f32 + cell_height,
                            cell_width,
                            color,
                        )
                    }
                }
            }
            if board[x][y].is_flaged {
                draw_texture_ex(
                    &flag,
                    cell_width * x as f32,
                    cell_height * y as f32,
                    WHITE,
                    text_params.clone(),
                );
            }
        }
    }
}

async fn handle_input(board: &mut Vec<Vec<Cell>>) {
    let cell_width = screen_width() / board.len() as f32;
    let cell_height = screen_height() / board[0].len() as f32;

    if is_mouse_button_pressed(MouseButton::Left) {
        let _mouse_pos = mouse_position();
        let mousex = (_mouse_pos.0 / cell_width) as usize;
        let mousey = (_mouse_pos.1 / cell_height) as usize;
        board[mousex][mousey].is_clicked = true;

        // Handle bombs
        if board[mousex][mousey].is_flaged == true {
            return;
        }
        if board[mousex][mousey].number == 9 {
            // mine
            for x in 0..board.len() {
                for y in 0..board[x].len() {
                    board[x][y].is_clicked = true;
                    board[x][y].is_flaged = false;
                }
            }
        }
    }

    if is_mouse_button_pressed(MouseButton::Right) {
        let _mouse_pos = mouse_position();
        let mousex = (_mouse_pos.0 / cell_width) as usize;
        let mousey = (_mouse_pos.1 / cell_height) as usize;

        if !board[mousex][mousey].is_clicked {
            if board[mousex][mousey].is_flaged {
                board[mousex][mousey].is_flaged = false;
            } else {
                board[mousex][mousey].is_flaged = true;
            }
        }
    }
}

#[macroquad::main("Minesweeper")]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let mut board: Vec<Vec<Cell>> = generate_begginer_board();
    if args.len() > 1 {
        if args[1] == "intermediate" {
            board = generate_intermediate_board();
        }
        if args[1] == "hard" {
            board = generate_hard_board();
        }
    }

    request_new_screen_size(1600.0, 800.0);
    let mut _mouse_pos: (f32, f32);

    clear_background(BLACK);
    loop {
        handle_input(&mut board).await;
        draw_board(&mut board).await;

        next_frame().await;
    }
}
