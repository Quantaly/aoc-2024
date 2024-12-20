use anyhow::{anyhow, Result};
use aoc_2024::{board::Direction, input_buf_read};
use pathfinding::prelude::astar;
use std::{collections::HashSet, env, io::BufRead};

fn find_path(
    mem_side_length: usize,
    fallen_bytes: &HashSet<(usize, usize)>,
) -> Option<(Vec<(usize, usize)>, usize)> {
    astar(
        &(0, 0),
        |&pos| {
            Direction::CARDINAL
                .into_iter()
                .filter_map(move |dir| dir.do_move(pos))
                .filter_map(|pos| {
                    if pos.0 <= mem_side_length
                        && pos.1 <= mem_side_length
                        && !fallen_bytes.contains(&pos)
                    {
                        Some((pos, 1))
                    } else {
                        None
                    }
                })
        },
        |pos| mem_side_length - pos.0 + mem_side_length - pos.1,
        |&pos| pos == (mem_side_length, mem_side_length),
    )
}

fn main() -> Result<()> {
    let incoming_bytes: Vec<_> = input_buf_read()?
        .lines()
        .map(|line| {
            let line = line?;
            let mut coords = line.split(",").map(str::parse);
            Ok((
                coords.next().ok_or_else(|| anyhow!("no x"))??,
                coords.next().ok_or_else(|| anyhow!("no y"))??,
            ))
        })
        .collect::<Result<_>>()?;

    let mem_side_length = env::args()
        .nth(2)
        .ok_or_else(|| anyhow!("specify mem side length on command line"))?
        .parse()?;
    let part_1_count = env::args()
        .nth(3)
        .ok_or_else(|| anyhow!("specify how many bytes to use for part 1 on the command line"))?
        .parse()?;

    let mut fallen_bytes: HashSet<_> = incoming_bytes.iter().copied().take(part_1_count).collect();

    let (path, cost) =
        find_path(mem_side_length, &fallen_bytes).ok_or_else(|| anyhow!("no path in part 1"))?;
    println!("{cost}");
    let mut path_bytes: HashSet<_> = path.into_iter().collect();

    for new_byte in incoming_bytes.into_iter().skip(part_1_count) {
        fallen_bytes.insert(new_byte);
        if path_bytes.contains(&new_byte) {
            if let Some((path, _)) = find_path(mem_side_length, &fallen_bytes) {
                path_bytes = path.into_iter().collect();
            } else {
                println!("{},{}", new_byte.0, new_byte.1);
                return Ok(());
            }
        }
    }

    Err(anyhow!("never blocked in part 2"))
}
