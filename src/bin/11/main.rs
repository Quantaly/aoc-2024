use anyhow::Result;
use aoc_2024::input_string;
use std::collections::BTreeMap;

fn split_digits(value: u64) -> Option<(u64, u64)> {
    let mut mid = 1;
    let mut top = 10;
    loop {
        if value < top {
            return None;
        }
        top *= 10;
        mid *= 10;
        if value < top {
            return Some((value / mid, value % mid));
        }
        top *= 10;
    }
}

struct MemoizedSuccessors(BTreeMap<(u64, u32), u64>);

impl MemoizedSuccessors {
    fn new() -> MemoizedSuccessors {
        MemoizedSuccessors(BTreeMap::new())
    }

    fn successors_count_after(&mut self, value: u64, blinks: u32) -> u64 {
        if blinks == 0 {
            1
        } else if let Some(&count) = self.0.get(&(value, blinks)) {
            count
        } else {
            let count = if value == 0 {
                self.successors_count_after(1, blinks - 1)
            } else if let Some((a, b)) = split_digits(value) {
                self.successors_count_after(a, blinks - 1)
                    + self.successors_count_after(b, blinks - 1)
            } else {
                self.successors_count_after(value * 2024, blinks - 1)
            };
            self.0.insert((value, blinks), count);
            count
        }
    }
}

fn main() -> Result<()> {
    let initial_stones: Vec<u64> = input_string()?
        .split_ascii_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    let mut total_after_25 = 0;
    let mut total_after_75 = 0;
    let mut table = MemoizedSuccessors::new();

    for value in initial_stones {
        total_after_25 += table.successors_count_after(value, 25);
        total_after_75 += table.successors_count_after(value, 75);
    }

    println!("{total_after_25}");
    println!("{total_after_75}");

    Ok(())
}
