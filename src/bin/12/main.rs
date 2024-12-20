use anyhow::Result;
use aoc_2024::board::{input_board, BoardExt, Direction};
use pathfinding::prelude::components;
use std::collections::HashSet;

fn main() -> Result<()> {
    let board = input_board()?;
    let board = &board;

    let groups: Vec<Vec<(usize, usize)>> = board
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().map(move |(j, &cell)| {
                [
                    Some((i, j)),
                    Direction::North.do_move((i, j)),
                    Direction::South.do_move((i, j)),
                    Direction::East.do_move((i, j)),
                    Direction::West.do_move((i, j)),
                ]
                .into_iter()
                .flatten()
                .filter(|&plot| board.get_at(plot) == Some(cell))
                .collect()
            })
        })
        .collect();

    let regions = components(&groups);
    let cost_sum = regions
        .iter()
        .map(|region| {
            let area = region.len();
            let perimeter = region
                .iter()
                .flat_map(|&plot| {
                    Direction::CARDINAL.into_iter().filter(move |dir| {
                        if let Some(outside) = dir.do_move(plot) {
                            !region.contains(&outside)
                        } else {
                            true
                        }
                    })
                })
                .count();
            area * perimeter
        })
        .sum::<usize>();
    println!("{cost_sum}");

    let discount_cost_sum = regions
        .iter()
        .map(|region| {
            let area = region.len();
            let border_plots = region
                .iter()
                .flat_map(|&plot| {
                    Direction::CARDINAL.into_iter().filter_map(move |dir| {
                        if let Some(outside) = dir.do_move(plot) {
                            if region.contains(&outside) {
                                None
                            } else {
                                Some((dir, plot))
                            }
                        } else {
                            Some((dir, plot))
                        }
                    })
                })
                .collect::<HashSet<_>>();
            let border_groups: Vec<Vec<(Direction, (usize, usize))>> = border_plots
                .iter()
                .map(|&(dir, plot)| {
                    [
                        Some(plot),
                        Direction::North.do_move(plot),
                        Direction::South.do_move(plot),
                        Direction::East.do_move(plot),
                        Direction::West.do_move(plot),
                    ]
                    .into_iter()
                    .flatten()
                    .filter_map(|plot| {
                        if border_plots.contains(&(dir, plot)) {
                            Some((dir, plot))
                        } else {
                            None
                        }
                    })
                    .collect()
                })
                .collect();
            let borders = components(&border_groups);
            let side_count = borders.len();
            area * side_count
        })
        .sum::<usize>();
    println!("{discount_cost_sum}");

    Ok(())
}
