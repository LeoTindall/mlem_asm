use {parse_line, parse_program};
use mlem::{Instruction, Register, Address};
#[test]
fn test() {
    assert!(parse_line("").unwrap() == None);
    assert!(parse_line("noop").unwrap() == Some(Instruction::NoOp));
    assert!(parse_line("noop; some comments").unwrap() == Some(Instruction::NoOp));
}

#[test]
fn test_single_argument() {
    assert!(parse_line("iput R:R0").unwrap() == Some(Instruction::Input(Address::RegAbs(Register::R0))));
    parse_line("oput R:R0 R:R1").unwrap_err();
}

#[test]
fn test_double_argument() {
    assert!(parse_line("move R:R0 R:R1").unwrap() == Some(Instruction::Move(
        Address::RegAbs(Register::R0),
        Address::RegAbs(Register::R1),
        )), "It was {:?}", parse_line("move R:R0 R:R1"));
}

#[test]
fn test_parse_invalid_program() {
    let invalid_program = "
    noop;
    move R:R0 R:R1;
    move R:Rx R:R1;
    move R:R0 L:1024p;
    ";
    let expected_errors = Err(vec![
            (3, "Unknown register name: rx".into()),
            (4, "Could not parse literal: invalid digit found in string".into())
    ]);
    let errors = parse_program(invalid_program);
    assert!(errors == expected_errors, "Program resulted in: {:?} not: {:?}", errors, expected_errors);
}

#[test]
fn test_parse_valid_program() {
    let valid_program = "
    noop
    move R:R0 R:SP;
    input R:R0;
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