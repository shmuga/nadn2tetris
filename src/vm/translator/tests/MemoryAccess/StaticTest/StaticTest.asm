// push segment constant to location 111
@111
D=A
@SP
A=M
M=D
@SP
M=M+1
// push segment constant to location 333
@333
D=A
@SP
A=M
M=D
@SP
M=M+1
// push segment constant to location 888
@888
D=A
@SP
A=M
M=D
@SP
M=M+1
// pop segment static to location 8
@SP
M=M-1
@SP
A=M
D=M
@StaticTest.vm.8
M=D
// pop segment static to location 3
@SP
M=M-1
@SP
A=M
D=M
@StaticTest.vm.3
M=D
// pop segment static to location 1
@SP
M=M-1
@SP
A=M
D=M
@StaticTest.vm.1
M=D
// push segment static to location 3
@StaticTest.vm.3
D=M
@SP
A=M
M=D
@SP
M=M+1
// push segment static to location 1
@StaticTest.vm.1
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
// push segment static to location 8
@StaticTest.vm.8
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
