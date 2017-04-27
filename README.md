# mlem-asm

[![Crates.io version badge](https://img.shields.io/crates/v/mlem-asm.svg)](https://crates.io/crates/mlem-asm)
[![Docs.rs version badge](https://docs.rs/mlem-asm/badge.svg)](https://docs.rs/mlem-asm/)

This crate provides an assembler for the [MLeM](https://github.com/SilverWingedSeraph/mlem) virtual machine.
It assembles codes like this:

```
; a simple program that prints some ASCII characters
; from space to tilde
move L:96  R:R7     ; 0 Set the counter
move L:31  R:R0     ; 1 Set the initial value to output
add R:R0 L:1        ; 2 Increment the value to output
output R:R0         ; 3 Output that value
sub R:R7 L:1        ; 4 Update the counter
jnz L:2 R:R7        ; 5 Loop if the counter is not 0
halt                ; 6 Allow the program to complete successfully
```

This is, in fact, the contents of `test.asm`. The provided front-end in the examples directory can be used to 
assemble and run this program, thus:

```
14:38:41: leo [~/Projects/mlem-asm]
$ cargo run --example mlem-asm r ~/Projects/mlem-asm/test.asm
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/examples/mlem-asm r /home/leo/Projects/mlem-asm/test.asm`
 !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~
Halt
```