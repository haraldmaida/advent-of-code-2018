//! # Day 19: Go With The Flow
//!
//! With the Elves well on their way constructing the North Pole base, you turn
//! your attention back to understanding the inner workings of programming the
//! device.
//!
//! You can't help but notice that the device's opcodes don't contain any flow
//! control like jump instructions. The device's manual goes on to explain:
//!
//! "In programs where flow control is required, the instruction pointer can be
//! bound to a register so that it can be manipulated directly. This way,
//! setr/seti can function as absolute jumps, addr/addi can function as relative
//! jumps, and other opcodes can cause truly fascinating effects."
//!
//! This mechanism is achieved through a declaration like #ip 1, which would
//! modify register 1 so that accesses to it let the program indirectly access
//! the instruction pointer itself. To compensate for this kind of binding,
//! there are now six registers (numbered 0 through 5); the five not bound to
//! the instruction pointer behave as normal. Otherwise, the same rules apply as
//! the last time you worked with this device.
//!
//! When the instruction pointer is bound to a register, its value is written to
//! that register just before each instruction is executed, and the value of
//! that register is written back to the instruction pointer immediately after
//! each instruction finishes execution. Afterward, move to the next instruction
//! by adding one to the instruction pointer, even if the value in the
//! instruction pointer was just updated by an instruction. (Because of this,
//! instructions must effectively set the instruction pointer to the instruction
//! before the one they want executed next.)
//!
//! The instruction pointer is 0 during the first instruction, 1 during the
//! second, and so on. If the instruction pointer ever causes the device to
//! attempt to load an instruction outside the instructions defined in the
//! program, the program instead immediately halts. The instruction pointer
//! starts at 0.
//!
//! It turns out that this new information is already proving useful: the CPU in
//! the device is not very powerful, and a background process is occupying most
//! of its time. You dump the background process' declarations and instructions
//! to a file (your puzzle input), making sure to use the names of the opcodes
//! rather than the numbers.
//!
//! For example, suppose you have the following program:
//!
//! ```text
//! #ip 0
//! seti 5 0 1
//! seti 6 0 2
//! addi 0 1 0
//! addr 1 2 3
//! setr 1 0 0
//! seti 8 0 4
//! seti 9 0 5
//! ```
//!
//! When executed, the following instructions are executed. Each line contains
//! the value of the instruction pointer at the time the instruction started,
//! the values of the six registers before executing the instructions (in square
//! brackets), the instruction itself, and the values of the six registers after
//! executing the instruction (also in square brackets).
//!
//! ```text
//! ip=0 [0, 0, 0, 0, 0, 0] seti 5 0 1 [0, 5, 0, 0, 0, 0]
//! ip=1 [1, 5, 0, 0, 0, 0] seti 6 0 2 [1, 5, 6, 0, 0, 0]
//! ip=2 [2, 5, 6, 0, 0, 0] addi 0 1 0 [3, 5, 6, 0, 0, 0]
//! ip=4 [4, 5, 6, 0, 0, 0] setr 1 0 0 [5, 5, 6, 0, 0, 0]
//! ip=6 [6, 5, 6, 0, 0, 0] seti 9 0 5 [6, 5, 6, 0, 0, 9]
//! ```
//!
//! In detail, when running this program, the following events occur:
//!
//! * The first line (#ip 0) indicates that the instruction pointer should be
//!   bound to register 0 in this program. This is not an instruction, and so
//!   the value of the instruction pointer does not change during the processing
//!   of this line.
//! * The instruction pointer contains 0, and so the first instruction is
//!   executed (seti 5 0 1). It updates register 0 to the current instruction
//!   pointer value (0), sets register 1 to 5, sets the instruction pointer to
//!   the value of register 0 (which has no effect, as the instruction did not
//!   modify register 0), and then adds one to the instruction pointer.
//! * The instruction pointer contains 1, and so the second instruction, seti 6
//!   0 2, is executed. This is very similar to the instruction before it: 6 is
//!   stored in register 2, and the instruction pointer is left with the value
//!   2.
//! * The instruction pointer is 2, which points at the instruction addi 0 1 0.
//!   This is like a relative jump: the value of the instruction pointer, 2, is
//!   loaded into register 0. Then, addi finds the result of adding the value in
//!   register 0 and the value 1, storing the result, 3, back in register 0.
//!   Register 0 is then copied back to the instruction pointer, which will
//!   cause it to end up 1 larger than it would have otherwise and skip the next
//!   instruction (addr 1 2 3) entirely. Finally, 1 is added to the instruction
//!   pointer.
//! * The instruction pointer is 4, so the instruction setr 1 0 0 is run. This
//!   is like an absolute jump: it copies the value contained in register 1, 5,
//!   into register 0, which causes it to end up in the instruction pointer. The
//!   instruction pointer is then incremented, leaving it at 6.
//! * The instruction pointer is 6, so the instruction seti 9 0 5 stores 9 into
//!   register 5. The instruction pointer is incremented, causing it to point
//!   outside the program, and so the program ends.
//!
//! What value is left in register 0 when the background process halts?
//!
//! ## Part 2
//!
//! A new background process immediately spins up in its place. It appears
//! identical, but on closer inspection, you notice that this time, register 0
//! started with the value 1.
//!
//! What value is left in register 0 when this new background process halts?
//!
//! [Advent of Code 2018 - Day 19](https://adventofcode.com/2018/day/19)

use std::{
    fmt::{self, Display},
    iter::FromIterator,
    ops::{Index, IndexMut},
};

use crate::day16::{Data, Mnemonic};

use self::Mnemonic::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Register([Data; 6]);

impl Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}, {}, {}, {}, {}, {}]",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

impl Default for Register {
    fn default() -> Self {
        Register([0; 6])
    }
}

impl From<[Data; 6]> for Register {
    fn from(value: [Data; 6]) -> Self {
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
    pub opcode: Mnemonic,
    pub a: Data,
    pub b: Data,
    pub c: Data,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.opcode, self.a, self.b, self.c)
    }
}

impl From<(Mnemonic, Data, Data, Data)> for Instruction {
    fn from((opcode, a, b, c): (Mnemonic, Data, Data, Data)) -> Self {
        Self { opcode, a, b, c }
    }
}

impl Instruction {
    pub fn new(opcode: Mnemonic, a: Data, b: Data, c: Data) -> Self {
        Self { opcode, a, b, c }
    }
}

pub type Addr = Data;

#[derive(Debug, Clone, PartialEq)]
pub struct Interpreter {
    ip_reg: Addr,
    ip: Addr,
}

impl Interpreter {
    pub fn new(ip_reg: Addr) -> Self {
        Self { ip_reg, ip: 0 }
    }

    #[inline]
    pub fn execute(&mut self, instruction: Instruction, register: &mut Register) {
        register[self.ip_reg] = self.ip;
        execute_mnemonic(instruction, register);
        self.ip = register[self.ip_reg] + 1;
    }

    pub fn run(&mut self, program: &[Instruction], register: &mut Register) -> Result<(), String> {
        while let Some(&instruction) = program.get(self.ip as usize) {
            if self.ip == 3 {
                self.ip = optimized(register);
                continue;
            }
            let _c_ip = self.ip;
            self.execute(instruction, register);
            //_trace(_c_ip, instruction, register);
        }
        Ok(())
    }
}

fn execute_mnemonic(Instruction { opcode, a, b, c }: Instruction, reg: &mut Register) {
    match opcode {
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

#[inline]
fn _trace(ip: Addr, Instruction { opcode, a, b, c }: Instruction, reg: &Register) {
    match opcode {
        AddR => println!("{:02}: {} {} {} {} : {} ", ip, opcode, a, b, c, reg),
        AddI => println!("{:02}: {} {} {} {} : {} ", ip, opcode, a, b, c, reg),
        MulR => println!("{:02}: {} {} {} {} : {} ", ip, opcode, a, b, c, reg),
        MulI => println!("{:02}: {} {} {} {} : {} ", ip, opcode, a, b, c, reg),
        BanR => println!("{:02}: {} {} {} {} : {} ", ip, opcode, a, b, c, reg),
        BanI => println!("{:02}: {} {} {} {} : {} ", ip, opcode, a, b, c, reg),
        BorR => println!("{:02}: {} {} {} {} : {} ", ip, opcode, a, b, c, reg),
        BorI => println!("{:02}: {} {} {} {} : {} ", ip, opcode, a, b, c, reg),
        SetR => println!("{:02}: {} {} {} {} : {} ", ip, opcode, a, b, c, reg),
        SetI => println!("{:02}: {} {} {} {} : {} ", ip, opcode, a, b, c, reg),
        GtIR => println!("{:02}: {} {} {} {} : {} ", ip, opcode, a, b, c, reg),
        GtRI => println!("{:02}: {} {} {} {} : {} ", ip, opcode, a, b, c, reg),
        GtRR => println!("{:02}: {} {} {} {} : {} ", ip, opcode, a, b, c, reg),
        EqIR => println!("{:02}: {} {} {} {} : {} ", ip, opcode, a, b, c, reg),
        EqRI => println!("{:02}: {} {} {} {} : {} ", ip, opcode, a, b, c, reg),
        EqRR => println!("{:02}: {} {} {} {} : {} ", ip, opcode, a, b, c, reg),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    ip_reg: Addr,
    instructions: Vec<Instruction>,
}

impl Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "#ip {}", self.ip_reg)?;
        for instruction in &self.instructions {
            writeln!(f, "{}", instruction)?;
        }
        Ok(())
    }
}

impl Program {
    pub fn new(ip_reg: Addr, instructions: impl IntoIterator<Item = Instruction>) -> Self {
        Self {
            ip_reg,
            instructions: Vec::from_iter(instructions.into_iter()),
        }
    }

    pub fn ip_reg(&self) -> Addr {
        self.ip_reg
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }
}

#[aoc_generator(day19)]
pub fn parse(input: &str) -> Result<Program, String> {
    let mut ip_reg = 6;
    let mut instructions = Vec::with_capacity(16);
    for line in input.lines() {
        if line.starts_with("#ip") {
            ip_reg = line[4..]
                .trim()
                .parse::<Addr>()
                .map_err(|e| e.to_string())?;
        } else {
            let opc = line[0..4].parse()?;
            let mut oprs = line[5..]
                .split(' ')
                .take(3)
                .map(|s| s.trim().parse::<Data>().map_err(|e| e.to_string()));
            let opr1 = oprs.next().unwrap()?;
            let opr2 = oprs.next().unwrap()?;
            let opr3 = oprs.next().unwrap()?;
            instructions.push(Instruction::new(opc, opr1, opr2, opr3));
        }
    }
    Ok(Program::new(ip_reg, instructions))
}

#[aoc(day19, part1)]
pub fn run_background_process(program: &Program) -> Data {
    let mut interpreter = Interpreter::new(program.ip_reg);
    let mut register = Register::default();
    interpreter
        .run(program.instructions(), &mut register)
        .unwrap();
    register[0]
}

#[aoc(day19, part2)]
pub fn run_background_process_2(program: &Program) -> Data {
    let mut interpreter = Interpreter::new(program.ip_reg);
    let mut register = Register::default();
    register[0] = 1;
    interpreter
        .run(program.instructions(), &mut register)
        .unwrap();
    register[0]
}

/// Repeated loop:
///
/// ```text
/// 'L1:  R1 = R5 * R2
///       if R4 == R1 then
///         R1 = 1
///         R0 = R5 + R0
///       else
///         R1 = 0
///       end if
///       R3 = R1 + R3
///       R2 = R2 + 1
///       if R2 > R4 then
///         R1 = 1
///         R3 = R3 + 1  // goto 'L2
///       else
///         R1 = 0
///         R3 = R3 + R1
///         R3 = 2  // goto 'L1
///       end if
/// 'L2:
/// ```
fn optimized(reg: &mut Register) -> Addr {
    if reg[4] % reg[5] == 0 {
        reg[0] = reg[5] + reg[0];
    }
    reg[2] = reg[4];
    reg[1] = 0;
    12
}

#[cfg(test)]
mod tests;
