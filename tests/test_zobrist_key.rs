use chess_kit::attack_table::DefaultAttackTable;
use chess_kit::position::{DefaultPosition, Position, PositionFromFEN};
use chess_kit::primitives::ZobristKey;
use chess_kit::state::{DefaultState, ReadOnlyState};

#[test]
fn test_zobrist_key() {
    for (i, test) in TEST_CASES.iter().enumerate() {
        let parts = test.split('|').collect::<Vec<&str>>();
        let fen = parts[0];
        let key = ZobristKey::try_from(parts[1])
            .unwrap_or_else(|_| panic!("Invalid Zobrist key {}", parts[1]));

        let mut position = DefaultPosition::<DefaultAttackTable, DefaultState>::new();
        if let Err(e) = position.load_fen(fen) {
            panic!("Error loading FEN: {}", e);
        }

        let zobrist_key = position.state().key();
        assert_eq!(
            zobrist_key,
            key,
            "Test {}: FEN: {}, Expected: {}, Actual: {}, Position: {}",
            i + 1,
            fen,
            key,
            zobrist_key,
            position
        );
    }
}

const TEST_CASES: [&str; 9] = [
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1|463b96181691fc9c",
    "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1|823c9b50fd114196",
    "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2|0756b94461c50fb0",
    "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 2|662fafb965db29d4",
    "rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3|22a48b5a8e47ff78",
    "rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPPKPPP/RNBQ1BNR b kq - 0 3|652a607ca3f242c1",
    "rnbq1bnr/ppp1pkpp/8/3pPp2/8/8/PPPPKPPP/RNBQ1BNR w - - 0 4|00fdd303c946bdd9",
    "rnbqkbnr/p1pppppp/8/8/PpP4P/8/1P1PPPP1/RNBQKBNR b KQkq c3 0 3|3c8123ea7b067637",
    "rnbqkbnr/p1pppppp/8/8/P6P/R1p5/1P1PPPP1/1NBQKBNR b Kkq - 0 4|5c3f9b829b279560",
];
