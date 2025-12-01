use crate::primitives::File;
use std::fmt;

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