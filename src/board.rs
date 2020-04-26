use crate::bits::*;
use crate::othello::*;

const NO_DISC_BOARD: u64 = 0x0000000000000000u64;
const BLACK_DEFAULT: u64 = 0x0000000810000000u64;
const WHITE_DEFAULT: u64 = 0x0000001008000000u64;

#[derive(Debug, Copy, Clone)]
pub struct Board {
    pub black: u64,
    pub white: u64,
}

impl Board {
    pub fn new() -> Board {
        Board {
            black: NO_DISC_BOARD,
            white: NO_DISC_BOARD,
        }
    }

    pub fn make(black: u64, white: u64) -> Board {
        Board { black, white }
    }

    pub fn ready(&mut self) {
        self.black = BLACK_DEFAULT;
        self.white = WHITE_DEFAULT;
    }

    pub fn to_array(board: Board) -> Box<[i32]> {
        (0..64)
            .rev()
            .map(|x| {
                if (board.black >> x) & LOWEST_BIT_MASK != 0 {
                    1
                } else if (board.white >> x) & LOWEST_BIT_MASK != 0 {
                    -1
                } else {
                    0
                }
            })
            .collect::<Vec<i32>>()
            .into_boxed_slice()
    }

    pub fn take_turn(&mut self, turn: Turn) -> Turn {
        if self.is_full() {
            return Turn::GAMEOVER;
        }
        let next = match turn {
            Turn::BLACK => Turn::WHITE,
            Turn::WHITE => Turn::BLACK,
            Turn::GAMEOVER => Turn::GAMEOVER,
        };
        if self.is_pass(next) {
            if self.is_opp_pass(next) {
                return Turn::GAMEOVER;
            } else {
                return turn;
            }
        } else {
            return next;
        }
    }

    pub fn move_and_reverse(&mut self, _move: u64, turn: Turn) -> Option<Board> {
        let board_before = self.clone();

        if turn == Turn::BLACK {
            let rev = get_reverses(self.black, self.white, _move);
            self.black = self.black | _move | rev;
            self.white ^= rev;
        } else {
            let rev = get_reverses(self.white, self.black, _move);
            self.white = self.white | _move | rev;
            self.black ^= rev;
        }

        let black_diff = self.black ^ board_before.black;
        let white_diff = self.white ^ board_before.white;

        match turn {
            Turn::BLACK => Some(Board::make(black_diff, 0)),
            Turn::WHITE => Some(Board::make(0, white_diff)),
            _ => None,
        }
    }

    pub fn list_moves(&self, turn: Turn) -> u64 {
        if turn == Turn::BLACK {
            get_moves(self.black, self.white)
        } else {
            get_moves(self.white, self.black)
        }
    }

    pub fn is_movable(&self, index: i32, turn: Turn) -> bool {
        let moves = self.list_moves(turn);
        (moves >> index) & LOWEST_BIT_MASK != 0
    }

    pub fn is_pass(&self, turn: Turn) -> bool {
        self.list_moves(turn) == 0
    }

    pub fn is_opp_pass(&self, turn: Turn) -> bool {
        let opponent = match turn {
            Turn::BLACK => Turn::WHITE,
            Turn::WHITE => Turn::BLACK,
            Turn::GAMEOVER => Turn::GAMEOVER,
        };
        self.list_moves(opponent) == 0
    }

    pub fn is_full(&self) -> bool {
        count_bits(self.black) + count_bits(self.white) == 64
    }

    pub fn count_black(&self) -> u64 {
        count_bits(self.black)
    }

    pub fn count_white(&self) -> u64 {
        count_bits(self.white)
    }
}
