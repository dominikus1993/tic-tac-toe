use std::fmt::{self, Debug, Display};

#[derive(Debug, Clone, Copy)]
pub enum FieldType {
    X,
    O,
    Empty
}

impl FieldType {
    pub fn format(&self) -> String {
        match self {
            FieldType::X => String::from("X"),
            FieldType::O => String::from("O"),
            FieldType::Empty => String::from(" ")
        }
    }
}

pub struct Board {
    pub fields: [[FieldType; 3]; 3]
}

impl Board {

    pub fn empty() -> Board {
        Board {
            fields: [[FieldType::Empty; 3]; 3]
        }
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
}