#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Default, Hash)]
pub struct Piece(pub(crate) usize);

impl Piece {
    pub const fn new(piece: usize) -> Self {
        Self(piece)
    }

    #[inline(always)]
    pub const fn unwrap(&self) -> usize {
        self.0
    }
}
