#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
pub enum CellType {
    Safe { adjacent_mines: usize },
    Mine,
}

#[derive(Debug)]
pub enum CellState {
    None,
    Opened,
    Flagged,
}

#[derive(Debug)]
pub struct Cell {
    pub position: Position,
    pub cell_type: CellType,
    pub cell_state: CellState,
}

impl Cell {
    pub fn new(position: Position, is_mine: bool, adjacent_mines: usize) -> Self {
        Cell {
            position,
            cell_type: if is_mine {
                CellType::Mine
            } else {
                CellType::Safe { adjacent_mines }
            },
            cell_state: CellState::None,
        }
    }
}
