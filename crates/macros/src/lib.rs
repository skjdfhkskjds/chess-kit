use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod tuple;
mod enum_types;

#[proc_macro_derive(BitOps)]
pub fn derive_bitops(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match tuple::bitops::expand_bitops(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro_derive(Arithmetic)]
pub fn derive_arithmetic(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match tuple::arithmetic::expand_arithmetic(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Derives the bitwise operator suite for fieldless `#[repr(<unsigned>)]` enums.
///
/// The enum must list every discriminant that can be produced by the operators.
/// If a bit operation yields a value without a corresponding variant the program
/// will have undefined behaviour because the macro converts through
/// `mem::transmute`.
///
/// ```
/// use chess_kit_derive::EnumBitOps;
///
/// #[derive(EnumBitOps, Copy, Clone, PartialEq, Eq)]
/// #[repr(u8)]
/// enum Mask {
///     Empty = 0,
///     FileA = 0b0000_0001,
///     FileB = 0b0000_0010,
///     FileAB = 0b0000_0011,
/// }
///
/// let mask = Mask::FileA | Mask::FileB;
/// assert_eq!(mask as u8, Mask::FileAB as u8);
/// ```
#[proc_macro_derive(EnumBitOps)]
pub fn derive_enum_bitops(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match enum_types::bitops::expand_enum_bitops(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Implements `idx`, `from_idx`, and `from_idx_safe` for fieldless enums whose
/// implicit discriminants form a contiguous range starting at zero.
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
/// ```
#[proc_macro_derive(IndexableEnum)]
pub fn derive_indexable_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match enum_types::indexable::expand_indexable_enum(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
