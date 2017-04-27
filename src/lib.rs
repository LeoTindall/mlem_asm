//! mlem-asm is an assembler library for MLeM, the Machine Learning Machine.
//! This is a Harvard architecture machine with immutable programs, 8 general purpouse registers,
//! and a hardware-supported stack with stack and base pointers.
//! 
//! # Assembly Language
//! mlem-asm assembles the mlasm language into CBOR-encoded data that can be read and executed by MLeM.
//! The mlasm language looks like so:
//! ```asm
//! ; Anything following a semicolon is a comment.
//! ; Lines can be terminated with a semicolon, or not. The following two lines are equivalent:
//! noop
//! noop;
//! ; Instructions that require arguments look like so:
//! move R:R0 R:R1 ; Set R1 equal to R0
//! ```

#[macro_use]
extern crate serde_derive;
extern crate serde_cbor;

extern crate mlem;
pub use mlem::{Address, Instruction, Register, Program};

#[cfg(test)]
mod test;

mod parse;
mod lex;

enum InstructionName {
    NoOp,
    Zero,
    Move,
    Output,
    Input,
    Add,
    Sub,
    Halt,
    Illegal,
    Jump,
    JumpIfZero,
    JumpNotZero,
    Push,
    Pop,
    None,
}

fn parse_instruction(name: &str) -> Result<InstructionName, String> {
    use std::borrow::Borrow;
    match name.to_lowercase().borrow() {
        "noop" => Ok(InstructionName::NoOp),
        "halt" => Ok(InstructionName::Halt),
        "illegal" | "illeg" | "illg" => Ok(InstructionName::Illegal),
        "zero" => Ok(InstructionName::Zero),
        "move" => Ok(InstructionName::Move),
        "output" | "oput" => Ok(InstructionName::Output),
        "input" | "iput" => Ok(InstructionName::Input),
        "add" | "uadd" => Ok(InstructionName::Add),
        "subtract" | "usub" | "sub" => Ok(InstructionName::Sub),
        "jump" => Ok(InstructionName::Jump),
        "jumpifzero" | "jumpzero" | "jz" => Ok(InstructionName::JumpIfZero),
        "jumpnotzer" | "jumpnotzero" | "jnz" => Ok(InstructionName::JumpNotZero),
        "push" => Ok(InstructionName::Push),
        "pop" => Ok(InstructionName::Pop),
        other => Err(format!("Unknown instruction: {}", other))
    }
}

fn parse_register_name(item: &str) -> Result<Register, String> { 
    use std::borrow::Borrow;
    match item.to_lowercase().borrow() {
        "r0" => Ok(Register::R0),
        "r1" => Ok(Register::R1),
        "r2" => Ok(Register::R2),
        "r3" => Ok(Register::R3),
        "r4" => Ok(Register::R4),
        "r5" => Ok(Register::R5),
        "r6" => Ok(Register::R6),
        "r7" => Ok(Register::R7),
        "sp" => Ok(Register::SP),
        "bp" => Ok(Register::BP),
        other => Err(format!("Unknown register name: {}", other))
    }
 }

fn parse_literal(item: &str) -> Result<u64, String> { 
    match u64::from_str_radix(item, 10) {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("Could not parse literal: {}", e))
    }
 }

// Parse an address of the form "specifier:item" where specifier is one of "R", "L", "M", or "P"
// and item is an appropriate specifier.
fn parse_address(name: &str) -> Result<Address, String> {
    let pieces: Vec<_> = name.split(':').collect();
    if pieces.len() != 2 { return Err("Malformed address.".into()); }
    use std::borrow::Borrow;
    let specifier = pieces[0];
    let item = pieces[1];
    match specifier.to_lowercase().borrow() {
        "r" => {
            match parse_register_name(item) {
                Ok(r) => Ok(Address::RegAbs(r)),
                Err(e) => Err(e)
            }
        },
        "l" => { 
            match parse_literal(item) {
                Ok(l) => Ok(Address::Literal(l)),
                Err(e) => Err(e)
            }
        },
        "m" => { 
            match parse_literal(item) {
                Ok(l) => Ok(Address::MemAbs(l)),
                Err(e) => Err(e)
            } 
        },
        "p" => { 
            match parse_register_name(item) {
                Ok(r) => Ok(Address::MemReg(r)),
                Err(e) => Err(e)
            }
        }
        other => Err(format!("Unknown address type specifier: {}", other))

    }
}

/// Parse a line of the form `instruction [operand1] [operand2] [operand3][;[comment text]]`
///
/// The return value is a `Result<Option<Instruction>, String>`. An `Ok(Some(_))` value means a valid
/// instruction (for instance, the line `move R:R0 R:R1`). An `Err(_)` value means that there is
/// unparsable about the line (like `move R:R0 R:r1 garbage garbage`); an `Ok(None)` value means that
/// the line was legal but meant nothing (like `; comment only`).
/// # Examples
/// Simple single-line parsing:
/// ```
/// use mlem_asm::{parse_line, Instruction};
/// assert!(parse_line("noop") == parse_line("noop;"));
/// assert!(parse_line("noop") == Ok(Some(Instruction::NoOp)));
/// assert!(parse_line("") == Ok(None));
/// ```
pub fn parse_line(line: &str) -> Result<Option<Instruction>, String> {
    // Split into "words"
    let pieces: Vec<_> = lex::lex_line(line);
    println!("{:?}", pieces);

    // If there are no words, this line is useless.
    if pieces.len() == 0 { return Ok(None); }

    let mut instruction_name = InstructionName::None;
    let mut arg1 = None;
    let mut arg2 = None;
    let mut arg3 = None;
    // Parse the name of the instruction.
    if pieces.len() >= 1 {
        match parse_instruction(pieces[0]) {
            Ok(v) => { instruction_name = v; },
            Err(e) => { return Err(e); }
        };
    }

    // A single piece means a no-arg instruction; it can go straight to an Instruction.
    if pieces.len() == 1 {
        return match instruction_name {
            InstructionName::NoOp => Ok(Some(Instruction::NoOp)),
            InstructionName::Halt => Ok(Some(Instruction::Halt)),
            InstructionName::Illegal => Ok(Some(Instruction::Illegal)),
            _ => Err("Wrong number of arguments. Got 0.".into())
        };
    }

    // More than one word means the args need parsed.
    if pieces.len() >= 2 {
        match parse_address(pieces[1].trim()) {
            Ok(v) => { arg1 = Some(v); },
            Err(e) => { return Err(e); }
        };
    }
    if pieces.len() >= 3 {
        match parse_address(pieces[2].trim()) {
            Ok(v) => { arg2 = Some(v); },
            Err(e) => { return Err(e); }
        };
    }
    if pieces.len() >= 4 {
        match parse_address(pieces[3].trim()) {
            Ok(v) => { arg3 = Some(v); },
            Err(e) => { return Err(e); }
        };
    }

    // Single argument instruction
    if pieces.len() == 2 {
        // Alias arg1 to its inner value, which DEFINITELY exists at this point.
        let arg1 = arg1.unwrap();
        return match instruction_name {
            InstructionName::Zero => Ok(Some(Instruction::Zero(arg1))),
            InstructionName::Input => Ok(Some(Instruction::Input(arg1))),
            InstructionName::Output => Ok(Some(Instruction::Output(arg1))),
            InstructionName::Jump => Ok(Some(Instruction::Jump(arg1))),
            InstructionName::Push => Ok(Some(Instruction::Push(arg1))),
            InstructionName::Pop => Ok(Some(Instruction::Pop(arg1))),
            _ => Err("Wrong number of arguments. Got 1.".into())
        };
    }

    // Two argument instructions
    if pieces.len() == 3 {
        // Alias the arguments known to exist
        let arg1 = arg1.unwrap();
        let arg2 = arg2.unwrap();
        return match instruction_name {
            InstructionName::Move => Ok(Some(Instruction::Move(arg1, arg2))),
            InstructionName::Add => Ok(Some(Instruction::Add(arg1, arg2))),
            InstructionName::Sub => Ok(Some(Instruction::Sub(arg1, arg2))),
            InstructionName::JumpIfZero => Ok(Some(Instruction::JumpIfZero(arg1, arg2))),
            InstructionName::JumpNotZero => Ok(Some(Instruction::JumpNotZero(arg1, arg2))),
            _ => Err("Wrong number of arguments. Got 2.".into())
        }
    }

    Err("Malformed. Perhaps there are too many terms?".into())
}

/// Simply parse a program, each line resulting in either a valid or invalid line (Ok or Err).
/// This function can't fail; however, there's no guarantee that even one valid instruction is produced.
fn initial_parse_program(program: &str) -> Vec<Result<Option<Instruction>, String>> {
    let lines = program.lines();
    let mut v = Vec::new();
    for line in lines {
        match parse_line(line) {
            Ok(i) => { v.push(Ok(i)); }
            Err(e) => { v.push(Err(e)); }
        }
    }
    v
}

/// Parse an entire program, returning either a ready-to-execute MLeM program or
/// a Vec of error messages, with line numbers, of all errors in the program.
/// # Example
/// A valid program:
/// ```
/// use mlem_asm::*;
/// let valid_program = "
///    noop
///    move R:R0 R:SP;
///    input R:R0;
///    ; comment only
///
///    ";
///    let expected_program = Ok(vec![
///            Instruction::NoOp,
///            Instruction::Move(Address::RegAbs(Register::R0), Address::RegAbs(Register::SP)),
///            Instruction::Input(Address::RegAbs(Register::R0))
///    ]);
///    let program = parse_program(valid_program);
///    assert!(program == expected_program, "Program resulted in: {:?} not: {:?}", program, expected_program);
/// ```
///
/// An invalid program:
/// ```
/// use mlem_asm::*;
/// let invalid_program = "
///    noop
///    move R:R0 R:xx;
///    output invalid;
///    ; comment only
///
///    ";
///    let expected_errors = Err(vec![(2, "Unknown register name: xx".into()), (3, "Malformed address.".into())]);
///    let errors = parse_program(invalid_program);
///    assert!(errors == expected_errors, "Program resulted in: {:?} not: {:?}", errors, expected_errors);
/// ```
pub fn parse_program(program: &str) -> Result<Program, Vec<(u64, String)>> {
    let mut p = Vec::new();
    let mut errors = Vec::new();
    for (n, line) in initial_parse_program(program).into_iter().enumerate() {
        match line {
            Ok(v) => {
                if let Some(i) = v { p.push(i) };
            },
            Err(e) => {
                errors.push((n as u64, format!("{}", e)));
            }
        };
    }
    if errors.len() == 0 {
        // No errors!
        Ok(p)
    } else {
        Err(errors)
    }
}

use std::io::Write;
/// Writes an assembled program to a writer in packed, self-describing CBOR (a format MLeM can natively consume.)
/// 
/// Writing to, i.e., a file allows you to save assembled "binaries" that MLeM can execute; you can also pass 
/// data over the network for distributed processing.
pub fn program_to_writer(p: &Program, mut w: &mut Write) -> Result<(), serde_cbor::Error> {
    use serde_cbor::ser::to_writer_packed_sd;
    to_writer_packed_sd(&mut w, &p)
}