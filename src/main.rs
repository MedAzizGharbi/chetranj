mod board {
    pub mod bitboard;
    pub mod square;
}
use board::square::*;

use board::bitboard::BitBoard;

fn main() {
    let mut bb = BitBoard::empty();
    let e4 = from_rank_file(3, 4);
    bb.set(e4);
    bb.set(from_rank_file(2, 3));
    println!("{}", bb.pretty());
}
