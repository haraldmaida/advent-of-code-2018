//! # Day 16: Chronal Classification
//!
//! As you see the Elves defend their hot chocolate successfully, you go back to
//! falling through time. This is going to become a problem.
//!
//! If you're ever going to return to your own time, you need to understand how
//! this device on your wrist works. You have a little while before you reach
//! your next destination, and with a bit of trial and error, you manage to pull
//! up a programming manual on the device's tiny screen.
//!
//! According to the manual, the device has four registers (numbered 0 through
//! 3) that can be manipulated by instructions containing one of 16 opcodes. The
//! registers start with the value 0.
//!
//! Every instruction consists of four values: an opcode, two inputs (named A
//! and B), and an output (named C), in that order. The opcode specifies the
//! behavior of the instruction and how the inputs are interpreted. The output,
//! C, is always treated as a register.
//!
//! In the opcode descriptions below, if something says "value A", it means to
//! take the number given as A literally. (This is also called an "immediate"
//! value.) If something says "register A", it means to use the number given as
//! A to read from (or write to) the register with that number. So, if the
//! opcode addi adds register A and value B, storing the result in register C,
//! and the instruction addi 0 7 3 is encountered, it would add 7 to the value
//! contained by register 0 and store the sum in register 3, never modifying
//! registers 0, 1, or 2 in the process.
//!
//! Many opcodes are similar except for how they interpret their arguments. The
//! opcodes fall into seven general categories:
//!
//! Addition:
//!
//! * addr (add register) stores into register C the result of adding register A
//!   and register B.
//! * addi (add immediate) stores into register C the result of adding register
//!   A and value B.
//!
//! Multiplication:
//!
//! * mulr (multiply register) stores into register C the result of multiplying
//!   register A and register B.
//! * muli (multiply immediate) stores into register C the result of multiplying
//!   register A and value B.
//!
//! Bitwise AND:
//!
//! * banr (bitwise AND register) stores into register C the result of the
//!   bitwise AND of register A and register B.
//! * bani (bitwise AND immediate) stores into register C the result of the
//!   bitwise AND of register A and value B.
//!
//! Bitwise OR:
//!
//! * borr (bitwise OR register) stores into register C the result of the
//!   bitwise OR of register A and register B.
//! * bori (bitwise OR immediate) stores into register C the result of the
//!   bitwise OR of register A and value B.
//!
//! Assignment:
//!
//! * setr (set register) copies the contents of register A into register C.
//!   (Input B is ignored.)
//! * seti (set immediate) stores value A into register C. (Input B is ignored.)
//!
//! Greater-than testing:
//!
//! * gtir (greater-than immediate/register) sets register C to 1 if value A is
//!   greater than register B. Otherwise, register C is set to 0.
//! * gtri (greater-than register/immediate) sets register C to 1 if register A
//!   is greater than value B. Otherwise, register C is set to 0.
//! * gtrr (greater-than register/register) sets register C to 1 if register A
//!   is greater than register B. Otherwise, register C is set to 0.
//!
//! Equality testing:
//!
//! * eqir (equal immediate/register) sets register C to 1 if value A is equal
//!   to register B. Otherwise, register C is set to 0.
//! * eqri (equal register/immediate) sets register C to 1 if register A is
//!   equal to value B. Otherwise, register C is set to 0.
//! * eqrr (equal register/register) sets register C to 1 if register A is equal
//!   to register B. Otherwise, register C is set to 0.
//!
//! Unfortunately, while the manual gives the name of each opcode, it doesn't
//! seem to indicate the number. However, you can monitor the CPU to see the
//! contents of the registers before and after instructions are executed to try
//! to work them out. Each opcode has a number from 0 through 15, but the manual
//! doesn't say which is which. For example, suppose you capture the following
//! sample:
//!
//! ```text
//! Before: [3, 2, 1, 1]
//! 9 2 1 2
//! After:  [3, 2, 2, 1]
//! ```
//!
//! This sample shows the effect of the instruction 9 2 1 2 on the registers.
//! Before the instruction is executed, register 0 has value 3, register 1 has
//! value 2, and registers 2 and 3 have value 1. After the instruction is
//! executed, register 2's value becomes 2.
//!
//! The instruction itself, 9 2 1 2, means that opcode 9 was executed with A=2,
//! B=1, and C=2. Opcode 9 could be any of the 16 opcodes listed above, but only
//! three of them behave in a way that would cause the result shown in the
//! sample:
//!
//! * Opcode 9 could be mulr: register 2 (which has a value of 1) times register
//!   1 (which has a value of 2) produces 2, which matches the value stored in
//!   the output register, register 2.
//! * Opcode 9 could be addi: register 2 (which has a value of 1) plus value 1
//!   produces 2, which matches the value stored in the output register,
//!   register 2.
//! * Opcode 9 could be seti: value 2 matches the value stored in the output
//!   register, register 2; the number given for B is irrelevant.
//!
//! None of the other opcodes produce the result captured in the sample. Because
//! of this, the sample above behaves like three opcodes.
//!
//! You collect many of these samples (the first section of your puzzle input).
//! The manual also includes a small test program (the second section of your
//! puzzle input) - you can ignore it for now.
//!
//! Ignoring the opcode numbers, how many samples in your puzzle input behave
//! like three or more opcodes?
//!
//! ## Part 2
//!
//! Using the samples you collected, work out the number of each opcode and
//! execute the test program (the second section of your puzzle input).
//!
//! What value is contained in register 0 after executing the test program?
//!
//! [Advent of Code 2018 - Day 16](https://adventofcode.com/2018/day/16)

use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display},
    iter::FromIterator,
    ops::{Index, IndexMut},
    str::FromStr,
};

use self::Mnemonic::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mnemonic {
    AddR,
    AddI,
    MulR,
    MulI,
    BanR,
    BanI,
    BorR,
    BorI,
    SetR,
    SetI,
    GtIR,
    GtRI,
    GtRR,
    EqIR,
    EqRI,
    EqRR,
}

const INSTRUCTION_SET: &[Mnemonic] = &[
    AddR, AddI, MulR, MulI, BanR, BanI, BorR, BorI, SetR, SetI, GtIR, GtRI, GtRR, EqIR, EqRI, EqRR,
];

impl Display for Mnemonic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = match *self {
            AddR => "addr",
            AddI => "addi",
            MulR => "mulr",
            MulI => "muli",
            BanR => "banr",
            BanI => "bani",
            BorR => "borr",
            BorI => "bori",
            SetR => "setr",
            SetI => "seti",
            GtIR => "gtir",
            GtRI => "gtri",
            GtRR => "gtrr",
            EqIR => "eqir",
            EqRI => "eqri",
            EqRR => "eqrr",
        };
        f.write_str(display)
    }
}

impl FromStr for Mnemonic {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "addr" => AddR,
            "addi" => AddI,
            "mulr" => MulR,
            "muli" => MulI,
            "banr" => BanR,
            "bani" => BanI,
            "borr" => BorR,
            "bori" => BorI,
            "setr" => SetR,
            "seti" => SetI,
            "gtir" => GtIR,
            "gtri" => GtRI,
            "gtrr" => GtRR,
            "eqir" => EqIR,
            "eqri" => EqRI,
            "eqrr" => EqRR,
            _ => return Err(format!("unknown opcode {:?}", s)),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OpCode(pub u8);

impl Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

pub type Data = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Register([Data; 4]);

impl Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}, {}, {}, {}]",
            self.0[0], self.0[1], self.0[2], self.0[3]
        )
    }
}

impl Default for Register {
    fn default() -> Self {
        Register([0; 4])
    }
}

impl From<[Data; 4]> for Register {
    fn from(value: [Data; 4]) -> Self {
        Register(value)
    }
}

impl Index<Data> for Register {
    type Output = Data;

    fn index(&self, index: Data) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<Data> for Register {
    fn index_mut(&mut self, index: Data) -> &mut <Self as Index<Data>>::Output {
        &mut self.0[index as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instruction {
    pub opcode: OpCode,
    pub a: Data,
    pub b: Data,
    pub c: Data,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.opcode, self.a, self.b, self.c)
    }
}

impl From<(u8, Data, Data, Data)> for Instruction {
    fn from((opc, a, b, c): (u8, Data, Data, Data)) -> Self {
        Self {
            opcode: OpCode(opc),
            a,
            b,
            c,
        }
    }
}

impl Instruction {
    pub fn new(opcode: impl Into<OpCode>, a: Data, b: Data, c: Data) -> Self {
        Self {
            opcode: opcode.into(),
            a,
            b,
            c,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Interpreter {
    opcodes: HashMap<OpCode, Mnemonic>,
}

impl Interpreter {
    pub fn with_instruction_set(
        instruction_set: impl IntoIterator<Item = (OpCode, Mnemonic)>,
    ) -> Self {
        Self {
            opcodes: HashMap::from_iter(instruction_set.into_iter()),
        }
    }

    pub fn execute(&self, instruction: Instruction, register: &mut Register) -> Result<(), String> {
        let &mnemonic = self.opcodes.get(&instruction.opcode).ok_or_else(|| {
            format!(
                "unsupported opcode {} in instruction {}",
                instruction.opcode, instruction
            )
        })?;

        execute_mnemonic(mnemonic, instruction, register);

        Ok(())
    }

    pub fn run(&self, program: &[Instruction], register: &mut Register) -> Result<(), String> {
        for &instruction in program {
            self.execute(instruction, register)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sample {
    before: Register,
    instruction: Instruction,
    after: Register,
}

impl Display for Sample {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Before: {}", self.before)?;
        writeln!(f, "{}", self.instruction)?;
        writeln!(f, "After:  {}", self.after)
    }
}

impl Sample {
    pub fn new(
        before: impl Into<Register>,
        instruction: impl Into<Instruction>,
        after: impl Into<Register>,
    ) -> Self {
        Self {
            before: before.into(),
            instruction: instruction.into(),
            after: after.into(),
        }
    }

    pub fn before(self) -> Register {
        self.before
    }

    pub fn instruction(self) -> Instruction {
        self.instruction
    }

    pub fn after(self) -> Register {
        self.after
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(clippy::unneeded_field_pattern))]
fn execute_mnemonic(
    mnemonic: Mnemonic,
    Instruction { opcode: _, a, b, c }: Instruction,
    reg: &mut Register,
) {
    match mnemonic {
        AddR => reg[c] = reg[a] + reg[b],
        AddI => reg[c] = reg[a] + b,
        MulR => reg[c] = reg[a] * reg[b],
        MulI => reg[c] = reg[a] * b,
        BanR => reg[c] = reg[a] & reg[b],
        BanI => reg[c] = reg[a] & b,
        BorR => reg[c] = reg[a] | reg[b],
        BorI => reg[c] = reg[a] | b,
        SetR => reg[c] = reg[a],
        SetI => reg[c] = a,
        GtIR => reg[c] = if a > reg[b] { 1 } else { 0 },
        GtRI => reg[c] = if reg[a] > b { 1 } else { 0 },
        GtRR => reg[c] = if reg[a] > reg[b] { 1 } else { 0 },
        EqIR => reg[c] = if a == reg[b] { 1 } else { 0 },
        EqRI => reg[c] = if reg[a] == b { 1 } else { 0 },
        EqRR => reg[c] = if reg[a] == reg[b] { 1 } else { 0 },
    }
}

fn possible_mnemonics(sample: Sample) -> Vec<Mnemonic> {
    let mut possible_mnemonics = Vec::default();
    for &mnemonic in INSTRUCTION_SET {
        let mut register = sample.before;
        execute_mnemonic(mnemonic, sample.instruction, &mut register);
        if register == sample.after {
            possible_mnemonics.push(mnemonic);
        }
    }
    possible_mnemonics
}

fn decode_opcodes(samples: &[Sample]) -> HashMap<OpCode, Mnemonic> {
    let mut opcodes: HashMap<OpCode, Mnemonic> = HashMap::with_capacity(16);
    let mut mnemonics: HashMap<OpCode, HashSet<Mnemonic>> = HashMap::with_capacity(16);
    for &sample in samples {
        mnemonics
            .entry(sample.instruction.opcode)
            .or_insert_with(HashSet::default)
            .extend(possible_mnemonics(sample));
    }
    while !mnemonics.is_empty() {
        debug!("{:?}\n", mnemonics);
        debug!("opcodes: {:?}\n\n", opcodes);
        let clarified_opcodes: Vec<(OpCode, Mnemonic)> = mnemonics
            .iter()
            .filter_map(|(&opc, mnes)| {
                if mnes.len() == 1 {
                    Some((opc, *mnes.iter().next().unwrap()))
                } else {
                    None
                }
            })
            .collect();
        for (opc, mnem) in clarified_opcodes {
            mnemonics.values_mut().for_each(|mnes| {
                mnes.remove(&mnem);
            });
            opcodes.insert(opc, mnem);
        }
        mnemonics.retain(|_, mnes| !mnes.is_empty());
    }
    opcodes
}

fn parse_register(s: &str) -> Result<Register, String> {
    let mut values = [0; 4];
    let mut c_idx = 0;
    let mut c_val = String::with_capacity(1);
    for chr in s.chars() {
        match chr {
            '[' => {},
            ']' => {
                let value = c_val.parse::<Data>().map_err(|e| e.to_string())?;
                values[c_idx] = value;
                c_idx += 1;
                c_val.clear();
                break;
            },
            ',' => {
                let value = c_val.parse::<Data>().map_err(|e| e.to_string())?;
                values[c_idx] = value;
                c_idx += 1;
                c_val.clear();
            },
            ' ' => {},
            _ if chr.is_digit(10) => c_val.push(chr),
            _ => return Err(format!("unexpected character {}", chr)),
        }
    }
    if c_idx != 4 {
        Err(format!("expected 4 values but got {}", values.len()))
    } else {
        Ok(Register::from(values))
    }
}

fn parse_instruction(line: &str) -> Result<Instruction, String> {
    let mut parts = line.split(' ');
    let opcode = parse_opcode(&mut parts, line)?;
    let data1 = parse_data(&mut parts, line)?;
    let data2 = parse_data(&mut parts, line)?;
    let data3 = parse_data(&mut parts, line)?;
    Ok(Instruction::from((opcode, data1, data2, data3)))
}

fn parse_opcode<'a>(parts: &mut impl Iterator<Item = &'a str>, line: &str) -> Result<u8, String> {
    parts
        .next()
        .ok_or_else(|| format!("unexpected end of line in {:?}", line))
        .and_then(|part| part.trim().parse::<u8>().map_err(|e| e.to_string()))
}

fn parse_data<'a>(parts: &mut impl Iterator<Item = &'a str>, line: &str) -> Result<Data, String> {
    parts
        .next()
        .ok_or_else(|| format!("unexpected end of line in {:?}", line))
        .and_then(|part| part.trim().parse::<Data>().map_err(|e| e.to_string()))
}

#[aoc_generator(day16)]
pub fn parse(input: &str) -> Result<(Vec<Sample>, Vec<Instruction>), String> {
    let mut samples = Vec::with_capacity(16);
    let mut instructions = Vec::with_capacity(16);
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.starts_with("Before:") {
            let before = parse_register(&line[8..])?;
            let line = lines
                .next()
                .ok_or_else(|| "sample instruction expected but no more input left".to_string())?;
            let instruction = parse_instruction(line)?;
            let line = lines.next().ok_or_else(|| {
                "register after instruction expected but no more input left".to_string()
            })?;
            let after = parse_register(&line[8..])?;
            samples.push(Sample::new(before, instruction, after));
        } else if !line.trim().is_empty() {
            let instruction = parse_instruction(line)?;
            instructions.push(instruction);
        }
    }
    Ok((samples, instructions))
}

#[aoc(day16, part1)]
pub fn num_samples_behaving_like_three_or_more_opcodes(
    (samples, _): &(Vec<Sample>, Vec<Instruction>),
) -> usize {
    samples
        .iter()
        .cloned()
        .map(possible_mnemonics)
        .filter(|ops| ops.len() >= 3)
        .count()
}

#[aoc(day16, part2)]
pub fn run_program((samples, program): &(Vec<Sample>, Vec<Instruction>)) -> Register {
    let opcodes = decode_opcodes(samples);
    let mut register = Register::default();
    let interpreter = Interpreter::with_instruction_set(opcodes);
    interpreter
        .run(program, &mut register)
        .unwrap_or_else(|err| println!("error executing program: {}", err));
    register
}

#[cfg(test)]
mod tests;
