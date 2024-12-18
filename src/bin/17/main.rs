use anyhow::Result;
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

#[derive(Debug, Clone, Copy)]
struct Instruction {
    opcode: Opcode,
    operand: Operand,
}

impl Instruction {
    fn execute(&self, state: &mut ExecutionState) -> Option<i64> {
        use Opcode::*;
        state.instr_index += 1;
        match self.opcode {
            Adv => {
                state.a = state.a / (1 << self.operand.value(state));
                None
            }
            Bxl => {
                state.b ^= self.operand.value(state);
                None
            }
            Bst => {
                state.b = self.operand.value(state) % 8;
                None
            }
            Jnz => {
                if state.a != 0 {
                    let value = self.operand.value(state);
                    if value % 2 != 0 {
                        panic!("program jumps to a misaligned instruction");
                    }
                    state.instr_index = (self.operand.value(state) / 2) as usize;
                }
                None
            }
            Bxc => {
                state.b ^= state.c;
                None
            }
            Out => Some(self.operand.value(state) % 8),
            Bdv => {
                state.b = state.a / (1 << self.operand.value(state));
                None
            }
            Cdv => {
                state.c = state.a / (1 << self.operand.value(state));
                None
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
enum Operand {
    Literal(i64),
    RegisterA,
    RegisterB,
    RegisterC,
}

impl Operand {
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
        if let Some(output) = instr.execute(&mut state) {
            if has_output {
                print!(",");
            }
            print!("{output}");
            has_output = true;
        }
    }
    println!();

    eprintln!("haven't done part 2 yet");

    Ok(())
}
