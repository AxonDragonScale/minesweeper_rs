use std::io::{stdout, Write};

use minesweeper_rs::{ Board, cell::Position };

fn main() {
    let mut board = Board::new(5, 5, 10);
    println!("{board}");
    board.open_cell(&Position {row: 1, col:3});
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush();
    println!("{board}");
    board.open_cell(&Position {row: 3, col:3});
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush();
    println!("{board}");
}