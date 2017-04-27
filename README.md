# mlem-asm

[![Crates.io version badge](https://img.shields.io/crates/v/mlem-asm.svg)](https://crates.io/crates/mlem)
[![Docs.rs version badge](https://docs.rs/mlem-asm/badge.svg)](https://docs.rs/mlem/)

This crate provides an assembler for the [MLeM](https://github.com/SilverWingedSeraph/mlem) virtual machine.
It assembles codes like this:

```
; a simple program that loops 1000 times
move L:1001 R:R7    ; 0
add R:R0 L:1        ; 1
output R:R0         ; 2, output current iteration
sub R:R7 L:1        ; 3
jnz L:1 R:R7        ; 4 Loop if the counter is not 0
```