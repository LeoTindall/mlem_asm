use {parse_program, program_to_writer};
use mlem::{Instruction, Register, Address};

#[test]
fn test_parse_invalid_program() {
    let invalid_program = "
    ; comment only
    noop;
    move R:R0 R:R1;
    move R:Rx R:R1;
    move R:R0 L:1024p;
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

#[test]
fn test_program_to_writer() {
    use std::io::Cursor;
    let valid_program = "
    noop
    move R:R0 R:SP;
    input R:R0;
    ; comment only

    ";
    let mut buffer = Cursor::new(Vec::<u8>::new());
    let expected_cbor: Vec<u8> = vec![217, 217, 247, 131, 0, 131, 2, 130, 0, 0, 130, 0, 8, 130, 4, 130, 0, 0];
    let prog = parse_program(valid_program).unwrap();
    program_to_writer(&prog, &mut buffer).unwrap();
    assert!(buffer.get_ref() == &expected_cbor, "Program resulted in: {:?} not: {:?}", buffer.get_ref(), expected_cbor);
}