use std::{fmt::{self, Debug, Display}};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Errors {
    InvalidCoordinatres,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FieldType {
    X,
    O,
    Empty
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate(pub usize, pub usize);

impl FieldType {
    pub fn format(&self) -> String {
        match self {
            FieldType::X => String::from("X"),
            FieldType::O => String::from("O"),
            FieldType::Empty => String::from(" ")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Board {
    fields: [[FieldType; 3]; 3]
}

impl Board {

    pub fn empty() -> Board {
        Board {
            fields: [[FieldType::Empty; 3]; 3]
        }
    }

    fn move_is_valid(&self, coordinate: Coordinate) -> bool {
        let Coordinate(x, y) = coordinate;
        x < 3 && y < 3 && self.fields[x][y] == FieldType::Empty
    }

    pub fn set_field(&self, coordinate: Coordinate, field_type: FieldType) -> Result<Board, Errors> {
        let Coordinate(x, y) = coordinate;
        if self.move_is_valid(coordinate) {
            let mut arr = self.fields;
            arr[x][y] = field_type;
            return Ok(Board{ fields: arr })
        }
        Err(Errors::InvalidCoordinatres)
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
    fn test_format_non_empty_board() {
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
        let mut board = Board::empty();
        board.fields[0][0] = FieldType::X;
        assert!(!board.move_is_valid(Coordinate(0, 0)));
    }

    #[test]
    fn test_set_field() {
        let mut board = Board::empty();
        let subject = board.set_field(Coordinate(0, 0), FieldType::X).unwrap();
        assert_eq!(FieldType::X, subject.fields[0][0]);
    }

    #[test]
    fn test_set_field_when_coordinates_is_invalid() {
        let mut board = Board::empty();
        let result = board.set_field(Coordinate(0, 5), FieldType::X);
        assert!(result.is_err());
    }
}