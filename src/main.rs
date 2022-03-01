
#[derive(Debug, Clone, Copy)]
enum FieldType {
    X,
    O,
    Empty
}

impl FieldType {
    fn format(&self) -> String {
        match self {
            FieldType::X => String::from("X"),
            FieldType::O => String::from("O"),
            FieldType::Empty => String::from(" ")
        }
    }
}

struct Board {
    fields: [[FieldType; 3]; 3]
}

impl Board {

    fn empty() -> Board {
        Board {
            fields: [[FieldType::Empty; 3]; 3]
        }
    }

    fn print_board(&self) {
        for row in self.fields.iter() {
            println!("{} | {} | {}", row[0].format(), row[1].format(), row[2].format());
        }
    }
}



fn main() {
    let board = Board::empty();
    board.print_board();
}
