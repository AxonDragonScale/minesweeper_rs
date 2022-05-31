
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
    position: Position,
    cell_type: CellType,
    cell_state: CellState,
}

impl Cell {
    pub fn new(position: Position, is_mine: bool) -> Self {
        Cell {
            position,
            cell_type: if is_mine {
                CellType::Mine
            } else {
                // Initially set adjacent_mines to 0
                CellType::Safe { adjacent_mines: 0 }
            },
            cell_state: CellState::None,
        }
    }
}
