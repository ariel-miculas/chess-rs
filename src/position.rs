use super::{MoveError, Result};
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct Position {
    row: usize,
    column: usize,
}
impl Position {
    pub fn try_new(row: usize, column: usize) -> Result<Self> {
        if row < 8 && column < 8 {
            Ok(Self { row, column })
        } else {
            Err(MoveError)
        }
    }

    pub fn get_column(&self) -> usize {
        self.column
    }

    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn try_add(&self, pos: (isize, isize)) -> Result<Self> {
        let new_row = self.row as isize + pos.0;
        let new_col = self.column as isize + pos.1;

        if new_row >= 0 && new_col >= 0 {
            Self::try_new(new_row as usize, new_col as usize)
        } else {
            Err(MoveError)
        }
    }

    pub fn get_left_squares(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        for i in (0..self.column).rev() {
            positions.push(Position::try_new(self.row, i).unwrap());
        }
        positions
    }

    pub fn get_right_squares(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        for i in self.column + 1..8 {
            positions.push(Position::try_new(self.row, i).unwrap());
        }
        positions
    }

    pub fn get_down_squares(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        for i in (0..self.row).rev() {
            positions.push(Position::try_new(i, self.column).unwrap());
        }
        positions
    }

    pub fn get_up_squares(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        for i in self.row + 1..8 {
            positions.push(Position::try_new(i, self.column).unwrap());
        }
        positions
    }

    pub fn get_vertical_squares(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        for i in 0..8 {
            if i != self.row {
                positions.push(Position::try_new(i, self.column).unwrap());
            }
        }
        positions
    }

    pub fn get_horizontal_squares(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        for i in 0..8 {
            if i != self.column {
                positions.push(Position::try_new(self.row, i).unwrap());
            }
        }
        positions
    }

    pub fn get_principal_diagonal_squares(&self) -> Vec<Position> {
        todo!("not implemented")
    }

    pub fn get_secondary_diagonal_squares(&self) -> Vec<Position> {
        todo!("not implemented")
    }

    pub fn get_principal_diagonal_up_squares(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        let mut row = self.row;
        let mut column = self.column;

        loop {
            if row == 7 || column == 7 {
                break;
            }
            row += 1;
            column += 1;
            positions.push(Position::try_new(row, column).unwrap());
        }

        positions
    }

    pub fn get_secondary_diagonal_up_squares(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        let mut row = self.row;
        let mut column = self.column;

        loop {
            if row == 0 || column == 7 {
                break;
            }
            row -= 1;
            column += 1;
            positions.push(Position::try_new(row, column).unwrap());
        }

        positions
    }

    pub fn get_principal_diagonal_down_squares(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        let mut row = self.row;
        let mut column = self.column;

        loop {
            if row == 0 || column == 0 {
                break;
            }
            row -= 1;
            column -= 1;
            positions.push(Position::try_new(row, column).unwrap());
        }

        positions
    }

    pub fn get_secondary_diagonal_down_squares(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        let mut row = self.row;
        let mut column = self.column;

        loop {
            if row == 7 || column == 0 {
                break;
            }
            row += 1;
            column -= 1;
            positions.push(Position::try_new(row, column).unwrap());
        }

        positions
    }

    pub fn get_surrounding_squares(&self) -> Vec<Position> {
        let positions = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 0),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        positions
            .into_iter()
            .filter_map(|pos| self.try_add(pos).ok())
            .collect::<Vec<Position>>()
    }
}
