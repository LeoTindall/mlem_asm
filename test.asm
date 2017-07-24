; a simple program that prints some ASCII characters
; from space to tilde
move    96  R7      ; 0 Set the counter
move    31  R0      ; 1 Set the initial value to output
add     R0  1       ; 2 Increment the value to output
output  R0          ; 3 Output that value
sub     R7  1       ; 4 Update the counter
jnz     2   R7      ; 5 Loop if the counter is not 0
halt                ; 6 Allow the program to complete successfully
