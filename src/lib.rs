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

extern crate serde_cbor;

extern crate mlem;
pub use mlem::{Address, Instruction, Register, Program};

#[cfg(test)]
mod test;

mod parse;
mod lex;

use std::io::Write;
/// Writes an assembled program to a writer in packed, self-describing CBOR (a format MLeM can natively consume.)
/// 
/// Writing to, i.e., a file allows you to save assembled "binaries" that MLeM can execute; you can also pass 
/// data over the network for distributed processing.
pub fn program_to_writer(p: &Program, mut w: &mut Write) -> Result<(), serde_cbor::Error> {
    use serde_cbor::ser::to_writer_packed_sd;
    to_writer_packed_sd(&mut w, &p)
}