mod board;
mod player;
use std::{cell::Cell, io::Read};

use board::*;
use self::player::Player;

pub type GameResult<T> = std::result::Result<T, Errors>;
pub struct Game {
    player1 : Player,
    player2 : Player,
    board : Cell<Board>,
}

impl Game {
    pub fn start() -> Game {
        let board = Board::empty();
        Game {
            player1 : Player::Human(FieldType::X),
            player2 : Player::Computer(FieldType::O),
            board : Cell::new(board),
        }
    }

    pub fn move_next(&self) -> GameResult<()> {
        loop {
            let p1 = self.player1.read_move(self.board.get());
            match p1 {
                Some(c) => {
                    let result = self.board.get().move_next(c, self.player1.get_field_type())?;
                    self.board.set(result);
                    let p2 = self.player2.read_move(self.board.get());
                    match p2 {
                        Some(c) => {
                            let result = self.board.get().move_next(c, self.player2.get_field_type())?;
                            self.board.set(result);
                        },
                        None => {
                            println!("No moves left");
                        }
                    }
                },
                None => {
                    println!("No moves left");
                }
            }

            let bstr = self.board.get().format_board();
            println!("{}", bstr);
            let game_result = self.board.get().check_game_result();
            if game_result != GameResults::InProgress {
                println!("Winner is {:?}", game_result);
                break;
            }
        }
        Ok(())
    }
}