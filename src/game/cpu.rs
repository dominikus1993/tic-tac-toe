use rand::{Rng, prelude::IteratorRandom};
use super::board::{Board, Coordinate};

pub fn run_advanced_ai_move(board: Board) -> Option<Coordinate> {
    let mut rng = rand::thread_rng();
    let free_cooords = board.get_free_coordinates();
    if free_cooords.is_empty() {
        return None;
    }
    let random_index = rng.gen_range(0..free_cooords.len());
    Some(free_cooords[random_index])
}