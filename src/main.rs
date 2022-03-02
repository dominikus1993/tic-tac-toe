mod game;
use game::*;

type Result<T> = std::result::Result<T, Errors>;

fn main() -> Result<()>{
    let board = Board::empty();
    let board = board.move_next(Coordinate(0,1), FieldType::O)?;
    let board = board.move_next(Coordinate(1,1), FieldType::O)?;
    let board = board.move_next(Coordinate(2,1), FieldType::O)?;
    let board_str = board.format_board();

    println!("{}", board_str);
    Ok(())
}
