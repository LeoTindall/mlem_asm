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


/// Simply parse a program, each line resulting in either a valid or invalid line (Ok or Err).
/// This function can't fail; however, there's no guarantee that even one valid instruction is produced.
fn initial_parse_program(program: &str) -> Vec<Result<Option<Instruction>, String>> {
    let lines = program.lines();
    let mut v = Vec::new();
    for line in lines {
        match parse::parse_line(line) {
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