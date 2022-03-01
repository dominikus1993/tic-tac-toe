mod game;
use game::*;

type Result<T> = std::result::Result<T, Errors>;

fn main() -> Result<()>{
    let board = Board::empty();
    let a = board.set_field(Coordinate(0,0), FieldType::X)?;
    let b = a.set_field(Coordinate(1,0), FieldType::O)?;
    let final_board = b.set_field(Coordinate(2,0), FieldType::X)?;
    let board_str = final_board.format_board();

    println!("{}", board_str);
    Ok(())
}
