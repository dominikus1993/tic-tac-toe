mod board;
use board::*;

type Result<T> = std::result::Result<T, Errors>;

fn main() -> Result<()>{
    let board = Board::empty();
    let board = board.move_next(Coordinate(0,0), FieldType::O)?;
    let board = board.move_next(Coordinate(1,0), FieldType::X)?;
    let board = board.move_next(Coordinate(2,0), FieldType::O)?;
    let board = board.move_next(Coordinate(1,1), FieldType::X)?;
    let board = board.move_next(Coordinate(2,1), FieldType::O)?;
    let board = board.move_next(Coordinate(0,2), FieldType::X)?;
    let board = board.move_next(Coordinate(1,2), FieldType::O)?;
    let board = board.move_next(Coordinate(2,2), FieldType::X)?;
    let board = board.move_next(Coordinate(0,1), FieldType::O)?;
    let board_str = board.format_board();

    println!("{}", board_str);

    println!("{:?}", board.check_game_result());
    Ok(())
}
