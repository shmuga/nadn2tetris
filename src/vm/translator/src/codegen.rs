use crate::opcode::*;

struct Codegen {
    label_c: u16,
    context: String,
    result: String,
    filename: String,
    last_comment: String,
}

impl Codegen {
    pub fn new(filename: String) -> Codegen {
        Codegen {
            result: String::new(),
            label_c: 0,
            context: "Global".to_string(),
            last_comment: "".to_string(),
            filename
        }
    }

    fn generate(&mut self, opcodes: &Vec<Opcode>) -> String {
        for opcode in opcodes.iter() {
            match opcode {
                Opcode::Call(meta) => self.call(meta),
                Opcode::Function(meta) => self.function(meta),
                Opcode::Return => self.ret(),
                Opcode::Label(label) => self.label(&label),
                Opcode::Goto(label) => self.goto(&label),
                Opcode::IfGoto(label) => self.if_goto(&label),
                Opcode::Push(pc) => self.push(&pc),
                Opcode::Pop(pc) => self.pop(&pc),
                Opcode::Add => self.binary("+"),
                Opcode::Sub => self.binary("-"),
                Opcode::And => self.binary("&"),
                Opcode::Or => self.binary("|"),
                Opcode::Not => self.unary("!"),
                Opcode::Neg => self.unary("-"),
                Opcode::Lt => self.logical("LT"),
                Opcode::Gt => self.logical("GT"),
                Opcode::Eq => self.logical("EQ"),
                other => { println!("Missing implementation of {:?}", other)}
            }
        }

        self.result.to_owned()
    }


    fn entry(&mut self) -> String {
        self.write_all(vec![
            "@256",
            "D=A",
            "@SP",
            "M=D",
        ]);
        self.call(&FunctionMetadata {
            name: "Sys.init",
            argv: 0
        });
        self.result.to_owned()
    }

    fn comment(&mut self, comment: String) {
        self.write(&comment);
        // self.last_comment = comment;
    }

    fn write(&mut self, instruction: &str) {
        self.result.push_str(instruction);
        self.result.push_str("\n");
    }

    fn write_all(&mut self, instructions: Vec<&str>) {
        for instruction in instructions {
            self.write(instruction);
        }
    }

    fn s_inc(&mut self) {
        self.write_all(vec![
            "@SP",
            "M=M+1"
        ]);
    }

    fn s_dec(&mut self) {
        self.write_all(vec![
            "@SP",
            "M=M-1"
        ]);
    }

    fn s_pop(&mut self) {
        self.write_all(vec![
            "@SP",
            "M=M-1",
            "A=M",
        ]);
    }

    fn addr_val_to_d(&mut self, addr: &str) {
        self.write_all(vec![
            &format!("@{}", addr),
            "D=M"
        ]);
    }

    fn s_to_d(&mut self) {
        self.write_all(vec![
            "@SP",
            "A=M",
            "D=M"
        ]);
    }

    fn d_to_s(&mut self) {
        self.write_all(vec![
            "@SP",
            "A=M",
            "M=D"
        ]);
    }

    fn num_to_d(&mut self, i: &u32) {
        self.write_all(vec![
            &format!("@{}", i),
            "D=A"
        ]);
    }

    fn binary(&mut self, operand: &str) {
        self.comment(format!("// {}", operand));
        self.s_pop();
        self.write("D=M");
        self.s_pop();
        self.write(&format!("M=M{}D", operand));
        self.s_inc();
    }

    fn unary(&mut self, operand: &str) {
        self.comment(format!("// {}", operand));
        self.s_pop();
        self.write(&format!("M={}M", operand),);
        self.s_inc();
    }

    fn logical(&mut self, operand: &str) {
        self.comment(format!("// {}", operand));
        self.binary("-");
        self.s_pop();
        self.write_all(vec![
            "D=M",
            &format!("@{}$TRUE_{}", self.context, self.label_c),
            &format!("D;J{}", operand),
            "@SP",
            "A=M",
            "M=0",
            &format!("@{}$FINISH_{}", self.context, self.label_c),
            "1;JMP",
            &format!("({}$TRUE_{})", self.context, self.label_c),
            "@SP",
            "A=M",
            "M=-1",
            &format!("({}$FINISH_{})",self.context, self.label_c),
        ]);
        self.s_inc();
        self.label_c += 1;
    }

    fn push(&mut self, code: &SegmentMetadata) {
        self.comment(format!("// push segment {} to location {}", code.segment, code.i));
        match code.segment {
            "constant" => self.push_constant(&code.i),
            "local" => self.push_segment("LCL", &code.i),
            "argument" => self.push_segment("ARG", &code.i),
            "this" => self.push_segment("THIS", &code.i),
            "that" => self.push_segment("THAT", &code.i),
            "temp" => self.push_temp(&code.i),
            "pointer" => self.push_pointer(&code.i),
            "static" => self.push_static(&code.i),
            _ => panic!(format!("Unknown segmen {}", code.segment))
        }
    }

    fn pop(&mut self, code: &SegmentMetadata) {
        self.comment(format!("// pop segment {} to location {}", code.segment, code.i));
        match code.segment {
            "local" => self.pop_segment("LCL", &code.i),
            "argument" => self.pop_segment("ARG", &code.i),
            "this" => self.pop_segment("THIS", &code.i),
            "that" => self.pop_segment("THAT", &code.i),
            "temp" => self.pop_temp(&code.i),
            "pointer" => self.pop_pointer(&code.i),
            "static" => self.pop_static(&code.i),
            _ => panic!(format!("Unknown segmen {}", code.segment))
        }
    }

    fn pop_segment(&mut self, base: &str, i: &u32) {
        self.num_to_d(&i);
        self.write_all(vec![
            &format!("@{}", base),
            "A=M+D",
            "D=A",
            "@TEMP", // saving adress of base + i to variable
            "M=D",
        ]);

        self.s_dec();
        self.s_to_d();

        self.write_all(vec![
            "@TEMP",
            "A=M",
            "M=D"
        ]);
    }

    fn pop_temp(&mut self, i: &u32) {
        self.s_dec();
        self.s_to_d();
        self.write(&format!("@{}", 5 + i));
        self.write("M=D");
    }

    fn pop_pointer(&mut self, i: &u32) {
        let to = match i {
            1 => "THAT",
            _ => "THIS"
        };

        self.s_dec();
        self.s_to_d();
        self.write_all(vec![
            &format!("@{}", to),
            "M=D",
        ]);
    }

    fn pop_static(&mut self, i: &u32) {
        self.s_dec();
        self.s_to_d();
        self.write(&format!("@{}.{}", self.filename, i));
        self.write("M=D");
    }

    fn push_segment(&mut self, base: &str, i: &u32) {
        self.num_to_d(&i);
        self.write_all(vec![
            &format!("@{}", base),
            "A=M+D",
            "D=M",
        ]);
        self.d_to_s();
        self.s_inc();
    }

    fn push_temp(&mut self, i: &u32) {
        self.write(&format!("@{}", 5 + i));
        self.write("D=M");
        self.d_to_s();
        self.s_inc();
    }

    fn push_pointer(&mut self, i: &u32) {
        let to = match i {
            1 => "THAT",
            _ => "THIS"
        };

        self.write_all(vec![
            &format!("@{}", to),
            "D=M",
            "@SP",
            "A=M",
            "M=D"
        ]);
        self.s_inc();
    }

    fn push_constant(&mut self, i: &u32) {
        self.num_to_d(&i);
        self.d_to_s();
        self.s_inc();
    }

    fn push_static(&mut self, i: &u32) {
        self.write(&format!("@{}.{}", self.filename,i));
        self.write("D=M");
        self.d_to_s();
        self.s_inc();
    }

    fn label(&mut self, label: &LabelMetadata) {
        self.comment(format!("// label {}", label.name));
        self.write(&format!("({}${})", self.context, label.name));
    }

    fn goto(&mut self, label: &LabelMetadata) {
        self.comment(format!("// goto {}", label.name));
        self.write_all(vec![
            &format!("@{}${}", self.context, label.name),
            "1;JMP",
        ]);
    }

    fn if_goto(&mut self, label: &LabelMetadata) {
        self.comment(format!("// if-goto {}", label.name));
        self.s_dec();
        self.s_to_d();
        self.write_all(vec![
            &format!("@{}${}", self.context, label.name),
            "D;JNE",
        ]);
    }

    fn function(&mut self, meta: &FunctionMetadata) {
        self.comment(format!("// function {}.{}", meta.name, meta.argv));
        self.context = meta.name.to_string();
        self.write(&format!("({})", self.context));

        let mut argv = meta.argv;

        while argv != 0 {
            self.write_all(vec![
                "@SP",
                "A=M",
                "M=0"
            ]);
            self.s_inc();
            argv -= 1;
        }
    }

    fn push_d(&mut self) {
        self.d_to_s();
        self.s_inc();
    }

    fn call(&mut self, meta: &FunctionMetadata) {
        self.comment(format!("// call {}.{} from {}", meta.name, meta.argv, self.context));
        let return_label = format!("{}$ret.{}", self.context, self.label_c);
        self.label_c += 1;

        // write return adress
        self.write_all(vec![
            &format!("@{}", return_label),
            "D=A",
        ]);
        self.push_d();

        // save segments
        self.addr_val_to_d("LCL");
        self.push_d();
        self.addr_val_to_d("ARG");
        self.push_d();
        self.addr_val_to_d("THIS");
        self.push_d();
        self.addr_val_to_d("THAT");
        self.push_d();

        // reposition ARG
        self.write_all(vec![
            "@SP",
            "D=M",
            "@5",
            "D=D-A",
            &format!("@{}", meta.argv),
            "D=D-A",
            "@ARG",
            "M=D",
        ]);

        // reposition LCL
        self.write_all(vec![
            "@SP",
            "D=M",
            "@LCL",
            "M=D"
        ]);

        // jmp
        self.write_all(vec![
            &format!("@{}", meta.name),
            "1;JMP"
        ]);

        // return label
        self.write(&format!("({})", return_label));
    }

    fn ret(&mut self) {
        self.comment(format!("// return {}", self.context));
        // @R13 - temp variable for LCL or endFrame
        // @R14 - temp variable for retAddr

        // endframe is a temporary variable
        self.addr_val_to_d("LCL");
        self.write_all(vec![
            "@R13",
            "M=D"
        ]);

        // gets the return address
        self.write_all(vec![
            "@5",
            "A=D-A",
            "D=M",
            "@R14",
            "M=D"
        ]);

        // repositions the return value for the caller
        self.s_pop();
        self.write_all(vec![
            "D=M",
            "@ARG",
            "A=M",
            "M=D"
        ]);

        // repositions SP of the caller
        self.write_all(vec![
            "@ARG",
            "D=M+1",
            "@SP",
            "M=D"
        ]);


        // restores THAT of the caller
        self.addr_val_to_d("R13");
        self.write_all(vec![
            "@1",
            "A=D-A",
            "D=M",
            "@THAT",
            "M=D"
        ]);

        // restores THIS of the caller
        self.addr_val_to_d("R13");
        self.write_all(vec![
            "@2",
            "A=D-A",
            "D=M",
            "@THIS",
            "M=D"
        ]);

        // restores ARG of the caller
        self.addr_val_to_d("R13");
        self.write_all(vec![
            "@3",
            "A=D-A",
            "D=M",
            "@ARG",
            "M=D"
        ]);

        // restores THAT of the caller
        self.addr_val_to_d("R13");
        self.write_all(vec![
            "@4",
            "A=D-A",
            "D=M",
            "@LCL",
            "M=D"
        ]);

        // goes to the caller’s return address
        self.addr_val_to_d("R14");
        self.write_all(vec![
            "A=D",
            "1;JMP"
        ]);

    }
}


pub fn generate(opcodes: &Vec<Opcode>, filename: String) -> String {
    let mut cg = Codegen::new(filename);

    cg.generate(opcodes)
}

pub fn entry() -> String {
    let mut cg = Codegen::new("Entry".to_string());

    cg.entry()
}
