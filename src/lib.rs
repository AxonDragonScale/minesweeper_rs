pub mod cell;

use std::{collections::HashSet, fmt::Display, char::from_u32};

use cell::{Cell, Position, CellState, CellType};
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Board {
    grid: Vec<Vec<Cell>>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "   ")?;
        for i in 0..(self.grid[0].len()) {
            write!(f, "{: ^5}", from_u32(i as u32 + 'A' as u32).unwrap())?;
        }
        write!(f, "\n")?;

        for (i, row) in self.grid.iter().enumerate() {
            write!(f, "   ")?;
            for _ in 0..(row.len()) { write!(f, "-----")?; }
            write!(f, "-\n{: <3}", i+1)?;

            for col in row {
                write!(f, "|")?;
                match col.cell_state {
                    cell::CellState::None => write!(f, "{: ^3}", "🟦")?,
                    cell::CellState::Opened => {
                        match col.cell_type {
                            cell::CellType::Safe { adjacent_mines } => write!(f, "{: ^4}", adjacent_mines)?,
                            cell::CellType::Mine => write!(f, "{: ^3}", "💣")?,
                        }
                    },
                    cell::CellState::Flagged => write!(f, "{: ^3}", "🚩")?,
                }
            }
            write!(f, "|{: >3}\n", i+1)?;

        }
        write!(f, "   ")?;
        for _ in 0..(self.grid[0].len()) { write!(f, "-----")?; }
        write!(f, "-\n")?;

        write!(f, "   ")?;
        for i in 0..(self.grid[0].len()) {
            write!(f, "{: ^5}", from_u32(i as u32 + 'A' as u32).unwrap())?;
        }
        write!(f, "\n")?;

        Ok(())
    }
}

impl Board {
    pub fn new(width: usize, height: usize, num_mines: Option<usize>) -> Self {
        let num_mines = num_mines.unwrap_or(width * height / 4);
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

    pub fn reset(&mut self, width: usize, height: usize, num_mines: Option<usize>) {
        self.grid = Self::new(width, height, num_mines).grid;
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
        let mut board = Board::new(10, 10, None);
        board.open_cell(&Position { row: 1, col: 1 });
        board.toggle_flag(&Position { row: 2, col: 2 });
    }
}