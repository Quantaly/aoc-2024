use ahash::AHashMap;
use anyhow::{anyhow, Result};
use aoc_2024::input_buf_read;
use std::{io::BufRead, str::FromStr};

fn main() -> Result<()> {
    let input = input_buf_read()?;
    let (mut a, mut b): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let line = line?;
            let mut split = line.split_ascii_whitespace();
            let a = i32::from_str(split.next().ok_or_else(|| anyhow!("no a"))?)?;
            let b = i32::from_str(split.next().ok_or_else(|| anyhow!("no b"))?)?;
            Ok((a, b))
        })
        .collect::<Result<_>>()?;
    a.sort_unstable();
    b.sort_unstable();

    let mut difference_score = 0;
    for (&a, &b) in a.iter().zip(&b) {
        difference_score += a.abs_diff(b);
    }
    println!("{difference_score}");

    let mut b_occurrences = AHashMap::new();
    for &b in &b {
        b_occurrences.entry(b).and_modify(|x| *x += 1).or_insert(1);
    }
    let mut similarity_score = 0;
    for &a in &a {
        similarity_score += a * b_occurrences.get(&a).copied().unwrap_or(0);
    }
    println!("{similarity_score}");

    Ok(())
}
