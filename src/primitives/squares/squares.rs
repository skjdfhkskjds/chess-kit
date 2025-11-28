use crate::primitives::squares::square::Square;

pub struct Squares;

impl Squares {
    pub const TOTAL: usize = 64;

    pub const A1: Square = Square::new(0);
    pub const B1: Square = Square::new(1);
    pub const C1: Square = Square::new(2);
    pub const D1: Square = Square::new(3);
    pub const E1: Square = Square::new(4);
    pub const F1: Square = Square::new(5);
    pub const G1: Square = Square::new(6);
    pub const H1: Square = Square::new(7);

    pub const A2: Square = Square::new(8);
    pub const B2: Square = Square::new(9);
    pub const C2: Square = Square::new(10);
    pub const D2: Square = Square::new(11);
    pub const E2: Square = Square::new(12);
    pub const F2: Square = Square::new(13);
    pub const G2: Square = Square::new(14);
    pub const H2: Square = Square::new(15);

    pub const A3: Square = Square::new(16);
    pub const B3: Square = Square::new(17);
    pub const C3: Square = Square::new(18);
    pub const D3: Square = Square::new(19);
    pub const E3: Square = Square::new(20);
    pub const F3: Square = Square::new(21);
    pub const G3: Square = Square::new(22);
    pub const H3: Square = Square::new(23);

    pub const A4: Square = Square::new(24);
    pub const B4: Square = Square::new(25);
    pub const C4: Square = Square::new(26);
    pub const D4: Square = Square::new(27);
    pub const E4: Square = Square::new(28);
    pub const F4: Square = Square::new(29);
    pub const G4: Square = Square::new(30);
    pub const H4: Square = Square::new(31);

    pub const A5: Square = Square::new(32);
    pub const B5: Square = Square::new(33);
    pub const C5: Square = Square::new(34);
    pub const D5: Square = Square::new(35);
    pub const E5: Square = Square::new(36);
    pub const F5: Square = Square::new(37);
    pub const G5: Square = Square::new(38);
    pub const H5: Square = Square::new(39);

    pub const A6: Square = Square::new(40);
    pub const B6: Square = Square::new(41);
    pub const C6: Square = Square::new(42);
    pub const D6: Square = Square::new(43);
    pub const E6: Square = Square::new(44);
    pub const F6: Square = Square::new(45);
    pub const G6: Square = Square::new(46);
    pub const H6: Square = Square::new(47);

    pub const A7: Square = Square::new(48);
    pub const B7: Square = Square::new(49);
    pub const C7: Square = Square::new(50);
    pub const D7: Square = Square::new(51);
    pub const E7: Square = Square::new(52);
    pub const F7: Square = Square::new(53);
    pub const G7: Square = Square::new(54);
    pub const H7: Square = Square::new(55);

    pub const A8: Square = Square::new(56);
    pub const B8: Square = Square::new(57);
    pub const C8: Square = Square::new(58);
    pub const D8: Square = Square::new(59);
    pub const E8: Square = Square::new(60);
    pub const F8: Square = Square::new(61);
    pub const G8: Square = Square::new(62);
    pub const H8: Square = Square::new(63);

    #[rustfmt::skip]
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
