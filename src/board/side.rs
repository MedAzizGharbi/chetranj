use crate::BitBoard;
// Struct l kol side
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Side {
    pub pawns: BitBoard,
    pub knights: BitBoard,
    pub bishops: BitBoard,
    pub rooks: BitBoard,
    pub queens: BitBoard,
    pub king: BitBoard,
}

impl Side {
    pub fn combined(&self) -> BitBoard {
        self.pawns | self.knights | self.bishops | self.rooks | self.queens | self.king
    }
}
