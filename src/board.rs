use crate::input_buf_read;
use anyhow::Result;
use std::{
    io::BufRead,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Clone)]
pub struct Board(Box<[Box<[u8]>]>);

impl Board {
    pub fn get_at(&self, (i, j): (usize, usize)) -> Option<u8> {
        self.get(i).and_then(|row| row.get(j)).copied()
    }
}

impl Deref for Board {
    type Target = Box<[Box<[u8]>]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn input_board() -> Result<Board> {
    input_buf_read()?
        .split(b'\n')
        .map(|line| {
            line.map(|mut line| {
                if line.last() == Some(&b'\r') {
                    line.pop();
                }
                line.into_boxed_slice()
            })
            .map_err(Into::into)
        })
        .collect::<Result<_>>()
        .map(Board)
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
}
