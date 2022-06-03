mod util;

use std::{io::{stdin, stdout, Write}, thread::sleep, time::Duration};

use crate::util::clear_screen;

use minesweeper_rs::{cell::Position, Board};

fn main() {
    let mut board = Board::new(9, 9, None);

    board = Board::new(9, 9, None);

    loop {
        print_board_and_prompt(&board);        

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command_vec: Vec<&str> = input.split_ascii_whitespace().collect();
        match command_vec[0] {
            "exit" => return,
            "open" => open(&mut board, command_vec),
            "flag" | "unflag" => flag(&mut board, command_vec),
            "reset" => reset(&mut board, command_vec),
            _ => continue,
        }
    }
}

fn print_board_and_prompt(board: &Board) {
    clear_screen();

    println!("-------------------------------- minesweeper_rs --------------------------------\n\n");
    println!("{board}\n");
    print!("-> ");

    stdout().flush().unwrap();
}

fn open(board: &mut Board, command_vec: Vec<&str>) {
    if command_vec.len() < 3 {
        println!("Invalid Command");
        return;
    }

    board.open_cell(&get_position(command_vec));
}

fn flag(board: &mut Board, command_vec: Vec<&str>) {
    if command_vec.len() < 3 {
        println!("Invalid Command");
        return;
    }

    board.toggle_flag(&get_position(command_vec));
}

fn get_position(command_vec: Vec<&str>) -> Position {
    let col = command_vec[1].chars().nth(0).unwrap() as usize - 'A' as usize;
    let row = command_vec[2].parse::<usize>().unwrap() - 1;
    Position { row, col }
}

fn reset(board: &mut Board, command_vec: Vec<&str>) {
    if command_vec.len() < 3 {
        println!("Invalid Command");
        return;
    }

    let width: usize = command_vec[1].parse().unwrap();
    let height: usize = command_vec[1].parse().unwrap();
    let num_mines: Option<usize> = command_vec[1].parse().ok();

    board.reset(width, height, num_mines);
}

