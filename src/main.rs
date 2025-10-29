use chess_engine::board::Bitboard;

fn main() {
    let bitboard = Bitboard::new(268435456);
    println!("{}", bitboard);
}
