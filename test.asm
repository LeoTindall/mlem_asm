; a simple program that loops 1000 times
move L:1001 R:R7    ; 0
add R:R0 L:1        ; 1
output R:R0         ; 2
sub R:R7 L:1        ; 3
jnz L:1 R:R7        ; 4 Loop if the counter is not 0
