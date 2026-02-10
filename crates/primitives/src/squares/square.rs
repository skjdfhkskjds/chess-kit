use crate::{File, Rank, Square};

impl Square {
    // new creates a new square from the given file and rank
    //
    // @param: file - file to create the square from
    // @param: rank - rank to create the square from
    // @return: new square
    #[inline(always)]
    pub const fn new(file: File, rank: Rank) -> Self {
        Self::from_idx((rank.idx() << 3) + file.idx())
    }

    // rank returns the rank of the square
    //
    // @param: self - immutable reference to the square
    // @return: rank of the square
    #[inline(always)]
    pub const fn rank(&self) -> Rank {
        Rank::from_idx(self.idx() >> 3)
    }

    // file returns the file of the square
    //
    // @param: self - immutable reference to the square
    // @return: file of the square
    #[inline(always)]
    pub const fn file(&self) -> File {
        File::from_idx(self.idx() & 7)
    }

    // is_white returns true if the square is a white square
    //
    // @param: self - immutable reference to the square
    // @return: true if the square is a white square, false otherwise
    #[inline(always)]
    pub const fn is_white(&self) -> bool {
        let even_rank = (self.rank().idx() & 1) == 0;
        let even_square = (self.idx() & 1) == 0;
        even_rank ^ even_square
    }

    // distance returns the distance between two squares
    //
    // @param: self - immutable reference to the square
    // @param: other - square to calculate the distance to
    // @return: distance between the two squares
    #[inline(always)]
    pub const fn distance(&self, other: Square) -> u8 {
        (self.idx() as i8 ^ other.idx() as i8) as u8
    }

    // on_rank returns true if the square is on the given rank
    //
    // @param: self - immutable reference to the square
    // @param: rank - rank to check
    // @return: true if the square is on the given rank, false otherwise
    #[inline(always)]
    pub const fn on_rank(&self, rank: Rank) -> bool {
        self.rank().idx() == rank.idx()
    }

    // on_file returns true if the square is on the given file
    //
    // @param: self - immutable reference to the square
    // @param: file - file to check
    // @return: true if the square is on the given file, false otherwise
    #[inline(always)]
    pub const fn on_file(&self, file: File) -> bool {
        self.file().idx() == file.idx()
    }
}
