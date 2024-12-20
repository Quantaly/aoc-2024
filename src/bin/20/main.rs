use anyhow::{anyhow, Result};
use aoc_2024::board::{input_board, Board, BoardExt, Direction};
use pathfinding::prelude::{dijkstra_all, dijkstra_partial, dijkstra_reach, DijkstraReachableItem};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RacePosition {
    position: (usize, usize),
    cheat: Option<((usize, usize), (usize, usize))>,
}

fn find_cheating_savings_counts(
    board: &Board,
    start: (usize, usize),
    end: (usize, usize),
    costs_to_end: &HashMap<(usize, usize), u32>,
    cheat_duration: u32,
) -> BTreeMap<u32, u32> {
    let (node_map, _) = dijkstra_partial(
        &RacePosition {
            position: start,
            cheat: None,
        },
        |&RacePosition { position, cheat }| -> Vec<_> {
            if position == end {
                // don't move from the end
                return vec![];
            }
            if cheat.is_some() {
                // skip to the end
                return vec![(
                    RacePosition {
                        position: end,
                        cheat,
                    },
                    costs_to_end.get(&position).copied().unwrap_or(0),
                )];
            } else {
                Direction::CARDINAL
                    .into_iter()
                    .filter_map(|dir| dir.do_move(position))
                    .filter_map(|new_position| match board.get_at(new_position) {
                        None => None,
                        Some(b'#') => None,
                        Some(_) => Some((
                            RacePosition {
                                position: new_position,
                                cheat,
                            },
                            1,
                        )),
                    })
                    .chain(
                        dijkstra_reach(&position, |&position, cost| {
                            (cost < cheat_duration)
                                .then(|| {
                                    Direction::CARDINAL
                                        .into_iter()
                                        .filter_map(move |dir| dir.do_move(position))
                                        .filter_map(|new_position| {
                                            match board.get_at(new_position) {
                                                None => None,
                                                Some(_) => Some((new_position, 1)),
                                            }
                                        })
                                })
                                .into_iter()
                                .flatten()
                        })
                        .filter_map(
                            |DijkstraReachableItem {
                                 node: cheat_end,
                                 total_cost,
                                 ..
                             }| {
                                match board.get_at(cheat_end) {
                                    None => None,
                                    Some(b'#') => None,
                                    Some(_) => Some((
                                        RacePosition {
                                            position: cheat_end,
                                            cheat: Some((position, cheat_end)),
                                        },
                                        total_cost,
                                    )),
                                }
                            },
                        ),
                    )
                    .collect()
            }
        },
        |&RacePosition { position, cheat }| position == end && cheat.is_none(),
    );

    let (_, cost_without_cheating) = node_map[&RacePosition {
        position: end,
        cheat: None,
    }];
    let mut savings_counts = BTreeMap::new();
    for (_, &(_, cost)) in node_map.iter().filter(|(node, _)| node.position == end) {
        if cost < cost_without_cheating {
            savings_counts
                .entry(cost_without_cheating - cost)
                .and_modify(|count| *count += 1)
                .or_insert(1u32);
        }
    }
    savings_counts
}

fn main() -> Result<()> {
    let board = input_board()?;

    let start = board
        .find_tile(b'S')
        .ok_or_else(|| anyhow!("no start position"))?;
    let end = board
        .find_tile(b'E')
        .ok_or_else(|| anyhow!("no end position"))?;

    let paths_to_end = dijkstra_all(&end, |&position| {
        Direction::CARDINAL
            .into_iter()
            .filter_map(move |dir| dir.do_move(position))
            .filter_map(|position| match board.get_at(position) {
                None => None,
                Some(b'#') => None,
                Some(_) => Some((position, 1)),
            })
    });
    let costs_to_end: HashMap<_, _> = paths_to_end
        .into_iter()
        .map(|(position, (_, cost))| (position, cost))
        .collect();

    {
        // part 1
        let savings_counts = find_cheating_savings_counts(&board, start, end, &costs_to_end, 2);
        let saved_at_least_100_count: u32 =
            savings_counts.range(100..).map(|(_, count)| count).sum();
        if saved_at_least_100_count > 0 {
            println!("{saved_at_least_100_count}");
        } else {
            println!("Part 1:");
            for (savings, count) in savings_counts {
                println!("There are {count} cheats that save {savings} picoseconds.");
            }
            println!();
        }
    }

    {
        // part 2
        let savings_counts = find_cheating_savings_counts(&board, start, end, &costs_to_end, 20);
        let saved_at_least_100_count: u32 =
            savings_counts.range(100..).map(|(_, count)| count).sum();
        if saved_at_least_100_count > 0 {
            println!("{saved_at_least_100_count}");
        } else {
            println!("Part 2:");
            for (savings, count) in savings_counts.range(50..) {
                println!("There are {count} cheats that save {savings} picoseconds.");
            }
            println!();
        }
    }

    Ok(())
}
