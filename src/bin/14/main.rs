use anyhow::{anyhow, Result};
use aoc_2024::input_string;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::{collections::BTreeMap, env};

#[derive(Parser)]
#[grammar = "src/bin/14/grammar.pest"]
struct InputParser;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn position_after(&self, seconds: i32, room_width: i32, room_height: i32) -> (i32, i32) {
        (
            (self.position.0 + self.velocity.0 * seconds).rem_euclid(room_width),
            (self.position.1 + self.velocity.1 * seconds).rem_euclid(room_height),
        )
    }

    fn step(&mut self, room_width: i32, room_height: i32) {
        self.position = (
            (self.position.0 + self.velocity.0).rem_euclid(room_width),
            (self.position.1 + self.velocity.1).rem_euclid(room_height),
        )
    }
}

fn parse_int_pair(pair: Pair<'_, Rule>) -> Result<(i32, i32)> {
    let mut pairs = pair.into_inner();
    Ok((
        pairs.next().unwrap().as_str().parse()?,
        pairs.next().unwrap().as_str().parse()?,
    ))
}

fn main() -> Result<()> {
    let input = input_string()?;
    let robots: Vec<_> = InputParser::parse(Rule::file, &input)?
        .filter(|robot| robot.as_rule() == Rule::robot)
        .map(|robot| {
            let mut robot = robot.into_inner();
            let position = parse_int_pair(robot.next().unwrap())?;
            let velocity = parse_int_pair(robot.next().unwrap())?;
            Ok(Robot { position, velocity })
        })
        .collect::<Result<_>>()?;

    let room_width = env::args()
        .nth(2)
        .ok_or_else(|| anyhow!("specify room width and height on command line"))?
        .parse()?;
    let room_height = env::args()
        .nth(3)
        .ok_or_else(|| anyhow!("specify room width and height on command line"))?
        .parse()?;

    let mut positions_after_100 = BTreeMap::new();
    for r in &robots {
        positions_after_100
            .entry(r.position_after(100, room_width, room_height))
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let (top_left_count, bottom_left_count) =
        positions_after_100.range((0, 0)..(room_width / 2, 0)).fold(
            (0, 0),
            |(top_left_count, bottom_left_count), (&(_, y), count)| {
                if y < room_height / 2 {
                    (top_left_count + count, bottom_left_count)
                } else if y > room_height / 2 {
                    (top_left_count, bottom_left_count + count)
                } else {
                    (top_left_count, bottom_left_count)
                }
            },
        );
    let (top_right_count, bottom_right_count) = positions_after_100
        .range((room_width / 2 + 1, 0)..(room_width, 0))
        .fold(
            (0, 0),
            |(top_right_count, bottom_right_count), (&(_, y), count)| {
                if y < room_height / 2 {
                    (top_right_count + count, bottom_right_count)
                } else if y > room_height / 2 {
                    (top_right_count, bottom_right_count + count)
                } else {
                    (top_right_count, bottom_right_count)
                }
            },
        );
    println!(
        "{}",
        top_left_count * top_right_count * bottom_left_count * bottom_right_count
    );

    part_2(robots, room_width, room_height);

    Ok(())
}

// this was the first part I had to look up a solution for...
// I just couldn't believe that what they wanted us to do was actually look through each picture individually
// so I found https://www.youtube.com/watch?v=Zyvd-MWo7uE
// and indeed, that seems to be the intended solution :/

fn part_2(mut robots: Vec<Robot>, room_width: i32, room_height: i32) {
    use std::collections::{BTreeSet, HashSet};

    let mut seen_states = HashSet::new();
    seen_states.insert(robots.clone());
    let mut step_count = 0;
    loop {
        println!();
        println!("== Step {step_count} ==");

        let positions: BTreeSet<_> = robots.iter().map(|r| r.position).collect();
        for y in 0..room_height {
            for x in 0..room_width {
                if positions.contains(&(x, y)) {
                    print!("*");
                } else {
                    print!(".");
                }
            }
            println!();
        }

        step_count += 1;
        for r in &mut robots {
            r.step(room_width, room_height);
        }
        if !seen_states.insert(robots.clone()) {
            break;
        }
    }
}
