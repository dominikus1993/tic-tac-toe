use rand::{Rng, prelude::IteratorRandom};
use super::board::{Board, Coordinate, FieldType};

pub enum Player{
    Human(FieldType),
    Computer(FieldType),
} 

fn read_stdin(board : Board, f: &FieldType) -> Option<Coordinate> {
    println!("{} Wybierz koordynaty: x,y ", f.format());
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let res = input.parse::<Coordinate>();
        match res {
            Ok(c) => {
                if board.is_move_valid(c) {
                    return Some(c);
                }
                println!("Nieprawidlowe koordynaty w");
            },
            Err(e) => {
                println!("Nieprawidlowe koordynaty {:?}", e);
            }
        }
    }

}

fn run_advanced_ai_move(board: Board) -> Option<Coordinate> {
    let mut rng = rand::thread_rng();
    let free_cooords = board.get_free_coordinates();
    if free_cooords.is_empty() {
        return None;
    }
    let random_index = rng.gen_range(0..free_cooords.len());
    Some(free_cooords[random_index])
}

impl Player {
    pub fn read_move(&self, board : Board) -> Option<Coordinate> {
       match self {
              Player::Human(f) => read_stdin(board, f),
              Player::Computer(_) => run_advanced_ai_move(board),
       }
    }

    pub fn get_field_type(&self) -> &FieldType {
        match self {
            Player::Human(f) => f,
            Player::Computer(f) => f,
        }
    }
}

