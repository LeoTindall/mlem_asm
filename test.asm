; a simple program that prints some ASCII characters
; from space to tilde
move L:96  R:R7     ; 0 Set the counter
move L:31  R:R0     ; 1 Set the initial value to output
add R:R0 L:1        ; 2 Increment the value to output
output R:R0         ; 3 Output that value
sub R:R7 L:1        ; 4 Update the counter
jnz L:2 R:R7        ; 5 Loop if the counter is not 0
halt                ; 6 Allow the program to complete successfully
