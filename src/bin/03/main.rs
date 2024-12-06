use anyhow::Result;
use aoc_2024::input_string;
use regex::Regex;

fn main() -> Result<()> {
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
    let enabled_block_regex =
        Regex::new(r"(?:^|do\(\))(?:[^d]|d[^o]|do[^n]|don[^']|don'[^t]|don't[^(]|don't\([^)])*")?;

    let input = input_string()?;

    let part_1_sum = mul_regex
        .captures_iter(&input)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            let a: u32 = a.parse()?;
            let b: u32 = b.parse()?;
            Ok(a * b)
        })
        .sum::<Result<u32>>()?;
    println!("{part_1_sum}");

    let part_2_sum = enabled_block_regex
        .find_iter(&input)
        .flat_map(|enabled_block| {
            mul_regex.captures_iter(enabled_block.as_str()).map(|c| {
                let (_, [a, b]) = c.extract();
                let a: u32 = a.parse()?;
                let b: u32 = b.parse()?;
                Ok(a * b)
            })
        })
        .sum::<Result<u32>>()?;
    println!("{part_2_sum}");

    Ok(())
}
