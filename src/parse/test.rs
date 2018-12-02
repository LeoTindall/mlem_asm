use super::{parse_line, parse_program};
use super::super::{Instruction, Address, Register};
#[test]
fn test() {
    assert!(parse_line("").unwrap() == None);
    assert!(parse_line("noop").unwrap() == Some(Instruction::NoOp));
    assert!(parse_line("noop; some comments").unwrap() == Some(Instruction::NoOp));
}

#[test]
fn test_single_argument() {
    assert!(parse_line("iput R0").unwrap() == Some(Instruction::Input(Address::RegAbs(Register::R0))));
    parse_line("oput R0 R1").unwrap_err();
}

#[test]
fn test_radix_literals() {
    assert_eq!(parse_line("move 0xff 0b11111111").unwrap(), Some(Instruction::Move(Address::Literal(255), Address::Literal(255))));
}

fn test_memory_absolute() {
    assert_eq!(parse_line("move *0xff R0").unwrap(), Some(Instruction::Move(Address::MemAbs(255), Address::RegAbs(Register::R0))));
}

#[test]
fn test_double_argument() {
    assert!(parse_line("move R0 R1").unwrap() == Some(Instruction::Move(
        Address::RegAbs(Register::R0),
        Address::RegAbs(Register::R1),
        )), "It was {:?}", parse_line("move R0 R1"));
}

#[test]
fn test_parse_invalid_program() {
    let invalid_program = "
    ; comment only
    noop;
    move R0 R1;
    move Rx R1;
    move R0 1024p;
    ";
    let expected_errors = Err(vec![
            (4, "Unknown register name: rx".into()),
            (5, "Could not parse literal: invalid digit found in string".into())
    ]);
    let errors = parse_program(invalid_program);
    assert!(errors == expected_errors, "Program resulted in: {:?} not: {:?}", errors, expected_errors);
}

#[test]
fn test_parse_valid_program() {
    let valid_program = "
    noop
    move R0 RSP;
    input R0;
    ; comment only

    ";
    let expected_program = Ok(vec![
            Instruction::NoOp,
            Instruction::Move(Address::RegAbs(Register::R0), Address::RegAbs(Register::SP)),
            Instruction::Input(Address::RegAbs(Register::R0))
    ]);
    let program = parse_program(valid_program);
    assert!(program == expected_program, "Program resulted in: {:?} not: {:?}", program, expected_program);
}
