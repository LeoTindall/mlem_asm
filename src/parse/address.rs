#![allow(dead_code)]
use mlem::{Register, Address};
use std::borrow::Borrow;

/// Parse a register name into a Register or a reason why it could not.
/// All register names begin with an R (case insensitive).
pub fn parse_register_name(item: &str) -> Result<Register, String> { 
    match item.to_lowercase().borrow() {
        "r0" => Ok(Register::R0),
        "r1" => Ok(Register::R1),
        "r2" => Ok(Register::R2),
        "r3" => Ok(Register::R3),
        "r4" => Ok(Register::R4),
        "r5" => Ok(Register::R5),
        "r6" => Ok(Register::R6),
        "r7" => Ok(Register::R7),
        "rsp" => Ok(Register::SP),
        "rbp" => Ok(Register::BP),
        other => Err(format!("Unknown register name: {}", other))
    }
 }

// Parse an address of the form "specifier:item" where specifier is one of "R", "L", "M", or "P"
// and item is an appropriate specifier.
// Literals can be decimal, hexidecimal (with 0x___), or binary (with 0b___).
pub fn parse_address(name: &str) -> Result<Address, String> {
    let first_character: char = match name.to_lowercase().chars().next() {
        Some(v) => v,
        None => return Err("Cannot parse empty address.".into())
    };

    if first_character == 'r' {
        match parse_register_name(&name) {
            Ok(r) => Ok(Address::RegAbs(r)),
            Err(e) => Err(e)
        }
    } else if first_character.is_digit(10) {
        // Either this starts with a radix specifier (a 0 and a letter) or a number.
        match parse_literal(&name) {
            Ok(l) => Ok(Address::Literal(l)),
            Err(e) => Err(e)
        }
    } else if first_character == '*' {
        match parse_literal(&name[1..]) {
            Ok(l) => Ok(Address::MemAbs(l)),
            Err(literal_parse_error) => { 
                match parse_register_name(&name[1..]) {
                    Ok(r) => Ok(Address::MemReg(r)),
                    Err(register_parse_error) => Err(
                        format!("Expected a register or memory address, failed to parse either. {}, {}", literal_parse_error, register_parse_error) 
                    )
                }
            }
        }
    } else {
        Err(format!("Unknown address type specifier: {} (expected r, *, or digit).", first_character))
    }
}

fn parse_literal(item: &str) -> Result<u64, String> {
    // Check if there is a radix specifier
    let non_decimal_radix: Option<u32>;
    if item.len() < 2 {
        non_decimal_radix = None;
    } else {
        non_decimal_radix = match &item[0..2] {
            "0x" => Some(16),
            "0b" => Some(2),
            _ => None,
        };
    }

    //Parse the literal value
    match non_decimal_radix {
        Some(radix) => {
            match u64::from_str_radix(&item[2..item.len()], radix) {
                Ok(v) => Ok(v),
                Err(e) => Err(format!("Could not parse literal of base : {}", e))
            }
        }
        None => {
            match u64::from_str_radix(item, 10) {
                Ok(v) => Ok(v),
                Err(e) => Err(format!("Could not parse literal: {}", e))
            }
        }
    }
 }
