pub mod board;
use board::*;

pub enum Player{
    Human,
    Computer,
}

pub struct Game {
    pub player1 : Player,
    pub player2 : Player,
    pub board : Board,
}

impl Game {
    pub fn start() -> Game {
        let board = Board::empty();
        Game {
            player1 : Player::Human,
            player2 : Player::Computer,
            board,
        }
    }
}