use chess_kit_derive::IndexableEnum;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, IndexableEnum)]
#[repr(u8)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub const TOTAL: usize = 8;

    // inc increments the file by one
    //
    // Note: this function is unsafe because it calls `from_idx`
    //
    // @param: self - immutable reference to the file
    // @return: file incremented by one
    #[inline(always)]
    pub fn inc(&mut self) {
        *self = Self::from_idx(self.idx() + 1);
    }

    // dec decrements the file by one
    //
    // Note: this function is unsafe because it calls `from_idx`
    //
    // @param: self - immutable reference to the file
    // @return: file decremented by one
    #[inline(always)]
    pub fn dec(&mut self) {
        *self = Self::from_idx(self.idx() - 1);
    }
}

impl TryFrom<&str> for File {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() != 1 {
            return Err("invalid file");
        }

        let char = s.chars().next().unwrap();
        match char {
            'A' => Ok(File::A),
            'B' => Ok(File::B),
            'C' => Ok(File::C),
            'D' => Ok(File::D),
            'E' => Ok(File::E),
            'F' => Ok(File::F),
            'G' => Ok(File::G),
            'H' => Ok(File::H),
            _ => Err("invalid file"),
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            File::A => write!(f, "a"),
            File::B => write!(f, "b"),
            File::C => write!(f, "c"),
            File::D => write!(f, "d"),
            File::E => write!(f, "e"),
            File::F => write!(f, "f"),
            File::G => write!(f, "g"),
            File::H => write!(f, "h"),
        }
    }
}
