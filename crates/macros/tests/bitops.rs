#![allow(clippy::op_ref)]

use chess_kit_derive::BitOps;

#[derive(BitOps, Copy, Clone, Debug, PartialEq, Eq)]
struct Flags(u8);

#[derive(BitOps, Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
enum Mask {
    Empty = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

#[test]
fn tuple_wrapper_operator_suite() {
    let lhs = Flags(0b1010);
    let rhs = Flags(0b1100);

    assert_eq!(lhs | rhs, Flags(0b1110));
    assert_eq!(&lhs & rhs, Flags(0b1000));
    assert_eq!(lhs ^ &rhs, Flags(0b0110));
    assert_eq!(!lhs, Flags(0b1111_0101));
    assert_eq!(lhs << 2_u8, Flags(0b0010_1000));
    assert_eq!(&lhs >> 1_u32, Flags(0b0101));

    let mut value = lhs;
    value |= rhs;
    value &= &Flags(0b1011);
    value ^= 0b0011_u8;
    value <<= 1_u64;
    value >>= Flags(1);
    assert_eq!(value, Flags(0b1001));

    assert_eq!(Flags::from(7_u8).const_unwrap(), 7);
    assert_eq!(u64::from(Flags(9)), 9);
}

#[test]
fn enum_operator_suite_uses_the_same_derive() {
    assert_eq!(Mask::One | Mask::Two, Mask::Three);
    assert_eq!(&Mask::Three & Mask::One, Mask::One);
    assert_eq!(Mask::Three ^ &Mask::One, Mask::Two);
    assert_eq!(Mask::One << 1_u8, Mask::Two);
    assert_eq!(&Mask::Two >> 1_u32, Mask::One);

    let mut value = Mask::One;
    value |= Mask::Two;
    value &= &Mask::Three;
    value ^= 1_u8;
    value >>= Mask::One;
    value <<= 1_u64;
    assert_eq!(value, Mask::Two);

    assert_eq!(Mask::Empty | 1_u8, Mask::One);
}
