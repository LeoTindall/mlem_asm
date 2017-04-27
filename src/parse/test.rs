use super::parse_line;
use super::{Instruction, Address, Register};
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
fn test_radix_literals() {
    assert_eq!(parse_line("move L:0xff L:0b11111111").unwrap(), Some(Instruction::Move(Address::Literal(255), Address::Literal(255))));
}

#[test]
fn test_double_argument() {
    assert!(parse_line("move R:R0 R:R1").unwrap() == Some(Instruction::Move(
        Address::RegAbs(Register::R0),
        Address::RegAbs(Register::R1),
        )), "It was {:?}", parse_line("move R:R0 R:R1"));
}