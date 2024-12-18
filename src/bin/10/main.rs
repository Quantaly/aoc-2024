use anyhow::Result;
use aoc_2024::board::{input_board, Board, BoardExt};
use pathfinding::prelude::count_paths;

fn find_all(board: &Board, value: u8) -> Vec<(usize, usize)> {
    board
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &cell)| cell == value)
                .map(move |(j, _)| (i, j))
        })
        .collect()
}

fn main() -> Result<()> {
    let board = input_board()?;
    let board = &board;

    let trailheads = find_all(board, b'0');
    let summits = find_all(board, b'9');

    let mut score_sum = 0;
    let mut rating_sum = 0;

    for &trailhead in &trailheads {
        for &summit in &summits {
            let trail_count = count_paths(
                trailhead,
                |&(i, j)| {
                    board
                        .get_at((i, j))
                        .map(|height| {
                            [
                                i.checked_sub(1).map(|i| (i, j)),
                                i.checked_add(1).map(|i| (i, j)),
                                j.checked_sub(1).map(|j| (i, j)),
                                j.checked_add(1).map(|j| (i, j)),
                            ]
                            .into_iter()
                            .flatten()
                            .filter(move |&(i, j)| {
                                board
                                    .get_at((i, j))
                                    .map(|new_height| new_height == height + 1)
                                    .unwrap_or(false)
                            })
                        })
                        .into_iter()
                        .flatten()
                },
                |&pos| pos == summit,
            );
            if trail_count > 0 {
                score_sum += 1;
            }
            rating_sum += trail_count;
        }
    }

    println!("{score_sum}");
    println!("{rating_sum}");

    Ok(())
}
