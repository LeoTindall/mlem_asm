#![allow(dead_code)]
use mlem::{Register, Address};

pub fn parse_register_name(item: &str) -> Result<Register, String> { 
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

// Parse an address; register names, literals, or pointers to either of those two (with * prefix).
// Literals can be decimal, hexidecimal (with 0x___), or binary (with 0b___).
pub fn parse_address(item: &str) -> Result<Address, String> {
    
    if item.chars().next().unwrap() == '*' {  // Try pointers first; if it has a * it's a pointer
        if let Ok(r) = parse_register_name(&item[1..]) {
            return Ok(Address::MemReg(r));
        }
        if let Ok(l) = parse_literal(&item[1..]) {
            return Ok(Address::MemAbs(l));
        }
    } else { // No * means it's a literal
        if let Ok(r) = parse_register_name(&item) {
            return Ok(Address::RegAbs(r));
        }
        if let Ok(l) = parse_literal(&item) {
            return Ok(Address::Literal(l));
        }

    }
    Err(format!("Unknown address type: \"{}\" is not a register name, literal, or *pointer", &item))
}

fn parse_literal(item: &str) -> Result<u64, String> {
    // Check if there is a radix specifier
    let radix: u32;
    let sanitized_item: &str;
    if item.len() < 2 {
        radix = 10;
        sanitized_item = item;
    } else {
        let r: (u32, &str) = match &item[0..2] {
            "0x" => (16, &item[2..]),
            "0b" => (2, &item[2..]),
            _ => (10, &item),
        };
        radix = r.0;
        sanitized_item = r.1;
    }

    //Parse the literal value
    match u64::from_str_radix(sanitized_item, radix) {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("Could not parse literal of base {} : {}", radix, e))
    }
 }
