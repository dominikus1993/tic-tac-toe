use std::{fmt::{self, Debug, Display}, f32::consts::E, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Errors {
    InvalidCoordinatres,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    X,
    O,
    Empty
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResults {
    XWon,
    OWon,
    Draw,
    InProgress
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate(pub usize, pub usize);

impl FieldType {
    pub fn format(&self) -> &str {
        match self {
            FieldType::X => "X",
            FieldType::O => "O",
            FieldType::Empty => " "
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Board {
    fields: [[FieldType; 3]; 3],
    move_count: usize
}

impl Board {

    pub fn empty() -> Board {
        Board {
            fields: [[FieldType::Empty; 3]; 3],
            move_count: 0
        }
    }

    fn move_is_valid(&self, coordinate: Coordinate) -> bool {
        let Coordinate(x, y) = coordinate;
        x < 3 && y < 3 && self.fields[x][y] == FieldType::Empty
    }

    pub fn move_next(&self, coordinate: Coordinate, field_type: FieldType) -> Result<Board, Errors> {
        let Coordinate(x, y) = coordinate;
        if self.move_is_valid(coordinate) {
            let mut arr = self.fields;
            arr[x][y] = field_type;
            return Ok(Board{ fields: arr, move_count: self.move_count + 1 });
        }
        Err(Errors::InvalidCoordinatres)
    }

    fn check_result(&self, (x_count, o_count, empty_field_count): (i32, i32, i32)) -> GameResults { 
        if x_count == 3 {
            return GameResults::XWon;
        }
        if o_count == 3 {
            return GameResults::OWon;
        }
        if empty_field_count == 0 {
            return GameResults::Draw;
        }
        GameResults::InProgress
    }

    fn check_colums(&self) -> GameResults {
        let mut x_count = 0;
        let mut o_count = 0;
        let mut empty_field_count = 0;
        for column in 0..3 {
            for row in 0..3 {
                match self.fields[row][column] {
                    FieldType::X => x_count += 1,
                    FieldType::O => o_count += 1,
                    FieldType::Empty => empty_field_count += 1
                }
            }
            let result = self.check_result((x_count, o_count, empty_field_count));
            if result != GameResults::InProgress || result != GameResults::Draw {
                return result;
            }
            x_count = 0;
            o_count = 0;
            empty_field_count = 0;
        }

        return self.check_result((x_count, o_count, empty_field_count));

    }

    pub fn check_game_result(&self) -> GameResults {
        return GameResults::XWon
    }

    pub fn format_board(&self) -> String {
        let mut res = String::from("");
        for (i, row) in self.fields.iter().enumerate() {
            let text = format!(" {} | {} | {} \n", row[0].format(), row[1].format(), row[2].format());
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
    fn test_format_non_empty_board() {
        let expected = r#" X | X | X
---+---+---
   |   |   
---+---+---
   |   |   
"#;
        let mut board = Board::empty();
        board.fields[0][0] = FieldType::X;
        board.fields[1][0] = FieldType::X;
        board.fields[2][0] = FieldType::X;
        let subject = board.format_board();
        assert_eq!(expected, subject);
    }

    #[test]
    fn test_move_is_valid() {
        let board = Board::empty();
        assert!(board.move_is_valid(Coordinate(0, 0)));
        assert!(board.move_is_valid(Coordinate(0, 1)));
        assert!(board.move_is_valid(Coordinate(0, 2)));
        assert!(board.move_is_valid(Coordinate(1, 0)));
        assert!(board.move_is_valid(Coordinate(1, 1)));
        assert!(board.move_is_valid(Coordinate(1, 2)));
        assert!(board.move_is_valid(Coordinate(2, 0)));
        assert!(board.move_is_valid(Coordinate(2, 1)));
        assert!(board.move_is_valid(Coordinate(2, 2)));
    }

    #[test]
    fn test_move_is_valid_when_coordinate_is_out_of_bound(){
        let board = Board::empty();
        assert!(!board.move_is_valid(Coordinate(3, 0)));
        assert!(!board.move_is_valid(Coordinate(0, 3)));
        assert!(!board.move_is_valid(Coordinate(3, 3)));
    }

    #[test]
    fn test_move_is_valid_when_coordinate_is_not_empty(){
        let board = Board::empty();
        assert!(!board.move_is_valid(Coordinate(0, 0)));
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
        let mut board = Board::empty();
        board.fields[0][0] = FieldType::X;
        board.fields[1][0] = FieldType::X;
        board.fields[2][0] = FieldType::X;
        assert_eq!(board.check_colums(), GameResults::XWon);
    }

    #[test]
    fn test_check_columns_result_when_o_wins() {
        let mut board = Board::empty();
        board.fields[0][1] = FieldType::O;
        board.fields[1][1] = FieldType::O;
        board.fields[2][1] = FieldType::O;
        assert_eq!(board.check_colums(), GameResults::OWon);
    }
}