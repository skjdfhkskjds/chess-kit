use chess_kit_position::PositionView;
use chess_kit_primitives::{Move, MoveList, MoveType, Pieces};

const HASH_MOVE_SCORE: i32 = 1_000_000;
const TACTICAL_MOVE_SCORE: i32 = 100_000;
const VICTIM_MULTIPLIER: i32 = 16;

/// order_moves orders moves by their expected alpha-beta cutoff value
///
/// The transposition-table move is always searched first. Captures use
/// most-valuable-victim/least-valuable-attacker ordering, while promotions
/// receive an additional bonus for the promoted material. Remaining moves keep
/// their generated order.
///
/// @param: position - immutable view of the position before any move is played
/// @param: moves - legal or pseudo-legal moves to order
/// @param: hash_move - best move from the transposition table, if available
/// @return: void
/// @side-effects: reorders the move list in place
pub(crate) fn order_moves<PositionT>(
    position: &PositionT,
    moves: &mut MoveList,
    hash_move: Option<Move>,
) where
    PositionT: PositionView,
{
    // Insertion sort is allocation-free and stable. Most generated moves have
    // the same quiet score, making the common case close to linear while
    // preserving their generator order.
    for index in 1..moves.len() {
        let mv = moves.as_slice()[index];
        let score = move_score(position, mv, hash_move);
        let mut insertion_index = index;

        while insertion_index > 0
            && score > move_score(position, moves.as_slice()[insertion_index - 1], hash_move)
        {
            let previous = moves.as_slice()[insertion_index - 1];
            moves.as_mut_slice()[insertion_index] = previous;
            insertion_index -= 1;
        }

        moves.as_mut_slice()[insertion_index] = mv;
    }
}

/// move_score assigns an ordering score to a move
///
/// @param: position - position before the move is played
/// @param: mv - move to score
/// @param: hash_move - best move from the transposition table, if available
/// @return: relative move-ordering score
#[inline]
fn move_score<PositionT>(position: &PositionT, mv: Move, hash_move: Option<Move>) -> i32
where
    PositionT: PositionView,
{
    if hash_move == Some(mv) {
        return HASH_MOVE_SCORE;
    }

    let attacker = position.piece_at(mv.from());
    let victim = if mv.type_of() == MoveType::EnPassant {
        Pieces::Pawn
    } else {
        position.piece_at(mv.to())
    };
    let promotion_gain = if mv.type_of() == MoveType::Promotion {
        piece_value(mv.promoted_to()) - piece_value(Pieces::Pawn)
    } else {
        0
    };

    if victim == Pieces::None && promotion_gain == 0 {
        return 0;
    }

    TACTICAL_MOVE_SCORE + piece_value(victim) * VICTIM_MULTIPLIER - piece_value(attacker)
        + promotion_gain
}

/// piece_value returns material values used only for move ordering
///
/// These values intentionally do not depend on the selected evaluation
/// implementation. Only their relative ordering matters to MVV-LVA.
#[inline]
const fn piece_value(piece: Pieces) -> i32 {
    match piece {
        Pieces::None => 0,
        Pieces::Pawn => 100,
        Pieces::Knight | Pieces::Bishop => 300,
        Pieces::Rook => 500,
        Pieces::Queen => 900,
        // The king needs a finite ordering value when it is the attacker. It
        // can never be a legal capture victim.
        Pieces::King => 1_000,
    }
}

#[cfg(test)]
mod tests {
    use chess_kit_attack_table::DefaultAttackTable;
    use chess_kit_position::{DefaultPosition, Fen, Setup};
    use chess_kit_primitives::{Move, MoveList, Pieces, Square};

    use super::order_moves;

    type TestPosition = DefaultPosition<DefaultAttackTable>;

    fn load(fen: &str) -> TestPosition {
        TestPosition::from(Setup::from(Fen::try_from(fen).unwrap()))
    }

    #[test]
    fn orders_captures_by_victim_then_attacker_value() {
        let position = load("4k3/8/8/2p1q3/3P4/8/2Q5/4K3 w - - 0 1");
        let queen_takes_pawn = Move::new(Square::C2, Square::C5);
        let pawn_takes_queen = Move::new(Square::D4, Square::E5);
        let quiet = Move::new(Square::E1, Square::E2);
        let mut moves = MoveList::new();
        moves.push(queen_takes_pawn);
        moves.push(quiet);
        moves.push(pawn_takes_queen);

        order_moves(&position, &mut moves, None);

        assert_eq!(
            moves.as_slice(),
            &[pawn_takes_queen, queen_takes_pawn, quiet]
        );
    }

    #[test]
    fn prefers_the_least_valuable_attacker_for_the_same_victim() {
        let position = load("7k/8/8/4q3/3P4/8/8/K3R3 w - - 0 1");
        let rook_takes_queen = Move::new(Square::E1, Square::E5);
        let pawn_takes_queen = Move::new(Square::D4, Square::E5);
        let mut moves = MoveList::new();
        moves.push(rook_takes_queen);
        moves.push(pawn_takes_queen);

        order_moves(&position, &mut moves, None);

        assert_eq!(moves.as_slice(), &[pawn_takes_queen, rook_takes_queen]);
    }

    #[test]
    fn hash_move_is_ordered_before_tactical_moves() {
        let position = load("4k3/8/8/2p1q3/3P4/8/2Q5/4K3 w - - 0 1");
        let capture = Move::new(Square::D4, Square::E5);
        let hash_move = Move::new(Square::E1, Square::E2);
        let mut moves = MoveList::new();
        moves.push(capture);
        moves.push(hash_move);

        order_moves(&position, &mut moves, Some(hash_move));

        assert_eq!(moves.as_slice(), &[hash_move, capture]);
    }

    #[test]
    fn recognizes_en_passant_as_a_capture() {
        let position = load("4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 1");
        let quiet = Move::new(Square::E1, Square::E2);
        let en_passant = Move::new(Square::E5, Square::D6).with_en_passant();
        let mut moves = MoveList::new();
        moves.push(quiet);
        moves.push(en_passant);

        order_moves(&position, &mut moves, None);

        assert_eq!(moves.as_slice(), &[en_passant, quiet]);
    }

    #[test]
    fn orders_promotions_by_promoted_material() {
        let position = load("4k3/P7/8/8/8/8/8/4K3 w - - 0 1");
        let quiet = Move::new(Square::E1, Square::E2);
        let knight_promotion = Move::new(Square::A7, Square::A8).with_promotion(Pieces::Knight);
        let queen_promotion = Move::new(Square::A7, Square::A8).with_promotion(Pieces::Queen);
        let mut moves = MoveList::new();
        moves.push(quiet);
        moves.push(knight_promotion);
        moves.push(queen_promotion);

        order_moves(&position, &mut moves, None);

        assert_eq!(
            moves.as_slice(),
            &[queen_promotion, knight_promotion, quiet]
        );
    }
}
