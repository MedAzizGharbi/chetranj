#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Square(pub u8);

impl Square {
    pub const A1: Square = Square(0);
    pub const H8: Square = Square(63);

    pub fn new(index: u8) -> Option<Self> {
        if index < 64 {
            Some(Square(index))
        } else {
            None
        }
    }

    pub fn from_coords(file: u8, rank: u8) -> Option<Self> {
        if file < 8 && rank < 8 {
            Some(Square(rank * 8 + file))
        } else {
            None
        }
    }

    pub fn index(&self) -> u8 {
        self.0
    }

    pub fn file(&self) -> u8 {
        self.0 % 8
    }

    pub fn rank(&self) -> u8 {
        self.0 / 8
    }

    pub fn name(&self) -> String {
        let files = "abcdefgh";
        format!(
            "{}{}",
            files.chars().nth(self.file() as usize).unwrap(),
            self.rank() + 1
        )
    }
}
