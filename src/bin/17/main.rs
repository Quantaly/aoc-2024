use anyhow::{anyhow, Result};
use aoc_2024::input_string;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/bin/17/grammar.pest"]
struct InputParser;

#[derive(Debug, Clone, Copy)]
struct ExecutionState {
    a: i64,
    b: i64,
    c: i64,
    instr_index: usize,
}

impl ExecutionState {
    fn execute(&mut self, instr: &Instruction) -> Option<i64> {
        use Opcode::*;
        self.instr_index += 1;
        match instr.opcode {
            Adv => {
                self.a /= 1 << instr.operand.value(self);
                None
            }
            Bxl => {
                self.b ^= instr.operand.value(self);
                None
            }
            Bst => {
                self.b = instr.operand.value(self) % 8;
                None
            }
            Jnz => {
                if self.a != 0 {
                    let value = instr.operand.value(self);
                    if value % 2 != 0 {
                        panic!("program jumps to a misaligned instruction");
                    }
                    self.instr_index = (instr.operand.value(self) / 2) as usize;
                }
                None
            }
            Bxc => {
                self.b ^= self.c;
                None
            }
            Out => Some(instr.operand.value(self) % 8),
            Bdv => {
                self.b = self.a / (1 << instr.operand.value(self));
                None
            }
            Cdv => {
                self.c = self.a / (1 << instr.operand.value(self));
                None
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    opcode: Opcode,
    operand: Operand,
}

impl Instruction {
    fn assemble(&self) -> [i64; 2] {
        [self.opcode.assemble(), self.operand.assemble()]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Opcode {
    fn assemble(&self) -> i64 {
        use Opcode::*;
        match self {
            Adv => 0,
            Bxl => 1,
            Bst => 2,
            Jnz => 3,
            Bxc => 4,
            Out => 5,
            Bdv => 6,
            Cdv => 7,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operand {
    Literal(i64),
    RegisterA,
    RegisterB,
    RegisterC,
}

impl Operand {
    fn assemble(&self) -> i64 {
        use Operand::*;
        match self {
            &Literal(value) => value,
            RegisterA => 4,
            RegisterB => 5,
            RegisterC => 6,
        }
    }

    fn value(&self, state: &ExecutionState) -> i64 {
        use Operand::*;
        match self {
            &Literal(value) => value,
            RegisterA => state.a,
            RegisterB => state.b,
            RegisterC => state.c,
        }
    }
}

fn main() -> Result<()> {
    let input = input_string()?;
    let mut file = InputParser::parse(Rule::file, &input)?;
    let mut state = ExecutionState {
        a: file.next().unwrap().as_str().parse()?,
        b: file.next().unwrap().as_str().parse()?,
        c: file.next().unwrap().as_str().parse()?,
        instr_index: 0,
    };
    let program: Vec<_> = file
        .filter_map(|instr| {
            let opcode = match instr.as_rule() {
                Rule::instr_adv => Opcode::Adv,
                Rule::instr_bxl => Opcode::Bxl,
                Rule::instr_bst => Opcode::Bst,
                Rule::instr_jnz => Opcode::Jnz,
                Rule::instr_bxc => Opcode::Bxc,
                Rule::instr_out => Opcode::Out,
                Rule::instr_bdv => Opcode::Bdv,
                Rule::instr_cdv => Opcode::Cdv,
                Rule::EOI => {
                    return None;
                }
                _ => unreachable!(),
            };
            let operand = match instr.into_inner().next().unwrap().as_rule() {
                Rule::lit_0 => Operand::Literal(0),
                Rule::lit_1 => Operand::Literal(1),
                Rule::lit_2 => Operand::Literal(2),
                Rule::lit_3 => Operand::Literal(3),
                Rule::lit_4 => Operand::Literal(4),
                Rule::lit_5 => Operand::Literal(5),
                Rule::lit_6 => Operand::Literal(6),
                Rule::lit_7 => Operand::Literal(7),
                Rule::reg_a => Operand::RegisterA,
                Rule::reg_b => Operand::RegisterB,
                Rule::reg_c => Operand::RegisterC,
                _ => unreachable!(),
            };
            Some(Instruction { opcode, operand })
        })
        .collect();

    let mut has_output = false;
    while let Some(instr) = program.get(state.instr_index) {
        if let Some(output) = state.execute(instr) {
            if has_output {
                print!(",");
            }
            print!("{output}");
            has_output = true;
        }
    }
    println!();

    {
        // my solution to part 2 requires some fairly particular constraints on the input
        // validate it here before trying to run my solution
        let mut has_adv_3 = false;
        let mut has_init_b = false;
        let mut has_init_c = false;
        let mut has_out = false;
        let mut has_jnz_0 = false;
        for instr in &program {
            if has_jnz_0 {
                return Err(anyhow!(
                    "this solution only works for programs without any branches except at the end"
                ));
            }
            match instr.operand {
                Operand::RegisterA => {
                    if has_adv_3 {
                        eprintln!("(my solution doesn't even work for their example, oops)");
                        return Err(anyhow!("this solution only works for programs that don't read regA after modifying it in the loop"));
                    }
                }
                Operand::RegisterB => {
                    if !has_init_b {
                        return Err(anyhow!("this solution only works for programs that initialize regB before reading it"));
                    }
                }
                Operand::RegisterC => {
                    if !has_init_c {
                        return Err(anyhow!("this solution only works for programs that initialize regC before reading it"));
                    }
                }
                _ => {}
            }
            match instr.opcode {
                Opcode::Adv => {
                    if has_adv_3 {
                        return Err(anyhow!("this solution only works for programs that only modify regA once per loop"));
                    }
                    if instr.operand != Operand::Literal(3) {
                        return Err(anyhow!("this solution only works for programs that only modify regA with an adv 3"));
                    }
                    has_adv_3 = true;
                }
                Opcode::Bxl => {
                    if !has_init_b {
                        return Err(anyhow!("this solution only works for programs that initialize regB before reading it"));
                    }
                }
                Opcode::Bst => {
                    has_init_b = true;
                }
                Opcode::Jnz => {
                    if instr.operand != Operand::Literal(0) {
                        return Err(anyhow!("this solution only works for programs whose only jnz instruction is jnz 0"));
                    }
                    has_jnz_0 = true;
                }
                Opcode::Bxc => {
                    if !has_init_b {
                        return Err(anyhow!("this solution only works for programs that initialize regB before reading it"));
                    }
                    if !has_init_c {
                        return Err(anyhow!("this solution only works for programs that initialize regC before reading it"));
                    }
                }
                Opcode::Out => {
                    if has_out {
                        return Err(anyhow!(
                            "this solution only works for programs with only one out instruction"
                        ));
                    }
                    has_out = true;
                }
                Opcode::Bdv => {
                    if has_adv_3 {
                        return Err(anyhow!("this solution only works for programs that don't read regA after modifying it in the loop"));
                    }
                    has_init_b = true;
                }
                Opcode::Cdv => {
                    if has_adv_3 {
                        return Err(anyhow!("this solution only works for programs that don't read regA after modifying it in the loop"));
                    }
                    has_init_c = true;
                }
            }
        }
        if !has_adv_3 {
            return Err(anyhow!(
                "this solution only works for programs with an adv 3 instruction"
            ));
        }
        if !has_out {
            return Err(anyhow!(
                "this solution only works for programs with an out instruction"
            ));
        }
        if !has_jnz_0 {
            return Err(anyhow!(
                "this solution only works for programs that end with a jnz 0 instruction"
            ));
        }

        let (_, loop_body) = program.split_last().unwrap();
        let mut possibilities = vec![0];
        let expected_output = program.iter().flat_map(|instr| instr.assemble());
        for value in expected_output.rev() {
            let mut new_possibilities = Vec::new();
            for possible_high_bits in possibilities {
                for possible_low_bits in 0i64..8 {
                    let possibility = (possible_high_bits << 3) | possible_low_bits;
                    let mut state = ExecutionState {
                        a: possibility,
                        b: 0,
                        c: 0,
                        instr_index: 0,
                    };
                    while let Some(instr) = loop_body.get(state.instr_index) {
                        if let Some(output) = state.execute(instr) {
                            if output == value {
                                new_possibilities.push(possibility);
                            }
                            break;
                        }
                    }
                }
            }
            possibilities = new_possibilities;
        }

        if let Some(input) = possibilities.first() {
            println!("{input}");
        } else {
            eprintln!("no solution to part 2");
        }
    }

    Ok(())
}
