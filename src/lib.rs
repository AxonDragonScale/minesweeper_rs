mod cell;

use std::collections::HashSet;

use cell::{Cell, Position};
use rand::{thread_rng, Rng};

#[derive(Debug)]
struct Board {
    grid: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(width: usize, height: usize, num_mines: usize) -> Self {
        let mine_positions = Self::generate_mine_positions(width, height, num_mines);
        let mut grid: Vec<Vec<Cell>> = Vec::with_capacity(height);
        for row in 0..height {
            let mut cur_row: Vec<Cell> = Vec::with_capacity(width);
            for col in 0..width {
                // TODO: Can we calculate adjacent_mines here only, before cell creation?
                // We already know all the mine_positions, should be possible
                let position = Position { row, col };
                let is_mine = mine_positions.contains(&position);
                let cell = Cell::new(position, is_mine);
                cur_row.push(cell);
            }
        }

        Board { grid }
    }

    fn generate_mine_positions(width: usize, height: usize, num_mines: usize) -> HashSet<Position> {
        let mut mine_positions: HashSet<Position> = HashSet::new();

        let mut rng = thread_rng();
        while mine_positions.len() != num_mines {
            mine_positions.insert(Position {
                row: rng.gen_range(0..width),
                col: rng.gen_range(0..height),
            });
        }

        mine_positions
    }
}
