#![allow(dead_code)]

pub enum InstructionName {
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

pub fn parse_instruction(name: &str) -> Result<InstructionName, String> {
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
