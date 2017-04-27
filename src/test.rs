use {program_to_writer};

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
    let prog = super::parse::parse_program(valid_program).unwrap();
    program_to_writer(&prog, &mut buffer).unwrap();
    assert!(buffer.get_ref() == &expected_cbor, "Program resulted in: {:?} not: {:?}", buffer.get_ref(), expected_cbor);
}