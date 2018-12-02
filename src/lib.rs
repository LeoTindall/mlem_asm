//! mlem-asm is an assembler library for MLeM, the Machine Learning Machine.
//! This is a Harvard architecture machine with immutable programs, 8 general purpouse registers,
//! and a hardware-supported stack with stack and base pointers.
//! 
//! # Assembly Language
//! mlem-asm assembles the mlasm language into CBOR-encoded data that can be read and executed by MLeM.
//! The mlasm language looks like so:
//!
//! ```mlasm,ignore
//! ; Anything following a semicolon is a comment.
//! ; Lines can be terminated with a semicolon, or not. The following two lines are equivalent:
//! noop
//! noop;
//! ; Instructions that require arguments look like so:
//! move r0 r1 ; Set R1 equal to R0
//! input *r0 ; read input into memory pointed at by r0
//! output *0b01101 ; write output from memory pointed at by 0b01101
//! ```
//! 
//! # Examples
//! Here is an example of parsing a program and converting it into CBOR.
//!
//! ```
//! use mlem_asm::*;
//! use std::io::Cursor;
//! let valid_program = "
//!    noop
//!    move r0 rsp; this is a comment
//!    input r0; input into r0
//!    ; comment only
//!
//!    ";
//!
//!    let mut buffer = Cursor::new(Vec::<u8>::new());
//!
//!    let program = parse_program(valid_program).unwrap();
//!    program_to_writer(&program, &mut buffer).unwrap();
//!    assert_eq!(buffer.get_ref(), 
//!               &[217, 217, 247, 131, 0, 131, 2, 130, 
//!                 0, 0, 130, 0, 8, 130, 4, 130, 0, 0])
//!   
//! ```

extern crate serde_cbor;

extern crate mlem;
pub use mlem::{Address, Instruction, Register, Program};

#[cfg(test)]
mod test;

pub mod parse;
pub use parse::{parse_program};
pub mod lex;

use std::io::Write;
/// Writes an assembled program to a writer in packed, self-describing CBOR (a format MLeM can natively consume.)
/// 
/// Writing to, i.e., a file allows you to save assembled "binaries" that MLeM can execute; you can also pass 
/// data over the network for distributed processing.
pub fn program_to_writer(p: &Program, mut w: &mut Write) -> Result<(), serde_cbor::Error> {
    use serde_cbor::ser::to_writer_packed_sd;
    to_writer_packed_sd(&mut w, &p)
}
