pub type BitBoard = u64;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

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

pub struct Board {
    pub white: Side,
    pub black: Side,
}

impl Board {
    pub fn new() -> Self {
        Board {
            white: Side {
                pawns: 0x0000_0000_0000_FF00,
                knights: 0x0000_0000_0000_0042,
                bishops: 0x0000_0000_0000_0024,
                rooks: 0x0000_0000_0000_0081,
                queens: 0x0000_0000_0000_0008,
                king: 0x0000_0000_0000_0010,
            },
            black: Side {
                pawns: 0x00FF_0000_0000_0000,
                knights: 0x4200_0000_0000_0000,
                bishops: 0x2400_0000_0000_0000,
                rooks: 0x8100_0000_0000_0000,
                queens: 0x0800_0000_0000_0000,
                king: 0x1000_0000_0000_0000,
            },
        }
    }
    pub fn combined_occupancy(&self) -> BitBoard {
        self.white.combined() | self.black.combined()
    }

    // Get a specific piece's bitboard by type and color
    pub fn get_piece_bitboard(&self, color: Color, piece: Piece) -> BitBoard {
        let side = match color {
            Color::White => &self.white,
            Color::Black => &self.black,
        };
        match piece {
            Piece::Pawn => side.pawns,
            Piece::Knight => side.knights,
            Piece::Bishop => side.bishops,
            Piece::Rook => side.rooks,
            Piece::Queen => side.queens,
            Piece::King => side.king,
        }
    }
}
