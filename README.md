# mlem-asm

[![Crates.io version badge](https://img.shields.io/crates/v/mlem-asm.svg)](https://crates.io/crates/mlem-asm)
[![Docs.rs version badge](https://docs.rs/mlem-asm/badge.svg)](https://docs.rs/mlem-asm/)

This crate provides an assembler for the [MLeM](https://github.com/leotindall/mlem) virtual machine.
It assembles codes like this:

```
; a simple program that prints some ASCII characters
; from space to tilde
move 96  R7     ; 0 Set the counter
move 31  R0     ; 1 Set the initial value to output
add R0 1        ; 2 Increment the value to output
output R0       ; 3 Output that value
sub R7 1        ; 4 Update the counter
jnz 2 R7        ; 5 Loop if the counter is not 0
halt            ; 6 Allow the program to complete successfully
```

This is, in fact, the contents of `test.asm`. The provided front-end can be used to
assemble and run this program, thus:

```
$ cargo run r test.asm
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/examples/mlem-asm r /home/leo/Projects/mlem-asm/test.asm`
 !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~
Halt
```

Using mlem-asm, this program is assembled into the following hex (in test.bin):

```
d9d9 f787 8302 8203 1860 8200 0783 0282 
0318 1f82 0000 8305 8200 0082 0301 8203
8200 0083 0682 0007 8203 0183 0982 0302
8200 070c
```

