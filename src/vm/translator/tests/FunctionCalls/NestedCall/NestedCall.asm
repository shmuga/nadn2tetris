@256
D=A
@SP
M=D
@Global$ret.0// call Sys.init.0 from Global
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
D=M
@5
D=D-A
@0
D=D-A
@ARG
M=D
@SP
D=M
@LCL
M=D
@Sys.init
1;JMP
(Global$ret.0)
(Sys.init)// function Sys.init.0
@4000// push segment constant to location 4000
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP// pop segment pointer to location 0
M=M-1
@SP
A=M
D=M
@THIS
M=D
@5000// push segment constant to location 5000
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP// pop segment pointer to location 1
M=M-1
@SP
A=M
D=M
@THAT
M=D
@Sys.init$ret.0// call Sys.main.0 from Sys.init
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
D=M
@5
D=D-A
@0
D=D-A
@ARG
M=D
@SP
D=M
@LCL
M=D
@Sys.main
1;JMP
(Sys.init$ret.0)
@SP// pop segment temp to location 1
M=M-1
@SP
A=M
D=M
@6
M=D
(Sys.init$LOOP)// label LOOP
@Sys.init$LOOP// goto LOOP
1;JMP
(Sys.main)// function Sys.main.5
@SP
A=M
M=0
@SP
M=M+1
@SP
A=M
M=0
@SP
M=M+1
@SP
A=M
M=0
@SP
M=M+1
@SP
A=M
M=0
@SP
M=M+1
@SP
A=M
M=0
@SP
M=M+1
@4001// push segment constant to location 4001
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP// pop segment pointer to location 0
M=M-1
@SP
A=M
D=M
@THIS
M=D
@5001// push segment constant to location 5001
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP// pop segment pointer to location 1
M=M-1
@SP
A=M
D=M
@THAT
M=D
@200// push segment constant to location 200
D=A
@SP
A=M
M=D
@SP
M=M+1
@1// pop segment local to location 1
D=A
@LCL
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
@40// push segment constant to location 40
D=A
@SP
A=M
M=D
@SP
M=M+1
@2// pop segment local to location 2
D=A
@LCL
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
@6// push segment constant to location 6
D=A
@SP
A=M
M=D
@SP
M=M+1
@3// pop segment local to location 3
D=A
@LCL
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
@123// push segment constant to location 123
D=A
@SP
A=M
M=D
@SP
M=M+1
@Sys.main$ret.1// call Sys.add12.1 from Sys.main
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
D=M
@5
D=D-A
@1
D=D-A
@ARG
M=D
@SP
D=M
@LCL
M=D
@Sys.add12
1;JMP
(Sys.main$ret.1)
@SP// pop segment temp to location 0
M=M-1
@SP
A=M
D=M
@5
M=D
@0// push segment local to location 0
D=A
@LCL
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1
@1// push segment local to location 1
D=A
@LCL
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1
@2// push segment local to location 2
D=A
@LCL
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1
@3// push segment local to location 3
D=A
@LCL
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1
@4// push segment local to location 4
D=A
@LCL
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP// +
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M+D
@SP
M=M+1
@SP// +
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M+D
@SP
M=M+1
@SP// +
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M+D
@SP
M=M+1
@SP// +
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M+D
@SP
M=M+1
@LCL// return Sys.main
D=M
@R13
M=D
@5
A=D-A
D=M
@R14
M=D
@SP
M=M-1
A=M
D=M
@ARG
A=M
M=D
@ARG
D=M+1
@SP
M=D
@R13
D=M
@1
A=D-A
D=M
@THAT
M=D
@R13
D=M
@2
A=D-A
D=M
@THIS
M=D
@R13
D=M
@3
A=D-A
D=M
@ARG
M=D
@R13
D=M
@4
A=D-A
D=M
@LCL
M=D
@R14
D=M
A=D
1;JMP
(Sys.add12)// function Sys.add12.0
@4002// push segment constant to location 4002
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP// pop segment pointer to location 0
M=M-1
@SP
A=M
D=M
@THIS
M=D
@5002// push segment constant to location 5002
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP// pop segment pointer to location 1
M=M-1
@SP
A=M
D=M
@THAT
M=D
@0// push segment argument to location 0
D=A
@ARG
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1
@12// push segment constant to location 12
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP// +
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M+D
@SP
M=M+1
@LCL// return Sys.add12
D=M
@R13
M=D
@5
A=D-A
D=M
@R14
M=D
@SP
M=M-1
A=M
D=M
@ARG
A=M
M=D
@ARG
D=M+1
@SP
M=D
@R13
D=M
@1
A=D-A
D=M
@THAT
M=D
@R13
D=M
@2
A=D-A
D=M
@THIS
M=D
@R13
D=M
@3
A=D-A
D=M
@ARG
M=D
@R13
D=M
@4
A=D-A
D=M
@LCL
M=D
@R14
D=M
A=D
1;JMP