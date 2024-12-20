use crate::input_buf_read;
use anyhow::Result;
use std::io::{self, BufRead};

mod private {
    pub trait Sealed {}
}

pub type Board = Box<[Box<[u8]>]>;

pub trait BoardExt: Sized + private::Sealed {
    fn read(input: impl BufRead) -> io::Result<Self>;
    fn get_at(&self, pos: impl Into<Option<(usize, usize)>>) -> Option<u8>;
    fn find_tile(&self, cell: u8) -> Option<(usize, usize)>;
}

impl private::Sealed for Board {}
impl BoardExt for Board {
    fn read(input: impl BufRead) -> io::Result<Board> {
        input
            .split(b'\n')
            .map(|line| {
                line.map(|mut line| {
                    if line.last() == Some(&b'\r') {
                        line.pop();
                    }
                    line.into_boxed_slice()
                })
            })
            .collect::<Result<_, _>>()
    }

    fn get_at(&self, pos: impl Into<Option<(usize, usize)>>) -> Option<u8> {
        pos.into()
            .and_then(|(i, j)| self.get(i).and_then(|row| row.get(j)).copied())
    }

    fn find_tile(&self, target: u8) -> Option<(usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(j, &tile)| if tile == target { Some((i, j)) } else { None })
            })
            .next()
    }
}

pub fn input_board() -> Result<Board> {
    Board::read(input_buf_read()?).map_err(Into::into)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Northwest,
    North,
    Northeast,
    West,
    East,
    Southwest,
    South,
    Southeast,
}

impl Direction {
    pub const CARDINAL: [Direction; 4] = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

    pub fn do_move(self, (i, j): (usize, usize)) -> Option<(usize, usize)> {
        use Direction::*;
        match self {
            Northwest => i.checked_sub(1).zip(j.checked_sub(1)),
            North => i.checked_sub(1).map(|i| (i, j)),
            Northeast => i.checked_sub(1).zip(j.checked_add(1)),
            West => j.checked_sub(1).map(|j| (i, j)),
            East => j.checked_add(1).map(|j| (i, j)),
            Southwest => i.checked_add(1).zip(j.checked_sub(1)),
            South => i.checked_add(1).map(|i| (i, j)),
            Southeast => i.checked_add(1).zip(j.checked_add(1)),
        }
    }

    pub fn reverse(self) -> Direction {
        use Direction::*;
        match self {
            Northwest => Southeast,
            North => South,
            Northeast => Southwest,
            West => East,
            East => West,
            Southwest => Northeast,
            South => North,
            Southeast => Northwest,
        }
    }

    pub fn clockwise(self) -> Direction {
        use Direction::*;
        match self {
            Northwest => Northeast,
            North => East,
            Northeast => Southeast,
            West => North,
            East => South,
            Southwest => Northwest,
            South => West,
            Southeast => Southwest,
        }
    }

    pub fn counter_clockwise(self) -> Direction {
        use Direction::*;
        match self {
            Northwest => Southwest,
            North => West,
            Northeast => Northwest,
            West => South,
            East => North,
            Southwest => Southeast,
            South => East,
            Southeast => Northeast,
        }
    }
}
