use chetranj::board::Board;
fn main() {
    println!("amjen chess engine");
    let board = Board::new();
    println!("Board visualization: ");
    board.print();
    // Hedhom lkol bch tefhem ya fazeni
    // println!("Board occupancy: {:064b}", board.combined_occupancy());
    // println!("total pieces {}", board.combined_occupancy().count_ones());
    // println!("white pieces {}", board.white.combined().count_ones());
    // println!("black pieces {}", board.black.combined().count_ones());
}
