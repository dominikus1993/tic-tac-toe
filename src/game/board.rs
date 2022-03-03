use std::{
    collections::HashMap,
    f32::consts::E,
    fmt::{self, Debug, Display}, str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Errors {
    InvalidCoordinatres,
    ParseCoordinatesError
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    X,
    O,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResults {
    XWon,
    OWon,
    Draw,
    InProgress,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate(pub usize, pub usize);


impl FromStr for Coordinate {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        let x = split.next().ok_or(Errors::ParseCoordinatesError)?;
        let y = split.next().ok_or(Errors::ParseCoordinatesError)?;
        let x = x.parse::<usize>().map_err(|_| Errors::ParseCoordinatesError)?;
        let y = y.parse::<usize>().map_err(|_| Errors::ParseCoordinatesError)?;
        Ok(Coordinate(x, y))
    }
}


impl FieldType {
    pub fn format(&self) -> &str {
        match self {
            FieldType::X => "X",
            FieldType::O => "O",
            FieldType::Empty => " ",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Board {
    fields: [[FieldType; 3]; 3],
    move_count: usize,
    n: usize,
}

impl Board {
    pub fn empty() -> Board {
        Board {
            fields: [[FieldType::Empty; 3]; 3],
            move_count: 0,
            n: 3,
        }
    }

    fn get(&self, coord: Coordinate) -> FieldType {
        let Coordinate(x, y) = coord;
        self.fields[y][x]
    }

    pub fn is_move_valid(&self, coordinate: Coordinate) -> bool {
        let Coordinate(x, y) = coordinate;
        x < self.n && y < self.n && self.get(coordinate) == FieldType::Empty
    }

    pub fn get_free_coordinates(&self) -> Vec<Coordinate> {
        let mut free_coordinates = Vec::new();
        for y in 0..self.n {
            for x in 0..self.n {
                if self.get(Coordinate(x, y)) == FieldType::Empty {
                    free_coordinates.push(Coordinate(x, y));
                }
            }
        }
        free_coordinates
    }

    pub fn move_next(
        &self,
        coordinate: Coordinate,
        field_type: FieldType,
    ) -> Result<Board, Errors> {
        let Coordinate(x, y) = coordinate;
        if self.is_move_valid(coordinate) {
            let mut arr = self.fields;
            arr[y][x] = field_type;
            return Ok(Board {
                fields: arr,
                move_count: self.move_count + 1,
                n: self.n,
            });
        }
        Err(Errors::InvalidCoordinatres)
    }

    fn check_result(&self, (x_count, o_count): (usize, usize)) -> GameResults {
        if x_count == self.n {
            return GameResults::XWon;
        }
        if o_count == self.n {
            return GameResults::OWon;
        }
        GameResults::InProgress
    }

    fn check_colums(&self) -> GameResults {
        let mut x_count = 0;
        let mut o_count = 0;
        for i in 0..self.n {
            for j in 0..self.n {
                match self.get(Coordinate(i, j)) {
                    FieldType::X => x_count += 1,
                    FieldType::O => o_count += 1,
                    FieldType::Empty => {}
                }
            }
            let result = self.check_result((x_count, o_count));
            if result != GameResults::InProgress {
                return result;
            }
            x_count = 0;
            o_count = 0;
        }

        self.check_result((x_count, o_count))
    }

    fn check_rows(&self) -> GameResults {
        let mut x_count = 0;
        let mut o_count = 0;
        for i in 0..self.n {
            for j in 0..self.n {
                match self.get(Coordinate(j, i)) {
                    FieldType::X => x_count += 1,
                    FieldType::O => o_count += 1,
                    FieldType::Empty => {}
                }
            }
            let result = self.check_result((x_count, o_count));
            if result != GameResults::InProgress {
                return result;
            }
            x_count = 0;
            o_count = 0;
        }

        self.check_result((x_count, o_count))
    }

    fn check_diag(&self) -> GameResults {
        let mut x_count = 0;
        let mut o_count = 0;
        for i in 0..3 {
            match self.get(Coordinate(i, i)) {
                FieldType::X => x_count += 1,
                FieldType::O => o_count += 1,
                FieldType::Empty => {}
            }
        }

        let result = self.check_result((x_count, o_count));
        if result != GameResults::InProgress {
            return result;
        }

        x_count = 0;
        o_count = 0;

        for i in 0..self.n {
            match self.get(Coordinate(i, (self.n - 1) - i)) {
                FieldType::X => x_count += 1,
                FieldType::O => o_count += 1,
                FieldType::Empty => {}
            }
        }

        self.check_result((x_count, o_count))
    }

    pub fn check_game_result(&self) -> GameResults {
        let columns_result = self.check_colums();
        if columns_result != GameResults::InProgress {
            return columns_result;
        }
        let rows_result = self.check_rows();
        if rows_result != GameResults::InProgress {
            return rows_result;
        }
        let diag_result = self.check_diag();
        if diag_result != GameResults::InProgress {
            return diag_result;
        }
        if self.move_count == (self.n * self.n) {
            return GameResults::Draw;
        }

        GameResults::InProgress
    }

    pub fn format_board(&self) -> String {
        let mut res = String::from("");
        for (i, row) in self.fields.iter().enumerate() {
            let text = format!(
                " {} | {} | {} \n",
                row[0].format(),
                row[1].format(),
                row[2].format()
            );
            res.push_str(&text);
            if i < 2 {
                res.push_str("---+---+---\n");
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_format_empty_board() {
        let expected = r#"   |   |   
---+---+---
   |   |   
---+---+---
   |   |   
"#;
        let board = Board::empty();
        let subject = board.format_board();
        assert_eq!(expected, subject);
    }

    #[test]
    fn test_format_non_empty_board_diag() {
        let expected = r#" X |   |   
---+---+---
   | O |   
---+---+---
   |   | X 
"#;
        let mut board = Board::empty();
        board.fields[0][0] = FieldType::X;
        board.fields[1][1] = FieldType::O;
        board.fields[2][2] = FieldType::X;
        let subject = board.format_board();
        assert_eq!(expected, subject);
    }

    #[test]
    fn test_format_non_empty_board_first_row() {
        let expected = r#" X | X | X 
---+---+---
   |   |   
---+---+---
   |   |   
"#;
        let board = Board::empty();
        let board = board.move_next(Coordinate(0, 0), FieldType::X).unwrap();
        let board = board.move_next(Coordinate(1, 0), FieldType::X).unwrap();
        let board = board.move_next(Coordinate(2, 0), FieldType::X).unwrap();
        let subject = board.format_board();
        assert_eq!(expected, subject);
    }

    #[test]
    fn test_format_non_empty_board_first_column() {
        let expected = r#" X |   |   
---+---+---
 X |   |   
---+---+---
 X |   |   
"#;
        let board = Board::empty();
        let board = board.move_next(Coordinate(0, 0), FieldType::X).unwrap();
        let board = board.move_next(Coordinate(0, 1), FieldType::X).unwrap();
        let board = board.move_next(Coordinate(0, 2), FieldType::X).unwrap();
        let subject = board.format_board();
        assert_eq!(expected, subject);
    }

    #[test]
    fn test_move_is_valid() {
        let board = Board::empty();
        assert!(board.is_move_valid(Coordinate(0, 0)));
        assert!(board.is_move_valid(Coordinate(0, 1)));
        assert!(board.is_move_valid(Coordinate(0, 2)));
        assert!(board.is_move_valid(Coordinate(1, 0)));
        assert!(board.is_move_valid(Coordinate(1, 1)));
        assert!(board.is_move_valid(Coordinate(1, 2)));
        assert!(board.is_move_valid(Coordinate(2, 0)));
        assert!(board.is_move_valid(Coordinate(2, 1)));
        assert!(board.is_move_valid(Coordinate(2, 2)));
    }

    #[test]
    fn test_move_is_valid_when_coordinate_is_out_of_bound() {
        let board = Board::empty();
        assert!(!board.is_move_valid(Coordinate(3, 0)));
        assert!(!board.is_move_valid(Coordinate(0, 3)));
        assert!(!board.is_move_valid(Coordinate(3, 3)));
    }

    #[test]
    fn test_move_is_valid_when_coordinate_is_not_empty() {
        let board = Board::empty();
        let board = board.move_next(Coordinate(0, 0), FieldType::X).unwrap();
        assert!(!board.is_move_valid(Coordinate(0, 0)));
    }

    #[test]
    fn test_set_field() {
        let board = Board::empty();
        let subject = board.move_next(Coordinate(0, 0), FieldType::X).unwrap();
        assert_eq!(FieldType::X, subject.fields[0][0]);
        assert_eq!(subject.move_count, 1);
    }

    #[test]
    fn test_set_field_when_coordinates_is_invalid() {
        let board = Board::empty();
        let result = board.move_next(Coordinate(0, 5), FieldType::X);
        assert!(result.is_err());
    }

    #[test]
    fn test_check_colums_result() {
        let board = Board::empty();
        let board = board.move_next(Coordinate(0, 0), FieldType::X).unwrap();
        let board = board.move_next(Coordinate(0, 1), FieldType::X).unwrap();
        let board = board.move_next(Coordinate(0, 2), FieldType::X).unwrap();
        assert_eq!(board.check_colums(), GameResults::XWon);
    }

    #[test]
    fn test_check_columns_result_when_o_wins() {
        let board = Board::empty();
        let board = board.move_next(Coordinate(1, 0), FieldType::O).unwrap();
        let board = board.move_next(Coordinate(1, 1), FieldType::O).unwrap();
        let board = board.move_next(Coordinate(1, 2), FieldType::O).unwrap();
        assert_eq!(board.check_colums(), GameResults::OWon);
    }

    #[test]
    fn test_check_columns_result_when_in_progress() {
        let board = Board::empty();
        let board = board.move_next(Coordinate(1, 0), FieldType::O).unwrap();
        let board = board.move_next(Coordinate(1, 1), FieldType::O).unwrap();
        assert_eq!(board.check_colums(), GameResults::InProgress);
    }

    #[test]
    fn test_check_rows_result() {
        let board = Board::empty();
        let board = board.move_next(Coordinate(0, 0), FieldType::X).unwrap();
        let board = board.move_next(Coordinate(1, 0), FieldType::X).unwrap();
        let board = board.move_next(Coordinate(2, 0), FieldType::X).unwrap();
        assert_eq!(board.check_rows(), GameResults::XWon);
    }

    #[test]
    fn test_check_crows_result_when_o_wins() {
        let board = Board::empty();
        let board = board.move_next(Coordinate(0, 1), FieldType::O).unwrap();
        let board = board.move_next(Coordinate(1, 1), FieldType::O).unwrap();
        let board = board.move_next(Coordinate(2, 1), FieldType::O).unwrap();
        assert_eq!(board.check_rows(), GameResults::OWon);
    }

    #[test]
    fn test_check_rows_result_when_in_progress() {
        let board = Board::empty();
        let board = board.move_next(Coordinate(1, 0), FieldType::O).unwrap();
        let board = board.move_next(Coordinate(1, 1), FieldType::O).unwrap();
        assert_eq!(board.check_rows(), GameResults::InProgress);
    }

    //dssd

    #[test]
    fn test_check_diag_result() {
        let board = Board::empty();
        let board = board.move_next(Coordinate(0, 0), FieldType::X).unwrap();
        let board = board.move_next(Coordinate(1, 1), FieldType::X).unwrap();
        let board = board.move_next(Coordinate(2, 2), FieldType::X).unwrap();
        assert_eq!(board.check_diag(), GameResults::XWon);
    }

    #[test]
    fn test_check_diag_result_when_o_wins() {
        let board = Board::empty();
        let board = board.move_next(Coordinate(2, 0), FieldType::O).unwrap();
        let board = board.move_next(Coordinate(1, 1), FieldType::O).unwrap();
        let board = board.move_next(Coordinate(0, 2), FieldType::O).unwrap();
        assert_eq!(board.check_diag(), GameResults::OWon);
    }

    #[test]
    fn test_check_diag_result_when_in_progress() {
        let board = Board::empty();
        let board = board.move_next(Coordinate(0, 0), FieldType::O).unwrap();
        let board = board.move_next(Coordinate(1, 1), FieldType::O).unwrap();
        assert_eq!(board.check_diag(), GameResults::InProgress);
    }

    #[test]
    fn test_draw_result() {
        let board = Board::empty();
        let board = board.move_next(Coordinate(0,0), FieldType::O).unwrap();
        let board = board.move_next(Coordinate(1,0), FieldType::X).unwrap();
        let board = board.move_next(Coordinate(2,0), FieldType::O).unwrap();
        let board = board.move_next(Coordinate(1,1), FieldType::X).unwrap();
        let board = board.move_next(Coordinate(2,1), FieldType::O).unwrap();
        let board = board.move_next(Coordinate(0,2), FieldType::X).unwrap();
        let board = board.move_next(Coordinate(1,2), FieldType::O).unwrap();
        let board = board.move_next(Coordinate(2,2), FieldType::X).unwrap();
        let board = board.move_next(Coordinate(0,1), FieldType::O).unwrap();
        assert_eq!(board.check_game_result(), GameResults::Draw);
    }
}
