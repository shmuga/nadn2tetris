// push segment argument to location 1
@1
D=A
@ARG
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1
// pop segment pointer to location 1
@SP
M=M-1
@SP
A=M
D=M
@THAT
M=D
// push segment constant to location 0
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop segment that to location 0
@0
D=A
@THAT
A=M+D
D=A
@TEMP
M=D
@SP
M=M-1
@SP
A=M
D=M
@TEMP
A=M
M=D
// push segment constant to location 1
@1
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop segment that to location 1
@1
D=A
@THAT
A=M+D
D=A
@TEMP
M=D
@SP
M=M-1
@SP
A=M
D=M
@TEMP
A=M
M=D
// push segment argument to location 0
@0
D=A
@ARG
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1
// push segment constant to location 2
@2
D=A
@SP
A=M
M=D
@SP
M=M+1
// -
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M-D
@SP
M=M+1
// pop segment argument to location 0
@0
D=A
@ARG
A=M+D
D=A
@TEMP
M=D
@SP
M=M-1
@SP
A=M
D=M
@TEMP
A=M
M=D
(FibonacciSeries.vm.MAIN_LOOP_START)
// push segment argument to location 0
@0
D=A
@ARG
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
@SP
A=M
D=M
@FibonacciSeries.vm.COMPUTE_ELEMENT
D;JGT
@FibonacciSeries.vm.END_PROGRAM
1;JMP
(FibonacciSeries.vm.COMPUTE_ELEMENT)
// push segment that to location 0
@0
D=A
@THAT
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1
// push segment that to location 1
@1
D=A
@THAT
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1
// +
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M+D
@SP
M=M+1
// pop segment that to location 2
@2
D=A
@THAT
A=M+D
D=A
@TEMP
M=D
@SP
M=M-1
@SP
A=M
D=M
@TEMP
A=M
M=D
// push segment pointer to location 1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
// push segment constant to location 1
@1
D=A
@SP
A=M
M=D
@SP
M=M+1
// +
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M+D
@SP
M=M+1
// pop segment pointer to location 1
@SP
M=M-1
@SP
A=M
D=M
@THAT
M=D
// push segment argument to location 0
@0
D=A
@ARG
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1
// push segment constant to location 1
@1
D=A
@SP
A=M
M=D
@SP
M=M+1
// -
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M-D
@SP
M=M+1
// pop segment argument to location 0
@0
D=A
@ARG
A=M+D
D=A
@TEMP
M=D
@SP
M=M-1
@SP
A=M
D=M
@TEMP
A=M
M=D
@FibonacciSeries.vm.MAIN_LOOP_START
1;JMP
(FibonacciSeries.vm.END_PROGRAM)
