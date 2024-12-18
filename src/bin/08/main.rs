use anyhow::Result;
use aoc_2024::board::{input_board, Board, BoardExt};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
struct AntinodeIterator<'a> {
    board: &'a Board,
    pos: (usize, usize),
    movement_mag: (usize, usize),
    movement_add: (bool, bool),
}

impl<'a> AntinodeIterator<'a> {
    fn new(board: &'a Board, (i1, j1): (usize, usize), (i2, j2): (usize, usize)) -> Self {
        let i_diff = i1.abs_diff(i2);
        let j_diff = j1.abs_diff(j2);
        Self {
            board,
            pos: (i1, j1),
            movement_mag: (i_diff, j_diff),
            movement_add: (i1 > i2, j1 > j2),
        }
    }
}

impl Iterator for AntinodeIterator<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (i, j) = self.pos;
        let (i_diff, j_diff) = self.movement_mag;
        let (i_add, j_add) = self.movement_add;
        let next_pos = (if i_add {
            i.checked_add(i_diff)
        } else {
            i.checked_sub(i_diff)
        })
        .zip(if j_add {
            j.checked_add(j_diff)
        } else {
            j.checked_sub(j_diff)
        });
        if let Some(next_pos) = next_pos {
            if self.board.get_at(next_pos).is_some() {
                self.pos = next_pos;
                return Some(next_pos);
            }
        }
        None
    }
}

fn main() -> Result<()> {
    let board = input_board()?;

    let mut antennae = BTreeMap::new();
    for (i, row) in board.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell != b'.' {
                antennae.entry(cell).or_insert(Vec::new()).push((i, j));
            }
        }
    }

    let mut first_antinodes = BTreeSet::new();
    let mut all_antinodes = BTreeSet::new();
    for antennae in antennae.values() {
        for (i, &a1) in antennae.iter().enumerate() {
            for &a2 in antennae.iter().skip(i + 1) {
                all_antinodes.insert(a1);
                all_antinodes.insert(a2);
                for (i, antinode) in AntinodeIterator::new(&board, a1, a2)
                    .enumerate()
                    .chain(AntinodeIterator::new(&board, a2, a1).enumerate())
                {
                    if i == 0 {
                        first_antinodes.insert(antinode);
                    }
                    all_antinodes.insert(antinode);
                }
            }
        }
    }

    println!("{}", first_antinodes.len());
    println!("{}", all_antinodes.len());

    Ok(())
}
