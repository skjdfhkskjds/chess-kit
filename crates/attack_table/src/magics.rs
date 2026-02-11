use crate::sliding_pieces::{bishop_attack_board, bishop_mask, rook_attack_board, rook_mask};
use crate::table::MagicTable;
use chess_kit_primitives::{Bitboard, Square};

const ROOK_TABLE_SIZE: usize = 102_400; // total permutations of all rook boards
const BISHOP_TABLE_SIZE: usize = 5_248; // total permutations of all bishop boards

#[rustfmt::skip]
#[allow(clippy::unreadable_literal)]
pub const ROOK_MAGIC_NUMS: [u64; Square::TOTAL] = [
    324259448050975248u64, 162139001189302336u64, 4647750006529359880u64, 144121785691422736u64,
    16176938657641660544u64, 9367489423970945072u64, 36051338366288384u64, 36029147746665088u64,
    3518447965192208u64, 4614078830617822340u64, 9241949523864129664u64, 11540615780106252u64,
    730287067600519297u64, 144819425575437312u64, 1225261127674627584u64, 40814017656160512u64,
    594475700577118276u64, 283675082228259u64, 148058037853261952u64, 14411662294658320384u64,
    2394186703782912u64, 1157847866488718336u64, 2306407062973841412u64, 4576167411597460u64,
    2323857959626489888u64, 18860477004136448u64, 621497027752297522u64, 3027553647748714496u64,
    9241953785514295424u64, 1970363492082688u64, 1729664285938024960u64, 4836870457972064321u64,
    141012374650913u64, 4652253601601699840u64, 58687601506263040u64, 281543780081672u64,
    1157433900411130112u64, 81628378934806544u64, 2310366730829959192u64, 2900476768907429780u64,
    36558770110480u64, 9042384969023488u64, 180425597514743824u64, 5487636764434923528u64,
    5766860422494879764u64, 9224498487624761348u64, 41702298761822218u64, 45599234000551940u64,
    70370891935872u64, 19210671497487104u64, 387030266675328u64, 289215847808893056u64,
    576469550545240192u64, 1153216449143113729u64, 9350715278336u64, 288521763922764288u64,
    282782794268833u64, 595672521157161122u64, 436884352794689609u64, 9241667927690743809u64,
    5188428314494240769u64, 1157988067282792450u64, 1152939243166828548u64, 4611967569673330817u64,
];

/** Bishop magic numbers. Don't touch them. Changing these numbers breaks the program. */
#[rustfmt::skip]
#[allow(clippy::unreadable_literal)]
pub const BISHOP_MAGIC_NUMS: [u64; Square::TOTAL] = [
    2310454429704290569u64, 37163502750244928u64, 145330200115150856u64, 573953659699200u64,
    9845999220824211456u64, 574016004032512u64, 10093699283674480640u64, 2306407060834902016u64,
    2883575003184432136u64, 1747410678824308864u64, 9259405249167245312u64, 936784527773139074u64,
    4629702641998381057u64, 201028145628315697u64, 4899992295377881088u64, 4630405483133404688u64,
    153474299838154784u64, 2286992943744036u64, 434597432802681416u64, 865817269052115456u64,
    9156750026475656u64, 599823317909770240u64, 4578375142474880u64, 2308525819264500224u64,
    18596057879421451u64, 18331093560345096u64, 2305880392877736000u64, 56602859688444160u64,
    5382084129205534724u64, 5767422822691897608u64, 283691220206592u64, 144398865845093376u64,
    1163523824685120u64, 20267333288223264u64, 325489801822240u64, 4755836425302245636u64,
    594475563668865152u64, 1162496335329427604u64, 9244765235704371236u64, 576667461564269056u64,
    146371454722771202u64, 426679365288452u64, 13724105480340736u64, 1152922330050364928u64,
    4620737202526097424u64, 1316476062695166464u64, 13981996823661781640u64, 12430506881068303489u64,
    5193780677221351424u64, 426612797737280u64, 37445932288049152u64, 1171147012042137601u64,
    504403227018657856u64, 4629845569785954560u64, 4686013077882208273u64, 1154056209263894528u64,
    613054853085794304u64, 9025075185721408u64, 9571249324951568u64, 10999715432448u64,
    290408795603472u64, 10664524198170591488u64, 5924513492108288u64, 90511840181764112u64,
];

pub(crate) struct BishopMagicsTable {
    pub table: [Bitboard; BISHOP_TABLE_SIZE],
    pub magics: MagicTable,
}

pub(crate) struct RookMagicsTable {
    pub table: [Bitboard; ROOK_TABLE_SIZE],
    pub magics: MagicTable,
}

impl BishopMagicsTable {
    /// new creates and initializes a new bishop magics table
    ///
    /// @return: new bishop magics table
    pub const fn new() -> Self {
        let mut attack_table = Self {
            table: [Bitboard::empty(); BISHOP_TABLE_SIZE],
            magics: [Magic::default(); Square::TOTAL],
        };

        // initialize the bishop magics table
        attack_table.magics = new_bishop_magics(&mut attack_table.table);
        attack_table
    }
}

impl RookMagicsTable {
    /// new creates and initializes a new rook magics table
    ///
    /// @return: new rook magics table
    pub const fn new() -> Self {
        let mut attack_table = Self {
            table: [Bitboard::empty(); ROOK_TABLE_SIZE],
            magics: [Magic::default(); Square::TOTAL],
        };

        // initialize the rook magics table
        attack_table.magics = new_rook_magics(&mut attack_table.table);
        attack_table
    }
}

/// `Magic` is a struct that represents a magic number and its associated mask,
/// shift, offset, and num
///
/// @type
#[derive(Copy, Clone)]
pub struct Magic {
    mask: u64,   // square mask
    shift: u8,   // shift for magic index
    offset: u64, // offset of the attack table for the given square
    num: u64,    // value of the magic number
}

impl Magic {
    /// new creates a new magic with the given mask, shift, offset, and num
    ///
    /// @param: mask - mask for the magic
    /// @param: shift - shift for the magic
    /// @param: offset - offset for the magic
    /// @param: num - num for the magic
    /// @return: new magic
    pub const fn new(mask: u64, shift: u8, offset: u64, num: u64) -> Self {
        Self {
            mask,
            shift,
            offset,
            num,
        }
    }

    /// default creates a new default magic
    ///
    /// @return: new default magic
    pub const fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }

    /// idx gets the magic index for the given occupancy
    ///
    /// @param: occupancy - occupancy to get the magic index for
    /// @return: magic index for the given occupancy
    #[inline(always)]
    pub const fn idx(&self, occupancy: Bitboard) -> usize {
        let blockerboard = occupancy.const_unwrap() & self.mask;
        ((blockerboard.wrapping_mul(self.num) >> self.shift) + self.offset) as usize
    }
}

/// new_rook_square_magics creates a new magic for the given square
///
/// note: this function is copy-pasted to improve CTFE performance by avoiding
///       branching in the hot path
///
/// @param: offset - offset into the existing attack table
/// @param: square - square to create the magic for
/// @param: table - attack table to create the magic for
/// @return: new magic for the given square
/// @side-effect: modifies the attack table to contain the bitboard(s) for
///               the given rook square
const fn new_rook_square_magics(offset: &mut u64, square: Square, table: &mut [Bitboard]) -> Magic {
    let mask = rook_mask(square).const_unwrap();

    let bits = mask.count_ones();
    // number of set bits in the mask
    let permutations = 1u64 << bits;
    // number of blocker boards to be indexed

    // create the magic for the given square
    let magic = Magic::new(
        mask,
        (64 - bits) as u8,
        // shift
        *offset,
        ROOK_MAGIC_NUMS[square.idx()],
    );

    // index the attack boards for the given square
    //
    // note: this loop uses the Carry-Rippler method to iterate through
    //       all the possible blocker boards for the given mask
    let mut next = 0;
    let mut n: u64 = 0;
    while next < permutations {
        let blocker_board = Bitboard::new(n);
        let index = magic.idx(blocker_board);

        // get the respective attack board for the given square and blocker
        // board
        table[index] = rook_attack_board(square, blocker_board);

        next += 1;
        n = n.wrapping_sub(mask) & mask;
    }

    // increment the offset for the next magic
    *offset += permutations;

    magic
}

/// new_bishop_square_magics creates a new magic for the given square
///
/// note: this function is copy-pasted to improve CTFE performance by avoiding
///       branching in the hot path
///
/// @param: offset - offset into the existing attack table
/// @param: square - square to create the magic for
/// @param: table - attack table to create the magic for
/// @return: new magic for the given square
/// @side-effect: modifies the attack table to contain the bitboard(s) for
///               the given square
const fn new_bishop_square_magics(
    offset: &mut u64,
    square: Square,
    table: &mut [Bitboard],
) -> Magic {
    let mask = bishop_mask(square).const_unwrap();

    let bits = mask.count_ones();
    // number of set bits in the mask
    let permutations = 1u64 << bits;
    // number of blocker boards to be indexed

    // create the magic for the given square
    let magic = Magic::new(
        mask,
        (64 - bits) as u8,
        // shift
        *offset,
        BISHOP_MAGIC_NUMS[square.idx()],
    );

    // index the attack boards for the given square
    //
    // note: this loop uses the Carry-Rippler method to iterate through
    //       all the possible blocker boards for the given mask
    let mut next = 0;
    let mut n: u64 = 0;
    while next < permutations {
        let blocker_board = Bitboard::new(n);
        let index = magic.idx(blocker_board);

        // get the respective attack board for the given square and blocker
        // board
        table[index] = bishop_attack_board(square, blocker_board);

        next += 1;
        n = n.wrapping_sub(mask) & mask;
    }

    // increment the offset for the next magic
    *offset += permutations;

    magic
}

/// new_rook_magics creates a new magics table for the rook
///
/// @param: table - attack table to create the magic for
/// @return: new magic table for the given piece
/// @side-effect: modifies the attack table to contain the bitboards for the
///               given piece
const fn new_rook_magics(table: &mut [Bitboard]) -> MagicTable {
    let mut magics: MagicTable = [Magic::default(); Square::TOTAL];

    // initialize the mutable offset for the magics
    let mut offset = 0;

    // initialize the magics for each square
    let mut sq = 0;
    while sq < Square::TOTAL {
        magics[sq] = new_rook_square_magics(&mut offset, Square::from_idx(sq), table);
        sq += 1;
    }

    magics
}

/// new_bishop_magics creates a new magics table for the bishop
///
/// @param: table - attack table to create the magic for
/// @return: new magic table for the bishop
/// @side-effect: modifies the attack table to contain the bitboards for the
///               bishop
const fn new_bishop_magics(table: &mut [Bitboard]) -> MagicTable {
    let mut magics: MagicTable = [Magic::default(); Square::TOTAL];

    // initialize the mutable offset for the magics
    let mut offset = 0;

    // initialize the magics for each square
    let mut sq = 0;
    while sq < Square::TOTAL {
        magics[sq] = new_bishop_square_magics(&mut offset, Square::from_idx(sq), table);
        sq += 1;
    }

    magics
}
