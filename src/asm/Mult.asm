// sets a value of r2 to 0
@R2
M=0

// check if r1 = 0 end jump to result
(SUM)
@R1
D=M
@END
D;JEQ

// adds r2 + r0 and stores at r2
@R0
D=M
@R2
M=D+M

// decrements R1 and jumps back to loop
@R1
M=M-1
@SUM
0;JMP

// else jump to end
@END
0;JMP

(END)
@END
0;JMP
