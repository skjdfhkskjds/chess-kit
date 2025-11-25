use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Default, Hash)]
pub struct Square(usize);

impl Square {
    // new creates a new square with the given usize value
    //
    // @param: square - usize value to create the square from
    // @return: new square
    #[inline(always)]
    pub const fn new(square: usize) -> Self {
        Self(square)
    }

    // unwrap unwraps the square to get the underlying usize value
    //
    // @param: self - immutable reference to the square
    // @return: underlying usize value
    #[inline(always)]
    pub fn unwrap(&self) -> usize {
        self.0
    }

    // rank returns the rank of the square
    //
    // @param: self - immutable reference to the square
    // @return: rank of the square
    #[inline(always)]
    pub const fn rank(&self) -> Rank {
        (self.0 / 8) as Rank
    }

    // file returns the file of the square
    //
    // @param: self - immutable reference to the square
    // @return: file of the square
    #[inline(always)]
    pub const fn file(&self) -> File {
        (self.0 % 8) as File
    }

    // is_white returns true if the square is a white square
    //
    // @param: self - immutable reference to the square
    // @return: true if the square is a white square, false otherwise
    #[inline(always)]
    pub fn is_white(&self) -> bool {
        let even_rank = ((self.0 / 8) & 1) == 0;
        let even_square = (self.0 & 1) == 0;
        even_rank ^ even_square
    }
}

impl TryFrom<&str> for Square {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() != 2 {
            return Err("invalid square");
        }

        let file_str = s.chars().next().unwrap();
        let rank_str = s.chars().nth(1).unwrap();

        let file = match file_str {
            'A' => Files::A,
            'B' => Files::B,
            'C' => Files::C,
            'D' => Files::D,
            'E' => Files::E,
            'F' => Files::F,
            'G' => Files::G,
            'H' => Files::H,
            _ => return Err("invalid square"),
        };

        let rank = match rank_str {
            '1' => Ranks::R1,
            '2' => Ranks::R2,
            '3' => Ranks::R3,
            '4' => Ranks::R4,
            '5' => Ranks::R5,
            '6' => Ranks::R6,
            '7' => Ranks::R7,
            '8' => Ranks::R8,
            _ => return Err("invalid square"),
        };

        Ok(Square::new(((rank as usize) * 8) + (file as usize)))
    }
}

// ================================================
//               bitwise operations
// ================================================

impl BitOr for Square {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Square {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for Square {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Square {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitXor for Square {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Square {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for Square {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.file() {
            Files::A => write!(f, "A"),
            Files::B => write!(f, "B"),
            Files::C => write!(f, "C"),
            Files::D => write!(f, "D"),
            Files::E => write!(f, "E"),
            Files::F => write!(f, "f"),
            Files::G => write!(f, "G"),
            Files::H => write!(f, "H"),
            _ => return Err(fmt::Error),
        }?;
        match self.rank() {
            Ranks::R1 => write!(f, "1"),
            Ranks::R2 => write!(f, "2"),
            Ranks::R3 => write!(f, "3"),
            Ranks::R4 => write!(f, "4"),
            Ranks::R5 => write!(f, "5"),
            Ranks::R6 => write!(f, "6"),
            Ranks::R7 => write!(f, "7"),
            Ranks::R8 => write!(f, "8"),
            _ => return Err(fmt::Error),
        }?;
        Ok(())
    }
}

pub struct Squares;

impl Squares {
    pub const TOTAL: usize = 64;

    pub const A1: Square = Square::new(0);
    pub const A2: Square = Square::new(1);
    pub const A3: Square = Square::new(2);
    pub const A4: Square = Square::new(3);
    pub const A5: Square = Square::new(4);
    pub const A6: Square = Square::new(5);
    pub const A7: Square = Square::new(6);
    pub const A8: Square = Square::new(7);

    pub const B1: Square = Square::new(8);
    pub const B2: Square = Square::new(9);
    pub const B3: Square = Square::new(10);
    pub const B4: Square = Square::new(11);
    pub const B5: Square = Square::new(12);
    pub const B6: Square = Square::new(13);
    pub const B7: Square = Square::new(14);
    pub const B8: Square = Square::new(15);

    pub const C1: Square = Square::new(16);
    pub const C2: Square = Square::new(17);
    pub const C3: Square = Square::new(18);
    pub const C4: Square = Square::new(19);
    pub const C5: Square = Square::new(20);
    pub const C6: Square = Square::new(21);
    pub const C7: Square = Square::new(22);
    pub const C8: Square = Square::new(23);

    pub const D1: Square = Square::new(24);
    pub const D2: Square = Square::new(25);
    pub const D3: Square = Square::new(26);
    pub const D4: Square = Square::new(27);
    pub const D5: Square = Square::new(28);
    pub const D6: Square = Square::new(29);
    pub const D7: Square = Square::new(30);
    pub const D8: Square = Square::new(31);

    pub const E1: Square = Square::new(32);
    pub const E2: Square = Square::new(33);
    pub const E3: Square = Square::new(34);
    pub const E4: Square = Square::new(35);
    pub const E5: Square = Square::new(36);
    pub const E6: Square = Square::new(37);
    pub const E7: Square = Square::new(38);
    pub const E8: Square = Square::new(39);

    pub const F1: Square = Square::new(40);
    pub const F2: Square = Square::new(41);
    pub const F3: Square = Square::new(42);
    pub const F4: Square = Square::new(43);
    pub const F5: Square = Square::new(44);
    pub const F6: Square = Square::new(45);
    pub const F7: Square = Square::new(46);
    pub const F8: Square = Square::new(47);

    pub const G1: Square = Square::new(48);
    pub const G2: Square = Square::new(49);
    pub const G3: Square = Square::new(50);
    pub const G4: Square = Square::new(51);
    pub const G5: Square = Square::new(52);
    pub const G6: Square = Square::new(53);
    pub const G7: Square = Square::new(54);
    pub const G8: Square = Square::new(55);

    pub const H1: Square = Square::new(56);
    pub const H2: Square = Square::new(57);
    pub const H3: Square = Square::new(58);
    pub const H4: Square = Square::new(59);
    pub const H5: Square = Square::new(60);
    pub const H6: Square = Square::new(61);
    pub const H7: Square = Square::new(62);
    pub const H8: Square = Square::new(63);

    pub const ALL: [Square; Self::TOTAL] = [
        Self::A1, Self::A2, Self::A3, Self::A4, Self::A5, Self::A6, Self::A7, Self::A8,
        Self::B1, Self::B2, Self::B3, Self::B4, Self::B5, Self::B6, Self::B7, Self::B8,
        Self::C1, Self::C2, Self::C3, Self::C4, Self::C5, Self::C6, Self::C7, Self::C8,
        Self::D1, Self::D2, Self::D3, Self::D4, Self::D5, Self::D6, Self::D7, Self::D8,
        Self::E1, Self::E2, Self::E3, Self::E4, Self::E5, Self::E6, Self::E7, Self::E8,
        Self::F1, Self::F2, Self::F3, Self::F4, Self::F5, Self::F6, Self::F7, Self::F8,
        Self::G1, Self::G2, Self::G3, Self::G4, Self::G5, Self::G6, Self::G7, Self::G8,
        Self::H1, Self::H2, Self::H3, Self::H4, Self::H5, Self::H6, Self::H7, Self::H8,
    ];
}

pub type File = usize;

pub struct Files;

impl Files {
    pub const TOTAL: usize = 8;

    pub const A: File = 0;
    pub const B: File = 1;
    pub const C: File = 2;
    pub const D: File = 3;
    pub const E: File = 4;
    pub const F: File = 5;
    pub const G: File = 6;
    pub const H: File = 7;
}

pub type Rank = usize;

pub struct Ranks;

impl Ranks {
    pub const TOTAL: usize = 8;

    pub const R1: Rank = 0;
    pub const R2: Rank = 1;
    pub const R3: Rank = 2;
    pub const R4: Rank = 3;
    pub const R5: Rank = 4;
    pub const R6: Rank = 5;
    pub const R7: Rank = 6;
    pub const R8: Rank = 7;
}
