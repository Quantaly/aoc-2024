use anyhow::Result;
use aoc_2024::input_string;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/bin/13/grammar.pest"]
struct InputParser;

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

fn checked_exact_div(dividend: i64, divisor: i64) -> Option<i64> {
    if divisor == 0 {
        None
    } else if dividend % divisor != 0 {
        None
    } else {
        dividend.checked_div(divisor)
    }
}

impl Machine {
    #[allow(
        non_snake_case,
        reason = "I'm doing algebra here, I need short and similar variable names"
    )]
    fn cost(&self) -> Option<i64> {
        let &Machine { a, b, prize: p } = self;
        // p.0 == A*a.0 + B*b.0
        // p.1 == A*a.1 + B*b.1
        //
        // uhh I definitely remember how to solve systems of linear equations
        //
        // p.0*b.1 == A*a.0*b.1 + B*b.0*b.1
        // p.1*b.0 == A*a.1*b.0 + B*b.1*b.0
        //
        // p.0*b.1 - p.1*b.0 == A*a.0*b.1 - A*a.1*b.0
        // p.0*b.1 - p.1*b.0 == A*(a.0*b.1 - a.1*b.0)
        // A == (p.0*b.1 - p.1*b.0) / (a.0*b.1 - a.1*b.0)
        let A = checked_exact_div(p.0 * b.1 - p.1 * b.0, a.0 * b.1 - a.1 * b.0)?;
        // B == (p.0 - A*a.0) / b.0
        let B = checked_exact_div(p.0 - A * a.0, b.0)?;
        Some(3 * A + B)
    }

    fn adjust_for_part_2(&mut self) {
        self.prize.0 += 10000000000000;
        self.prize.1 += 10000000000000;
    }
}

fn main() -> Result<()> {
    let input = input_string()?;
    let mut machines: Vec<_> = InputParser::parse(Rule::file, &input)?
        .filter(|machine| machine.as_rule() == Rule::machine)
        .map(|machine| {
            let mut machine = machine.into_inner();
            let a = (
                machine.next().unwrap().as_str().parse()?,
                machine.next().unwrap().as_str().parse()?,
            );
            let b = (
                machine.next().unwrap().as_str().parse()?,
                machine.next().unwrap().as_str().parse()?,
            );
            let prize = (
                machine.next().unwrap().as_str().parse()?,
                machine.next().unwrap().as_str().parse()?,
            );
            Ok(Machine { a, b, prize })
        })
        .collect::<Result<_>>()?;

    let part_1_cost = machines.iter().filter_map(Machine::cost).sum::<i64>();
    println!("{part_1_cost}");

    machines.iter_mut().for_each(Machine::adjust_for_part_2);
    let part_2_cost = machines.iter().filter_map(Machine::cost).sum::<i64>();
    println!("{part_2_cost}");

    Ok(())
}
