use anyhow::Result;
use aoc_2024::input_string;
use count_paths_num::count_paths_num;
use pest::Parser;
use pest_derive::Parser;

mod count_paths_num;

#[derive(Parser)]
#[grammar = "src/bin/19/grammar.pest"]
struct InputParser;

fn main() -> Result<()> {
    let input = input_string()?;
    let mut file = InputParser::parse(Rule::file, &input)?;
    let towels: Vec<_> = file
        .next()
        .unwrap()
        .into_inner()
        .map(|towel| towel.as_str())
        .collect();

    let mut valid_pattern_count = 0;
    let mut total_arrangement_count = 0;
    for pattern in file
        .next()
        .unwrap()
        .into_inner()
        .map(|pattern| pattern.as_str())
    {
        let arrangement_count: u64 = count_paths_num(
            String::new(),
            |at| {
                towels
                    .iter()
                    .map(|towel| at.clone() + towel)
                    .filter(|next| pattern.starts_with(next))
                    .collect::<Vec<_>>()
            },
            |at| at == pattern,
        );
        if arrangement_count > 0 {
            valid_pattern_count += 1;
            total_arrangement_count += arrangement_count as u64;
        }
    }

    println!("{valid_pattern_count}");
    println!("{total_arrangement_count}");

    Ok(())
}
