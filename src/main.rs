mod game;
use game::*;

fn main() {
    let mut board = Board::empty();
    board.fields[0][0] = FieldType::X;
    board.fields[0][1] = FieldType::O;
    board.fields[0][2] = FieldType::X;
    let board_str = board.format_board();

    println!("{}", board_str);
}
