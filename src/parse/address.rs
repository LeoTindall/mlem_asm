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

// Parse an address of the form "specifier:item" where specifier is one of "R", "L", "M", or "P"
// and item is an appropriate specifier.
// Literals can be decimal, hexidecimal (with 0x___), or binary (with 0b___).
pub fn parse_address(name: &str) -> Result<Address, String> {
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
