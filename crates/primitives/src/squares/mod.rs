mod display;

use crate::{File, Rank};
use chess_kit_derive::{EnumBitOps, IndexableEnum};

/// `Square` is an enum that represents a square on the chess board. It is intended to provide
/// a type-safe way to represent (and index on) a square on the board.
///
/// @type
#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash, EnumBitOps, IndexableEnum)]
#[rustfmt::skip]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    pub const TOTAL: usize = 64;

    /// ALL is a constant array of all squares on the board
    #[rustfmt::skip]
    pub const ALL: [Square; Self::TOTAL] = [
        Self::A1, Self::B1, Self::C1, Self::D1, Self::E1, Self::F1, Self::G1, Self::H1,
        Self::A2, Self::B2, Self::C2, Self::D2, Self::E2, Self::F2, Self::G2, Self::H2,
        Self::A3, Self::B3, Self::C3, Self::D3, Self::E3, Self::F3, Self::G3, Self::H3,
        Self::A4, Self::B4, Self::C4, Self::D4, Self::E4, Self::F4, Self::G4, Self::H4,
        Self::A5, Self::B5, Self::C5, Self::D5, Self::E5, Self::F5, Self::G5, Self::H5,
        Self::A6, Self::B6, Self::C6, Self::D6, Self::E6, Self::F6, Self::G6, Self::H6,
        Self::A7, Self::B7, Self::C7, Self::D7, Self::E7, Self::F7, Self::G7, Self::H7,
        Self::A8, Self::B8, Self::C8, Self::D8, Self::E8, Self::F8, Self::G8, Self::H8,
    ];

    /// INVERTED is a constant array of squares in the rank-opposite order of ALL
    ///
    /// note: this is sometimes useful in scenarios where we need to find board
    ///       squares relative to a particular side's perspective
    #[rustfmt::skip]
    pub const INVERTED: [Square; Self::TOTAL] = [
        Self::A8, Self::B8, Self::C8, Self::D8, Self::E8, Self::F8, Self::G8, Self::H8,
        Self::A7, Self::B7, Self::C7, Self::D7, Self::E7, Self::F7, Self::G7, Self::H7,
        Self::A6, Self::B6, Self::C6, Self::D6, Self::E6, Self::F6, Self::G6, Self::H6,
        Self::A5, Self::B5, Self::C5, Self::D5, Self::E5, Self::F5, Self::G5, Self::H5,
        Self::A4, Self::B4, Self::C4, Self::D4, Self::E4, Self::F4, Self::G4, Self::H4,
        Self::A3, Self::B3, Self::C3, Self::D3, Self::E3, Self::F3, Self::G3, Self::H3,
        Self::A2, Self::B2, Self::C2, Self::D2, Self::E2, Self::F2, Self::G2, Self::H2,
        Self::A1, Self::B1, Self::C1, Self::D1, Self::E1, Self::F1, Self::G1, Self::H1,
    ];
}

impl Square {
    /// new creates a new square from the given file and rank
    ///
    /// @param: file - file to create the square from
    /// @param: rank - rank to create the square from
    /// @return: new square
    #[inline(always)]
    pub const fn new(file: File, rank: Rank) -> Self {
        Self::from_idx((rank.idx() << 3) + file.idx())
    }

    /// rank returns the rank of the square
    ///
    /// @param: self - immutable reference to the square
    /// @return: rank of the square
    #[inline(always)]
    pub const fn rank(&self) -> Rank {
        Rank::from_idx(self.idx() >> 3)
    }

    /// file returns the file of the square
    ///
    /// @param: self - immutable reference to the square
    /// @return: file of the square
    #[inline(always)]
    pub const fn file(&self) -> File {
        File::from_idx(self.idx() & 7)
    }

    /// is_white returns true if the square is a white square
    ///
    /// @param: self - immutable reference to the square
    /// @return: true if the square is a white square, false otherwise
    #[inline(always)]
    pub const fn is_white(&self) -> bool {
        let even_rank = (self.rank().idx() & 1) == 0;
        let even_square = (self.idx() & 1) == 0;
        even_rank ^ even_square
    }

    /// distance returns the distance between two squares
    ///
    /// @param: self - immutable reference to the square
    /// @param: other - square to calculate the distance to
    /// @return: distance between the two squares
    #[inline(always)]
    pub const fn distance(&self, other: Square) -> u8 {
        (self.idx() as i8 ^ other.idx() as i8) as u8
    }

    /// on_rank returns true if the square is on the given rank
    ///
    /// @param: self - immutable reference to the square
    /// @param: rank - rank to check
    /// @return: true if the square is on the given rank, false otherwise
    #[inline(always)]
    pub const fn on_rank(&self, rank: Rank) -> bool {
        self.rank().idx() == rank.idx()
    }

    /// on_file returns true if the square is on the given file
    ///
    /// @param: self - immutable reference to the square
    /// @param: file - file to check
    /// @return: true if the square is on the given file, false otherwise
    #[inline(always)]
    pub const fn on_file(&self, file: File) -> bool {
        self.file().idx() == file.idx()
    }
}
