#![feature(unchecked_math)]

type RANK = i32;
type FILE = i32;
#[derive(Debug)]
pub struct ChessPosition {
    rank: RANK,
    file: FILE,
}

#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            ((0..=7), (0..=7)) => Some(ChessPosition { rank, file }),
            _ => None,
        }
    }

    pub fn same_rank(&self, other: &ChessPosition) -> bool {
        self.rank == other.rank
    }

    pub fn same_file(&self, other: &ChessPosition) -> bool {
        self.file == other.file
    }

    pub fn in_diagonal(&self, other: &ChessPosition) -> bool {
        unsafe {
            self.rank.unchecked_sub(other.rank).abs() == self.file.unchecked_sub(other.file).abs()
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { pos: position }
    }
    pub fn can_attack(&self, other: &Queen) -> bool {
        self.pos.same_rank(&other.pos)
            || self.pos.same_file(&other.pos)
            || self.pos.in_diagonal(&other.pos)
    }
}
