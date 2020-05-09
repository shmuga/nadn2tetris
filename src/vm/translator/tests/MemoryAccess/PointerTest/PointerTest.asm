// push segment constant to location 3030
@3030
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop segment pointer to location 0
@SP
M=M-1
@SP
A=M
D=M
@THIS
M=D
// push segment constant to location 3040
@3040
D=A
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
// push segment constant to location 32
@32
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop segment this to location 2
@2
D=A
@THIS
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
// push segment constant to location 46
@46
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop segment that to location 6
@6
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
// push segment pointer to location 0
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
// push segment pointer to location 1
@THAT
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
// push segment this to location 2
@2
D=A
@THIS
A=M+D
D=M
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
// push segment that to location 6
@6
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
