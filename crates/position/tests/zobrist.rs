use chess_kit_attack_table::DefaultAttackTable;
use chess_kit_position::{DefaultPosition, Fen, PositionView, Setup};
use chess_kit_primitives::ZobristKey;

#[test]
fn zobrist_keys_match_known_positions() {
    for (index, line) in include_str!("fixtures/zobrist.epd").lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts = line.split('|').collect::<Vec<_>>();
        assert_eq!(
            parts.len(),
            2,
            "invalid Zobrist fixture line {}: expected <FEN>|<key>",
            index + 1
        );

        let fen_text = parts[0];
        let key = ZobristKey::try_from(parts[1])
            .unwrap_or_else(|_| panic!("invalid Zobrist key {}", parts[1]));

        let fen = Fen::try_from(fen_text)
            .unwrap_or_else(|err| panic!("error loading FEN '{fen_text}': {err}"));
        let position = DefaultPosition::<DefaultAttackTable>::from(Setup::from(fen));

        let zobrist_key = position.key();
        assert_eq!(
            zobrist_key,
            key,
            "fixture line {}: FEN: {}, Expected: {}, Actual: {}, Position: {}",
            index + 1,
            fen_text,
            key,
            zobrist_key,
            position
        );
    }
}
