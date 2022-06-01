mod cell;

use std::{collections::HashSet, fmt::Display};

use cell::{Cell, Position, CellState, CellType};
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Board {
    grid: Vec<Vec<Cell>>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for col in row {
                match col.cell_state {
                    cell::CellState::None => write!(f, "🟦 ")?,
                    cell::CellState::Opened => {
                        match col.cell_type {
                            cell::CellType::Safe { adjacent_mines } => write!(f, "{adjacent_mines}  ")?,
                            cell::CellType::Mine => write!(f, "💣 ")?,
                        }
                    },
                    cell::CellState::Flagged => write!(f, "🚩 ")?,
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Board {
    pub fn new(width: usize, height: usize, num_mines: usize) -> Self {
        let mine_positions = Self::generate_mine_positions(width, height, num_mines);
        let mut grid: Vec<Vec<Cell>> = Vec::with_capacity(height);
        for row in 0..height {
            let mut cur_row: Vec<Cell> = Vec::with_capacity(width);
            for col in 0..width {
                let position = Position { row, col };
                let adjacent_mines =
                    Self::calculate_adjacent_mines(&position, &mine_positions, width, height);
                let is_mine = mine_positions.contains(&position);
                let cell = Cell::new(position, is_mine, adjacent_mines);
                cur_row.push(cell);
            }
            grid.push(cur_row);
        }

        Board { grid }
    }

    pub fn open_cell(&mut self, position: &Position) -> Option<&CellType> {
        let cell = &mut self.grid[position.row][position.col];
        return match cell.cell_state {
            CellState::None | CellState::Opened => {
                cell.cell_state = CellState::Opened;
                Some(&cell.cell_type)
            },
            CellState::Flagged => None,
        }
    }

    pub fn toggle_flag(&mut self, position: &Position) {
        let cell = &mut self.grid[position.row][position.col];
        cell.cell_state = match cell.cell_state {
            CellState::None => CellState::Flagged,
            CellState::Flagged => CellState::None,
            CellState::Opened => CellState::Opened,
        }
    }

    fn generate_mine_positions(width: usize, height: usize, num_mines: usize) -> HashSet<Position> {
        assert!(num_mines <= width * height, "Number of mines cannot be more than {width} * {height}");
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

    fn calculate_adjacent_mines(
        position: &Position,
        mine_positions: &HashSet<Position>,
        width: usize,
        height: usize,
    ) -> usize {
        let row_range = (position.row.max(1) - 1)..=((position.row + 1).min(height - 1));
        row_range.flat_map(move |row| {
            let col_range = (position.col.max(1) - 1)..=((position.col + 1).min(width - 1));
            col_range.map(move |col| Position { row, col } )
        })
        .filter(|p| p != position && mine_positions.contains(p))
        .count()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Board, cell::Position};

    #[test]
    fn basic_board() {
        let mut board = Board::new(10, 10, 10);
        board.open_cell(&Position { row: 1, col: 1 });
        board.toggle_flag(&Position { row: 2, col: 2 });
        println!("\n{board}");
    }
}