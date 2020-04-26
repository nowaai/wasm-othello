use crate::bits::*;
use crate::board::*;
use crate::othello::*;
use rand::Rng;

pub struct User;

impl User {
    pub fn new() -> User {
        User {}
    }
}

pub struct AI {
    pub select: fn(Board) -> u64,
}

impl AI {
    pub fn new(select: fn(Board) -> u64) -> AI {
        AI { select: select }
    }
}

pub struct Players {
    pub black: User,
    pub white: AI,
}

impl Players {
    pub fn new(black: User, white: AI) -> Players {
        Players { black, white }
    }
}

pub fn select_randomly(board: Board) -> u64 {
    let moves = board.list_moves(Turn::WHITE);
    let indices = search_onbit_index(moves);
    let mut rng = rand::thread_rng();
    let index: usize = rng.gen_range(0, indices.len() as usize);
    indices[index]
}
