use chess_kit_derive::IndexableEnum;
use std::fmt;

/// `File` is an enum that represents a file on the chess board
///
/// @type
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
}

impl TryFrom<&str> for File {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() != 1 {
            return Err("invalid file");
        }

        let char = s.chars().next().unwrap();
        match char {
            'A' | 'a' => Ok(File::A),
            'B' | 'b' => Ok(File::B),
            'C' | 'c' => Ok(File::C),
            'D' | 'd' => Ok(File::D),
            'E' | 'e' => Ok(File::E),
            'F' | 'f' => Ok(File::F),
            'G' | 'g' => Ok(File::G),
            'H' | 'h' => Ok(File::H),
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
