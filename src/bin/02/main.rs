use anyhow::Result;
use aoc_2024::input_buf_read;
use std::{io::BufRead, str::FromStr};

fn find_error(record: &[i32], skip: Option<usize>) -> Option<usize> {
    if record.len() < 2 {
        None
    } else {
        let mut has_increased = false;
        let mut has_decreased = false;
        let mut prev_level: Option<i32> = None;
        for (i, &level) in record.iter().enumerate() {
            if Some(i) != skip {
                if let Some(prev_level) = prev_level {
                    if level > prev_level {
                        has_increased = true;
                        if level > prev_level + 3 {
                            return Some(i);
                        }
                    } else if level < prev_level {
                        has_decreased = true;
                        if level < prev_level - 3 {
                            return Some(i);
                        }
                    } else {
                        return Some(i);
                    }
                    if has_increased && has_decreased {
                        return Some(i);
                    }
                }
                prev_level = Some(level)
            }
        }
        None
    }
}

fn main() -> Result<()> {
    let input = input_buf_read()?;
    let records: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line?
                .split_ascii_whitespace()
                .map(|x| i32::from_str(x).map_err(Into::into))
                .collect()
        })
        .collect::<Result<_>>()?;

    let mut safe_count = 0;
    let mut safe_dampened_count = 0;

    for record in &records {
        if let Some(error_index) = find_error(record, None) {
            for skip in (0..=error_index).rev() {
                if find_error(record, Some(skip)).is_none() {
                    safe_dampened_count += 1;
                    break;
                }
            }
        } else {
            safe_count += 1;
            safe_dampened_count += 1;
        }
    }

    println!("{safe_count}");
    println!("{safe_dampened_count}");

    Ok(())
}
