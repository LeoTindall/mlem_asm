use parse_line;
use mlem::{Instruction, Register, Address};
#[test]
fn test() {
    assert!(parse_line("") == None);
    assert!(parse_line("noop") == Some(Instruction::NoOp));
    assert!(parse_line("noop; some comments") == Some(Instruction::NoOp));
}

#[test]
fn test_single_argument() {
    assert!(parse_line("iput R:R0") == Some(Instruction::Input(Address::RegAbs(Register::R0))));
    assert!(parse_line("oput R:R0 R:R1") == None);
}

#[test]
fn test_double_argument() {
    assert!(parse_line("move R:R0 R:R1") == Some(Instruction::Move(
        Address::RegAbs(Register::R0),
        Address::RegAbs(Register::R1),
        )), "It was {:?}", parse_line("move R:R0 R:R1"));
}