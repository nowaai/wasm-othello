extern crate js_sys;
extern crate wasm_bindgen;

use crate::bits::*;
use crate::board::*;
use crate::player::*;

use wasm_bindgen::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Turn {
    BLACK,
    WHITE,
    GAMEOVER,
}

#[wasm_bindgen]
pub struct Othello {
    board: Board,
    players: Players,
    turn: Turn,
}

#[wasm_bindgen]
impl Othello {
    pub fn make() -> Othello {
        let user = User::new();
        let cpu = AI::new(select_randomly);
        let players = Players::new(user, cpu);

        let mut board = Board::new();
        board.ready();

        Othello {
            board,
            players,
            turn: Turn::BLACK,
        }
    }

    pub fn move_by_ai(&mut self) -> Box<[i32]> {
        let index = (self.players.white.select)(self.board);
        let diff_board = self
            .board
            .move_and_reverse(LOWEST_BIT_MASK << index, self.turn)
            .unwrap();
        Board::to_array(diff_board)
    }

    pub fn move_by_user(&mut self, index: i32) -> Option<Box<[i32]>> {
        if self.board.is_movable(index, self.turn) {
            let diff_board = self
                .board
                .move_and_reverse(LOWEST_BIT_MASK << index, self.turn)
                .unwrap();
            Some(Board::to_array(diff_board))
        } else {
            None
        }
    }

    pub fn which_turn(&mut self) -> String {
        self.turn = self.board.take_turn(self.turn);
        match self.turn {
            Turn::BLACK => String::from("black"),
            Turn::WHITE => String::from("white"),
            Turn::GAMEOVER => String::from("gameover"),
        }
    }

    pub fn get_score(&self) -> Box<[i32]> {
        Box::new([
            self.board.count_black() as i32,
            self.board.count_white() as i32,
        ])
    }
}

#[wasm_bindgen]
pub fn make_othello() -> Othello {
    Othello::make()
}
