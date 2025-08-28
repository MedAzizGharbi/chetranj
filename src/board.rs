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

/// El Board hiya 3ibara aala two sides
/// Kol side hiya Struct feha el pieces lkol  
/// pub struct Side {
/// pub pawns: BitBoard,
/// pub knights: BitBoard,
/// pub bishops: BitBoard,
/// ...
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

    fn get_piece_char(&self, square: usize) -> char {
        let mask = 1 << square;

        if self.white.pawns & mask != 0 {
            'p'
        } else if self.white.knights & mask != 0 {
            'n'
        } else if self.white.bishops & mask != 0 {
            'b'
        } else if self.white.rooks & mask != 0 {
            'r'
        } else if self.white.queens & mask != 0 {
            'q'
        } else if self.white.king & mask != 0 {
            'k'
        } else if self.black.pawns & mask != 0 {
            'P'
        } else if self.black.knights & mask != 0 {
            'N'
        } else if self.black.bishops & mask != 0 {
            'B'
        } else if self.black.rooks & mask != 0 {
            'R'
        } else if self.black.queens & mask != 0 {
            'Q'
        } else if self.black.king & mask != 0 {
            'K'
        } else {
            '.'
        }
    }

    pub fn print(&self) {
        println!("\n  +------------------------+");
        for rank in (0..8).rev() {
            print!("{} |", rank + 1);
            for file in 0..8 {
                let square = rank * 8 + file;
                let piece_char = self.get_piece_char(square);
                print!(" {} ", piece_char);
            }
            println!("|");
        }
        println!("  +------------------------+");
        println!("    a  b  c  d  e  f  g  h");
    }
}

// Houni tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starting_position_squares() {
        let board = Board::new();
        assert_eq!(board.get_piece_char(0), 'r', "a1 should be black rook");
        assert_eq!(board.get_piece_char(1), 'n', "b1 should be black knight");
        assert_eq!(board.get_piece_char(2), 'b', "c1 should be black bishop");
        assert_eq!(board.get_piece_char(3), 'q', "d1 should be black queen");
        assert_eq!(board.get_piece_char(4), 'k', "e1 should be black king");
        assert_eq!(board.get_piece_char(5), 'b', "f1 should be black bishop");
        assert_eq!(board.get_piece_char(6), 'n', "g1 should be black knight");
        assert_eq!(board.get_piece_char(7), 'r', "h1 should be black rook");

        assert_eq!(board.get_piece_char(56), 'R', "a8 should be white rook");
        assert_eq!(board.get_piece_char(57), 'N', "b8 should be white knight");
        assert_eq!(board.get_piece_char(58), 'B', "c8 should be white bishop");
        assert_eq!(board.get_piece_char(59), 'Q', "d8 should be white queen");
        assert_eq!(board.get_piece_char(60), 'K', "e8 should be white king");
        assert_eq!(board.get_piece_char(61), 'B', "f8 should be white bishop");
        assert_eq!(board.get_piece_char(62), 'N', "g8 should be white knight");
        assert_eq!(board.get_piece_char(63), 'R', "h8 should be white rook");
    }

    #[test]
    fn test_count_pieces() {
        let board = Board::new();
        assert_eq!(
            board.white.pawns.count_ones(),
            8,
            "White should have 8 pawns"
        );
        assert_eq!(board.white.king.count_ones(), 1, "White should have 1 king");
    }

    #[test]
    fn test_basic_squares() {
        let board = Board::new();
        assert_eq!(board.get_piece_char(0), 'r');
    }
    #[test]
    fn test_get_piece_bitboard() {
        let board = Board::new();
        let pawns = board.get_piece_bitboard(Color::White, Piece::Pawn);
        let knights = board.get_piece_bitboard(Color::White, Piece::Knight);
        let bishops = board.get_piece_bitboard(Color::White, Piece::Bishop);
        let rooks = board.get_piece_bitboard(Color::White, Piece::Rook);
        let queens = board.get_piece_bitboard(Color::White, Piece::Queen);
        let king = board.get_piece_bitboard(Color::White, Piece::King);
        let bking = board.get_piece_bitboard(Color::Black, Piece::King);
        assert_eq!(pawns.count_ones(), 8);
        assert_eq!(knights.count_ones(), 2);
        assert_eq!(bishops.count_ones(), 2);
        assert_eq!(rooks.count_ones(), 2);
        assert_eq!(queens.count_ones(), 1);
        assert_eq!(king.count_ones(), 1);
        assert_eq!(king.count_ones(), 1);
        assert_eq!(bking.count_ones(), 1);
    }
    #[test]
    fn test_combined_occupancy() {
        let board: Board = Board::new();
        assert_eq!(
            board.combined_occupancy().count_ones(),
            32,
            "Total board pieces count should be 32"
        );
        assert_eq!(
            board.white.combined().count_ones(),
            16,
            "Total white pieces count should be 16"
        );
        assert_eq!(
            board.black.combined().count_ones(),
            16,
            "Total black pieces count should be 16"
        );
    }
}
