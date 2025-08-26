pub struct BitBoard(pub u64);
use super::square::Square;

impl BitBoard {
    pub fn empty() -> Self {
        BitBoard(0)
    }
    pub fn from_square(sq: Square) -> Self {
        BitBoard(1u64 << sq)
    }
    pub fn set(&mut self, sq: Square) {
        self.0 |= 1u64 << sq
    }
    pub fn popcount(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn lsb(&self) -> Option<Square> {
        if self.0 == 0 {
            None
        } else {
            Some(self.0.trailing_zeros() as Square)
        }
    }
    pub fn pretty(&self) -> String {
        let mut s = String::new();
        for rank in (0..8).rev() {
            let curr = &rank + 1;
            s.push_str(&curr.to_string());
            for file in 0..8 {
                let sq = rank * 8 + file;
                if (self.0 >> sq) & 1 == 1 {
                    s.push_str(" X ");
                } else {
                    s.push_str(" . ");
                }
            }
            s.push('\n');
        }
        s.push_str("  a  b  c  d  e  f  g  h\n");
        s
    }
}
