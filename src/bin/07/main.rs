use anyhow::Result;
use aoc_2024::input_buf_read;
use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
    Cat,
}

fn next_ops(ops: &mut [Op]) -> bool {
    use Op::*;
    for o in ops {
        match o {
            Add => {
                *o = Mul;
                return true;
            }
            Mul => {
                *o = Cat;
                return true;
            }
            Cat => {
                *o = Add;
            }
        }
    }
    false
}

fn main() -> Result<()> {
    let equations: Vec<(u64, Vec<u64>)> = input_buf_read()?
        .lines()
        .map(|line| {
            let line = line?;
            let mut split = line.split(": ");
            let test_value = split.next().unwrap_or("").parse()?;
            let values = split
                .next()
                .unwrap_or("")
                .split_ascii_whitespace()
                .map(str::parse)
                .collect::<Result<_, _>>()?;
            Ok((test_value, values))
        })
        .collect::<Result<_>>()?;

    let mut part_1_sum = 0;
    let mut part_2_sum = 0;

    'eqn_loop: for (test_value, values) in &equations {
        let test_value = *test_value;
        let mut ops = vec![Op::Add; values.len() - 1];
        let mut success_with_cat = false;
        'op_loop: loop {
            let mut acc = values[0];
            let mut any_cat = false;
            for (&x, &op) in values.iter().skip(1).zip(&ops) {
                acc = match op {
                    Op::Add => acc + x,
                    Op::Mul => acc * x,
                    Op::Cat => {
                        any_cat = true;
                        let mut decimal_shift = 10;
                        while decimal_shift <= x {
                            decimal_shift *= 10;
                        }
                        acc * decimal_shift + x
                    }
                };
                if acc > test_value {
                    if !next_ops(&mut ops) {
                        break 'op_loop;
                    }
                    continue 'op_loop;
                }
            }

            if acc == test_value {
                if any_cat {
                    success_with_cat = true;
                } else {
                    part_1_sum += test_value;
                    part_2_sum += test_value;
                    continue 'eqn_loop;
                }
            }

            if !next_ops(&mut ops) {
                break;
            }
        }

        if success_with_cat {
            part_2_sum += test_value;
        }
    }

    println!("{part_1_sum}");
    println!("{part_2_sum}");

    Ok(())
}
