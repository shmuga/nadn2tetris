(LOOP)
@24576
D=M
@WHITE
D;JEQ

(BLACK)
@R0
M=-1
@PAINT
0;JMP

(WHITE)
@R0
M=0

(PAINT)
// initial values for paint
@8191
D=A

@SCREEN
D=A+D

@counter
M=D

// paint 1 word from the end while not reached start of canvas
(PAINT1)
@R0
D=M

@counter
A=M
M=D

// decrement  counter and check if begining reached
@counter
MD=M-1

@SCREEN
D=D-A

@PAINT1
D;JGE
// --- (PAINT1)

@LOOP
0;JMP
