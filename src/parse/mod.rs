use mlem::{Address, Instruction, Register, Program};
use super::lex;
mod address;
use self::address::parse_address;
mod instruction;
use self::instruction::{InstructionName, parse_instruction};
#[cfg(test)]
mod test;

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