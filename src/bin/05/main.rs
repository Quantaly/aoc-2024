use anyhow::Result;
use aoc_2024::input_string;
use pest::Parser;
use pest_derive::Parser;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Parser)]
#[grammar = "src/bin/05/grammar.pest"]
struct InputParser;

fn main() -> Result<()> {
    let input = input_string()?;
    let mut file = InputParser::parse(Rule::file, &input)?;

    let mut prereqs = BTreeMap::new();
    for ordering_rule in file
        .next()
        .unwrap()
        .into_inner()
        .map(|ordering_rule| -> Result<_> {
            let mut ordering_rule = ordering_rule.into_inner();
            let needed = ordering_rule.next().unwrap().as_str().parse::<u32>()?;
            let by = ordering_rule.next().unwrap().as_str().parse::<u32>()?;
            Ok((needed, by))
        })
    {
        let (needed, by) = ordering_rule?;
        prereqs
            .entry(by)
            .or_insert_with(BTreeSet::new)
            .insert(needed);
    }

    let mut already_correct_sum = 0;
    let mut corrected_sum = 0;

    for update in file.next().unwrap().into_inner().map(|update| {
        update
            .into_inner()
            .map(|uint| uint.as_str().parse())
            .collect::<Result<Vec<u32>, _>>()
    }) {
        let mut update = update?;
        let mut corrected = false;
        'outerLoop: loop {
            let mut seen = BTreeSet::new();
            let mut unmet_prereqs: BTreeMap<u32, usize> = BTreeMap::new();
            for (i, &page) in update.iter().enumerate() {
                if let Some(&error_i) = unmet_prereqs.get(&page) {
                    update.copy_within(error_i..i, error_i + 1);
                    update[error_i] = page;
                    corrected = true;
                    continue 'outerLoop;
                } else {
                    seen.insert(page);
                    if let Some(prereqs) = prereqs.get(&page) {
                        unmet_prereqs.extend(prereqs.difference(&seen).map(|&page| (page, i)));
                    }
                }
            }
            break;
        }
        *(if corrected {
            &mut corrected_sum
        } else {
            &mut already_correct_sum
        }) += update[update.len() / 2];
    }

    println!("{already_correct_sum}");
    println!("{corrected_sum}");

    Ok(())
}
