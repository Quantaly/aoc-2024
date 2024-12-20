use anyhow::{anyhow, Result};
use aoc_2024::board::{input_board, Board, BoardExt, Direction};
use pathfinding::prelude::{dijkstra_all, dijkstra_partial};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CheatingRacePosition {
    position: (usize, usize),
    cheating: Cheating,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cheating {
    Available,
    Started((usize, usize), u32),
    Done((usize, usize), (usize, usize)),
}

fn find_cheating_savings_counts(
    board: &Board,
    start: (usize, usize),
    end: (usize, usize),
    costs_to_end: &HashMap<(usize, usize), u32>,
    cheat_duration: u32,
) -> BTreeMap<u32, u32> {
    let (node_map, _) = dijkstra_partial(
        &CheatingRacePosition {
            position: start,
            cheating: Cheating::Available,
        },
        |&CheatingRacePosition { position, cheating }| {
            [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ]
            .into_iter()
            .filter_map(move |dir| dir.do_move(position))
            .flat_map(
                move |new_position| match (board.get_at(new_position), cheating) {
                    (None, _) => [None, None],
                    (Some(b'#'), Cheating::Available) => [
                        Some((
                            CheatingRacePosition {
                                position: new_position,
                                cheating: Cheating::Started(position, cheat_duration - 2),
                            },
                            1,
                        )),
                        None,
                    ],
                    (Some(b'#'), Cheating::Started(cheat_start, cheat_time_remaining)) => [
                        if cheat_time_remaining > 0 {
                            Some((
                                CheatingRacePosition {
                                    position: new_position,
                                    cheating: Cheating::Started(
                                        cheat_start,
                                        cheat_time_remaining - 1,
                                    ),
                                },
                                1,
                            ))
                        } else {
                            None
                        },
                        None,
                    ],
                    (Some(b'#'), _) => [None, None],
                    (Some(b'E'), Cheating::Started(cheat_start, _)) => [
                        Some((
                            CheatingRacePosition {
                                position: new_position,
                                cheating: Cheating::Done(cheat_start, new_position),
                            },
                            1,
                        )),
                        None,
                    ],
                    (Some(_), Cheating::Started(cheat_start, cheat_time_remaining)) => [
                        Some((
                            CheatingRacePosition {
                                position: end,
                                cheating: Cheating::Done(cheat_start, new_position),
                            },
                            costs_to_end
                                .get(&new_position)
                                .map(|&cost| cost + 1)
                                .unwrap_or(1),
                        )),
                        if cheat_time_remaining > 0 {
                            Some((
                                CheatingRacePosition {
                                    position: new_position,
                                    cheating: Cheating::Started(
                                        cheat_start,
                                        cheat_time_remaining - 1,
                                    ),
                                },
                                1,
                            ))
                        } else {
                            None
                        },
                    ],
                    (Some(_), _) => [
                        Some((
                            CheatingRacePosition {
                                position: new_position,
                                cheating,
                            },
                            1,
                        )),
                        None,
                    ],
                },
            )
            .flatten()
        },
        |&CheatingRacePosition { position, cheating }| {
            position == end && cheating == Cheating::Available
        },
    );

    let (_, cost_without_cheating) = node_map[&CheatingRacePosition {
        position: end,
        cheating: Cheating::Available,
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
        [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
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
