use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod bitops;
mod enum_types;
mod tuple;

/// Derives the bitwise operator suite for supported tuple structs and enums.
///
/// Tuple structs must be non-generic and have a single primitive field.
/// Enums must be non-generic, fieldless, and use an unsigned integer `repr`.
///
/// Enum operations must only produce values represented by an enum variant.
/// Producing any other value is undefined behavior.
///
/// ```
/// use chess_kit_derive::BitOps;
///
/// #[derive(BitOps, Copy, Clone, Debug, PartialEq, Eq)]
/// struct Flags(u8);
///
/// assert_eq!(Flags(0b1010) & Flags(0b1100), Flags(0b1000));
/// assert_eq!(Flags(0b1010) | Flags(0b1100), Flags(0b1110));
/// assert_eq!(Flags(0b1010) ^ Flags(0b1100), Flags(0b0110));
/// assert_eq!(Flags(0b1010) << 2_u8, Flags(0b101000));
/// ```
///
/// ```
/// use chess_kit_derive::BitOps;
///
/// #[derive(BitOps, Copy, Clone, Debug, PartialEq, Eq)]
/// #[repr(u8)]
/// enum Mask {
///     Empty = 0,
///     FileA = 1,
///     FileB = 2,
///     FileAB = 3,
/// }
///
/// assert_eq!(Mask::FileA | Mask::FileB, Mask::FileAB);
/// ```
#[proc_macro_derive(BitOps)]
pub fn derive_bitops(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match bitops::expand_bitops(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Derives addition, subtraction, and multiplication for tuple structs with a
/// single field. Each operation takes the wrapped primitive type as its
/// right-hand operand and returns the tuple struct.
///
/// The struct must have a single field that is a primitive type.
///
/// ```
/// use chess_kit_derive::Arithmetic;
///
/// #[derive(Arithmetic, Copy, Clone, PartialEq, Eq, Debug)]
/// struct Arithmetic(i32);
///
/// let arithmetic = Arithmetic(10);
/// assert_eq!(arithmetic + 20, Arithmetic(30));
/// assert_eq!(arithmetic - 5, Arithmetic(5));
/// assert_eq!(arithmetic * 2, Arithmetic(20));
/// ```
#[proc_macro_derive(Arithmetic)]
pub fn derive_arithmetic(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match tuple::arithmetic::expand_arithmetic(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Implements `idx`, `from_idx`, and `from_idx_safe` for fieldless enums whose
/// implicit discriminants form a contiguous range starting at zero in a const
/// context. For cases where indexing is possible in non-const contexts, it also
/// allows arrays to be indexed and mutated with the enum via `Index` and
/// `IndexMut`.
///
/// `from_idx` panics when the index is outside the enum's range. Use
/// `from_idx_safe` when an invalid index should return `None` instead.
///
/// ```
/// use chess_kit_derive::IndexableEnum;
///
/// #[derive(IndexableEnum, Copy, Clone, Debug, PartialEq, Eq)]
/// #[repr(u8)]
/// enum File {
///     A, B, C, D, E, F, G, H,
/// }
///
/// let idx = File::C.idx();
/// assert_eq!(idx, 2);
/// assert_eq!(File::from_idx_safe(idx + 1), Some(File::D));
///
/// let mut labels = ["a", "b", "c", "d", "e", "f", "g", "h"];
/// assert_eq!(labels[File::C], "c");
/// labels[File::C] = "C";
/// assert_eq!(labels[File::C], "C");
/// ```
#[proc_macro_derive(IndexableEnum)]
pub fn derive_indexable_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match enum_types::indexable::expand_indexable_enum(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
