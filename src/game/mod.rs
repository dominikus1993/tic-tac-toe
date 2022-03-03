mod board;
mod cpu;
use std::cell::Cell;

use board::*;

pub type GameResult<T> = std::result::Result<T, Errors>;

pub enum Player{
    Human,
    Computer,
}

pub struct Game {
    player1 : Player,
    player2 : Player,
    board : Cell<Board>,
}

impl Game {
    pub fn start() -> Game {
        let board = Board::empty();
        Game {
            player1 : Player::Human,
            player2 : Player::Computer,
            board : Cell::new(board),
        }
    }

    fn read_stdin() -> GameResult<Coordinate> {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        input.parse::<Coordinate>()?
    }

    pub fn move_next(&self) -> GameResult<()> {
        loop {
            let p1 = cpu::run_advanced_ai_move(self.board.get());
            match p1 {
                Some(c) => {
                    let result = self.board.get().move_next(c, FieldType::O)?;
                    self.board.set(result);
                    let p2 = cpu::run_advanced_ai_move(self.board.get());
                    match p2 {
                        Some(c) => {
                            let result = self.board.get().move_next(c, FieldType::X)?;
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