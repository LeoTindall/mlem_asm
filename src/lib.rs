#[macro_use]
extern crate serde_derive;
extern crate serde_cbor;

extern crate mlem;
use mlem::{Address, Instruction, Register};

#[cfg(test)]
mod test;

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
    None,
}

fn parse_instruction(name: &str) -> Option<InstructionName> {
    use std::borrow::Borrow;
    match name.to_lowercase().borrow() {
        "noop" => Some(InstructionName::NoOp),
        "halt" => Some(InstructionName::Halt),
        "illegal" | "illeg" | "illg" => Some(InstructionName::Illegal),
        "zero" => Some(InstructionName::Zero),
        "move" => Some(InstructionName::Move),
        "output" | "oput" => Some(InstructionName::Output),
        "input" | "iput" => Some(InstructionName::Input),
        "add" | "uadd" => Some(InstructionName::Add),
        "subtract" | "usub" | "sub" => Some(InstructionName::Sub),
        _ => None
    }
}

fn parse_register_name(item: &str) -> Option<Register> { 
    use std::borrow::Borrow;
    match item.to_lowercase().borrow() {
        "r0" => Some(Register::R0),
        "r1" => Some(Register::R1),
        "r2" => Some(Register::R2),
        "r3" => Some(Register::R3),
        "r4" => Some(Register::R4),
        "r5" => Some(Register::R5),
        "r6" => Some(Register::R6),
        "r7" => Some(Register::R7),
        "sp" => Some(Register::SP),
        "bp" => Some(Register::BP),
        _ => None
    }
 }

fn parse_literal(item: &str) -> Option<u64> { 
    match u64::from_str_radix(item, 10) {
        Ok(v) => Some(v),
        Err(_) => None
    }
 }

// Parse an address of the form "specifier:item" where specifier is one of "R", "L", "M", or "P"
// and item is an appropriate specifier.
fn parse_address(name: &str) -> Option<Address> {
    let pieces: Vec<_> = name.split(':').collect();
    if pieces.len() != 2 { return None; }
    use std::borrow::Borrow;
    let specifier = pieces[0];
    let item = pieces[1];
    match specifier.to_lowercase().borrow() {
        "r" => {
            match parse_register_name(item) {
                Some(r) => Some(Address::RegAbs(r)),
                None => None
            }
        },
        "l" => { 
            match parse_literal(item) {
                Some(l) => Some(Address::Literal(l)),
                None => None
            }
        },
        "m" => { 
            match parse_literal(item) {
                Some(l) => Some(Address::MemAbs(l)),
                None => None
            } 
        },
        "p" => { 
            match parse_register_name(item) {
                Some(r) => Some(Address::MemReg(r)),
                None => None
            }
        }
        _ => None

    }
}

// Parse a line of the form:`
// instruction [operand1] [operand2] [operand3][; [comment text]]
pub fn parse_line(line: &str) -> Option<Instruction> {
    // Split off comments portion
    let semantic_portion;
    match line.split(';').next() {
        Some(v) => { semantic_portion = v; },
        None => { return None; }
    };
    
    // Split into "words"
    let pieces: Vec<_> = semantic_portion.split_whitespace().collect();

    // If there are no words, this line is useless.
    if pieces.len() == 0 { return None; }

    let mut instruction_name = InstructionName::None;
    let mut arg1 = None;
    let mut arg2 = None;
    let mut arg3 = None;
    // Parse the name of the instruction.
    if pieces.len() >= 1 {
        match parse_instruction(pieces[0]) {
            Some(v) => { instruction_name = v; },
            None => { return None; }
        };
    }

    // A single piece means a no-arg instruction; it can go straight to an Instruction.
    if pieces.len() == 1 {
        return match instruction_name {
            InstructionName::NoOp => Some(Instruction::NoOp),
            InstructionName::Halt => Some(Instruction::Halt),
            InstructionName::Illegal => Some(Instruction::Illegal),
            _ => None
        };
    }

    // More than one word means the args need parsed.
    if pieces.len() >= 2 {
        match parse_address(pieces[1]) {
            Some(v) => { arg1 =Some(v); },
            None => { return None; }
        };
    }
    if pieces.len() >= 3 {
        match parse_address(pieces[2]) {
            Some(v) => { arg2 = Some(v); },
            None => { return None; }
        };
    }
    if pieces.len() >= 4 {
        match parse_address(pieces[3]) {
            Some(v) => { arg3 = Some(v); },
            None => { return None; }
        };
    }

    // Single argument instruction
    if pieces.len() == 2 {
        // Alias arg1 to its inner value, which DEFINITELY exists at this point.
        let arg1 = arg1.unwrap();
        return match instruction_name {
            InstructionName::Zero => Some(Instruction::Zero(arg1)),
            InstructionName::Input => Some(Instruction::Input(arg1)),
            InstructionName::Output => Some(Instruction::Output(arg1)),
            _ => None
        };
    }

    // Two argument instructions
    if pieces.len() == 3 {
        // Alias the arguments known to exist
        let arg1 = arg1.unwrap();
        let arg2 = arg2.unwrap();
        return match instruction_name {
            InstructionName::Move => Some(Instruction::Move(arg1, arg2)),
            InstructionName::Add => Some(Instruction::Add(arg1, arg2)),
            InstructionName::Sub => Some(Instruction::Sub(arg1, arg2)),
            _ => None
        }
    }
    
    None
}