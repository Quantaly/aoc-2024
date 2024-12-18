use std::collections::BTreeSet;

use anyhow::{anyhow, Result};
use aoc_2024::board::{input_board, BoardExt, Direction};
use pathfinding::prelude::astar_bag;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Orientation {
    position: (usize, usize),
    direction: Direction,
}

fn main() -> Result<()> {
    let board = input_board()?;
    let board = &board;
    let start_pos = board
        .find_tile(b'S')
        .ok_or_else(|| anyhow!("no start position"))?;
    let end_pos = board
        .find_tile(b'E')
        .ok_or_else(|| anyhow!("no end position"))?;

    let (paths, cost) = astar_bag(
        &Orientation {
            position: start_pos,
            direction: Direction::East,
        },
        |&Orientation {
             position,
             direction,
         }| {
            [
                direction.do_move(position).and_then(|new_position| {
                    match board.get_at(new_position) {
                        Some(b'#') => None,
                        Some(_) => Some((
                            Orientation {
                                position: new_position,
                                direction,
                            },
                            1,
                        )),
                        None => None,
                    }
                }),
                Some((
                    Orientation {
                        position,
                        direction: direction.clockwise(),
                    },
                    1000,
                )),
                Some((
                    Orientation {
                        position,
                        direction: direction.counter_clockwise(),
                    },
                    1000,
                )),
            ]
            .into_iter()
            .flatten()
        },
        |Orientation { position, .. }| {
            position.0.abs_diff(end_pos.0) + position.1.abs_diff(end_pos.1)
        },
        |&Orientation { position, .. }| position == end_pos,
    )
    .ok_or_else(|| anyhow!("no path"))?;
    println!("{cost}");

    let used_tile_count = paths
        .flat_map(|path| {
            path.into_iter()
                .map(|Orientation { position, .. }| position)
        })
        .collect::<BTreeSet<_>>()
        .len();
    println!("{used_tile_count}");

    Ok(())
}
