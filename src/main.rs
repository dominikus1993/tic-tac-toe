mod game;
use game::*;

fn main() -> GameResult<()>{
    let game = Game::start();
    game.move_next()?;
    Ok(())
}
