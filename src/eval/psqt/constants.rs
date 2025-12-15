use super::{GamePhase, PSQTable, PieceValue as PV};
use crate::primitives::{Pieces, Sides, Square};

// PHASE_VALUES is a constant array of values representing the impact that a
// piece has on the game phase heuristic
//
// TODO: we have a leading 0 for Pieces::NONE, remove when possible
pub(super) const PHASE_VALUES: [GamePhase; Pieces::TOTAL] = [0, 0, 1050, 405, 305, 155, 0];

// MIDDLEGAME_PHASE_MIN and MIDDLEGAME_PHASE_MAX are the minimum and maximum
// values for the middlegame phase
pub(super) const MIDDLEGAME_PHASE_MIN: GamePhase = 435;
pub(super) const MIDDLEGAME_PHASE_MAX: GamePhase = 5255;

// PIECE_TABLES is a constant array of piece tables for each side and piece
//
// note: this table is useful to autovectorize access to the PSQT collection of
//       constants AND avoids the need to conditionally invert the square lookup
//       for the white perspective at runtime
pub(super) const PIECE_TABLES: [[PSQTable; Pieces::TOTAL]; Sides::TOTAL] = {
    let mut tables = [[[PV::new(0, 0); Square::TOTAL]; Pieces::TOTAL]; Sides::TOTAL];

    let mut piece_idx = Pieces::Pawn.idx();
    while piece_idx < Pieces::TOTAL {
        let psqt = match Pieces::from_idx(piece_idx) {
            Pieces::King => KING_PSQT,
            Pieces::Queen => QUEEN_PSQT,
            Pieces::Rook => ROOK_PSQT,
            Pieces::Bishop => BISHOP_PSQT,
            Pieces::Knight => KNIGHT_PSQT,
            Pieces::Pawn => PAWN_PSQT,
            _ => unreachable!(),
        };

        let mut square_idx = 0;
        while square_idx < Square::TOTAL {
            // invert the square index for the white perspective since white
            // believes that A1 is on the bottom-left corner of the board
            //
            // TODO: abstract this into a Square::relative<SideT> or something
            tables[Sides::White.idx()][piece_idx][square_idx] =
                psqt[Square::INVERTED[square_idx].idx()];

            // black's perspective is normal, so we don't need to invert
            tables[Sides::Black.idx()][piece_idx][square_idx] = psqt[square_idx];

            square_idx += 1;
        }

        piece_idx += 1;
    }

    tables
};

#[rustfmt::skip]
const KING_PSQT: PSQTable = 
[
    PV::new(0,-95), PV::new(0,-95), PV::new( 0,-90), PV::new(  0,-90), PV::new(  0,-90), PV::new(0,-90), PV::new( 0,-95), PV::new(0,-95),
    PV::new(0,-95), PV::new(0,-50), PV::new( 0,-50), PV::new(  0,-50), PV::new(  0,-50), PV::new(0,-50), PV::new( 0,-50), PV::new(0,-95),
    PV::new(0,-90), PV::new(0,-50), PV::new( 0,-20), PV::new(  0,-20), PV::new(  0,-20), PV::new(0,-20), PV::new( 0,-50), PV::new(0,-90),
    PV::new(0,-90), PV::new(0,-50), PV::new( 0,-20), PV::new(  0,  0), PV::new(  0,  0), PV::new(0,-20), PV::new( 0,-50), PV::new(0,-90),
    PV::new(0,-90), PV::new(0,-50), PV::new( 0,-20), PV::new(  0,  0), PV::new(  0,  0), PV::new(0,-20), PV::new( 0,-50), PV::new(0,-90),
    PV::new(0,-90), PV::new(0,-50), PV::new( 0,-20), PV::new(  0,-20), PV::new(  0,-20), PV::new(0,-20), PV::new( 0,-50), PV::new(0,-90),
    PV::new(0,-95), PV::new(0,-50), PV::new( 0,-50), PV::new(-10,-50), PV::new(-10,-50), PV::new(0,-50), PV::new( 0,-50), PV::new(0,-95),
    PV::new(0,-95), PV::new(0,-95), PV::new(20,-90), PV::new(-10,-90), PV::new(-10,-90), PV::new(0,-90), PV::new(20,-95), PV::new(0,-95),
];

#[rustfmt::skip]
const QUEEN_PSQT: PSQTable = 
[
    PV::new(870,870), PV::new(880,880), PV::new(890,890), PV::new(890,890), PV::new(890,890), PV::new(890,890), PV::new(880,880), PV::new(870,870),
    PV::new(880,880), PV::new(890,890), PV::new(895,895), PV::new(895,895), PV::new(895,895), PV::new(895,895), PV::new(890,890), PV::new(880,880),
    PV::new(890,890), PV::new(895,895), PV::new(910,910), PV::new(910,910), PV::new(910,910), PV::new(910,910), PV::new(895,895), PV::new(890,890),
    PV::new(890,890), PV::new(895,895), PV::new(910,910), PV::new(920,920), PV::new(920,920), PV::new(910,910), PV::new(895,895), PV::new(890,890),
    PV::new(890,890), PV::new(895,895), PV::new(910,910), PV::new(920,920), PV::new(920,920), PV::new(910,910), PV::new(895,895), PV::new(890,890),
    PV::new(890,890), PV::new(895,895), PV::new(895,895), PV::new(895,895), PV::new(895,895), PV::new(895,895), PV::new(895,895), PV::new(890,890),
    PV::new(880,880), PV::new(890,890), PV::new(895,895), PV::new(895,895), PV::new(895,895), PV::new(895,895), PV::new(890,890), PV::new(880,880),
    PV::new(870,870), PV::new(880,880), PV::new(890,890), PV::new(890,890), PV::new(890,890), PV::new(890,890), PV::new(880,880), PV::new(870,870),
];

#[rustfmt::skip]
const ROOK_PSQT: PSQTable = 
[
    PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500),
    PV::new(515,515), PV::new(515,515), PV::new(515,515), PV::new(520,520), PV::new(520,520), PV::new(515,515), PV::new(515,515), PV::new(515,515),
    PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500),
    PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500),
    PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500),
    PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500),
    PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(500,500),
    PV::new(500,500), PV::new(500,500), PV::new(500,500), PV::new(510,510), PV::new(510,510), PV::new(510,510), PV::new(500,500), PV::new(500,500),
];

#[rustfmt::skip]
const BISHOP_PSQT: PSQTable = 
[
    PV::new(300,300), PV::new(320,320), PV::new(320,320), PV::new(320,320), PV::new(320,320), PV::new(320,320), PV::new(320,320), PV::new(300,300),
    PV::new(305,305), PV::new(320,320), PV::new(320,320), PV::new(320,320), PV::new(320,320), PV::new(320,320), PV::new(320,320), PV::new(305,305),
    PV::new(310,310), PV::new(320,320), PV::new(320,320), PV::new(325,325), PV::new(325,325), PV::new(320,320), PV::new(320,320), PV::new(310,310),
    PV::new(310,310), PV::new(330,330), PV::new(330,330), PV::new(350,350), PV::new(350,350), PV::new(330,330), PV::new(330,330), PV::new(310,310),
    PV::new(325,325), PV::new(325,325), PV::new(330,330), PV::new(345,345), PV::new(345,345), PV::new(330,330), PV::new(325,325), PV::new(325,325),
    PV::new(325,325), PV::new(325,325), PV::new(325,325), PV::new(330,330), PV::new(330,330), PV::new(325,325), PV::new(325,325), PV::new(325,325),
    PV::new(310,310), PV::new(325,325), PV::new(325,325), PV::new(330,330), PV::new(330,330), PV::new(325,325), PV::new(325,325), PV::new(310,310),
    PV::new(300,300), PV::new(310,310), PV::new(310,310), PV::new(310,310), PV::new(310,310), PV::new(310,310), PV::new(310,310), PV::new(300,300),
];

#[rustfmt::skip]
const KNIGHT_PSQT: PSQTable =     
[
    PV::new(290,290), PV::new(300,300), PV::new(300,300), PV::new(300,300), PV::new(300,300), PV::new(300,300), PV::new(300,300), PV::new(290,290),
    PV::new(300,300), PV::new(305,305), PV::new(305,305), PV::new(305,305), PV::new(305,305), PV::new(305,305), PV::new(305,305), PV::new(300,300),
    PV::new(300,300), PV::new(305,305), PV::new(325,325), PV::new(325,325), PV::new(325,325), PV::new(325,325), PV::new(305,305), PV::new(300,300),
    PV::new(300,300), PV::new(305,305), PV::new(325,325), PV::new(325,325), PV::new(325,325), PV::new(325,325), PV::new(305,305), PV::new(300,300),
    PV::new(300,300), PV::new(305,305), PV::new(325,325), PV::new(325,325), PV::new(325,325), PV::new(325,325), PV::new(305,305), PV::new(300,300),
    PV::new(300,300), PV::new(305,305), PV::new(320,320), PV::new(325,325), PV::new(325,325), PV::new(325,325), PV::new(305,305), PV::new(300,300),
    PV::new(300,300), PV::new(305,305), PV::new(305,305), PV::new(305,305), PV::new(305,305), PV::new(305,305), PV::new(305,305), PV::new(300,300),
    PV::new(290,290), PV::new(310,310), PV::new(300,300), PV::new(300,300), PV::new(300,300), PV::new(300,300), PV::new(310,310), PV::new(290,290),
];

#[rustfmt::skip]
const PAWN_PSQT: PSQTable = 
[
    PV::new(100,100), PV::new(100,100), PV::new(100,100), PV::new(100,100), PV::new(100,100), PV::new(100,100), PV::new(100,100), PV::new(100,100),
    PV::new(160,160), PV::new(160,160), PV::new(160,160), PV::new(160,160), PV::new(170,170), PV::new(160,160), PV::new(160,160), PV::new(160,160),
    PV::new(140,140), PV::new(140,140), PV::new(140,140), PV::new(150,150), PV::new(160,160), PV::new(140,140), PV::new(140,140), PV::new(140,140),
    PV::new(120,120), PV::new(120,120), PV::new(120,120), PV::new(140,140), PV::new(150,150), PV::new(120,120), PV::new(120,120), PV::new(120,120),
    PV::new(105,105), PV::new(105,105), PV::new(115,115), PV::new(130,130), PV::new(140,140), PV::new(110,110), PV::new(105,105), PV::new(105,105),
    PV::new(105,105), PV::new(105,105), PV::new(110,110), PV::new(120,120), PV::new(130,130), PV::new(105,105), PV::new(105,105), PV::new(105,105),
    PV::new(105,105), PV::new(105,105), PV::new(105,105), PV::new( 70, 70), PV::new( 70, 70), PV::new(105,105), PV::new(105,105), PV::new(105,105),
    PV::new(100,100), PV::new(100,100), PV::new(100,100), PV::new(100,100), PV::new(100,100), PV::new(100,100), PV::new(100,100), PV::new(100,100),
];
