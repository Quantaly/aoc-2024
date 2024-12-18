use anyhow::{anyhow, Result};
use aoc_2024::board::{self, Board, BoardExt, Direction};
use std::collections::BTreeSet;

fn find_start(board: &mut Board) -> Option<(usize, usize)> {
    for (i, row) in board.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            if *cell == b'^' {
                *cell = b'.';
                return Some((i, j));
            }
        }
    }
    None
}

fn turn(facing: Direction) -> Direction {
    use Direction::*;
    match facing {
        North => East,
        East => South,
        South => West,
        West => North,
        _ => unimplemented!(),
    }
}

fn main() -> Result<()> {
    let mut board = board::input_board()?;
    let start_position = find_start(&mut board).ok_or_else(|| anyhow!("no start position"))?;
    let board = board;

    let mut position = start_position;
    let mut facing = Direction::North;
    let mut visited_positions = BTreeSet::new();
    visited_positions.insert(position);
    let mut visited_states = BTreeSet::new();
    visited_states.insert((position, facing));
    let mut loop_blocks = BTreeSet::new();

    while let Some(new_position) = facing.do_move(position) {
        match board.get_at(new_position) {
            Some(b'#') => {
                facing = turn(facing);
            }
            Some(b'.') => {
                if new_position != start_position && !loop_blocks.contains(&new_position) {
                    // what if this was an obstacle?
                    let block_position = new_position;
                    let mut position = start_position;
                    let mut facing = Direction::North;
                    let mut visited_states = BTreeSet::new();
                    visited_states.insert((position, facing));

                    while let Some(new_position) = facing.do_move(position) {
                        match board.get_at(new_position) {
                            Some(b'#') => {
                                facing = turn(facing);
                            }
                            Some(b'.') => {
                                if new_position == block_position {
                                    facing = turn(facing);
                                } else {
                                    position = new_position;
                                }
                            }
                            _ => break,
                        }
                        if !visited_states.insert((position, facing)) {
                            loop_blocks.insert(block_position);
                            break;
                        }
                    }
                }
                position = new_position;
                visited_positions.insert(position);
            }
            _ => break,
        }
        visited_states.insert((position, facing));
    }

    println!("{}", visited_positions.len());
    println!("{}", loop_blocks.len());

    Ok(())
}
