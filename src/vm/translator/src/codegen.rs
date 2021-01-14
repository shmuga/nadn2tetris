use crate::opcode::*;

struct Codegen {
    label: u16,
    result: String,
    filename: String,
}

impl Codegen {
    pub fn new(filename: String) -> Codegen {
        Codegen { result: String::new(), label: 0, filename }
    }

    fn generate(&mut self, opcodes: &Vec<Opcode>) -> String {
        for opcode in opcodes.iter() {
            match opcode {
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
        self.write(&format!("// {}", operand));
        self.s_pop();
        self.write("D=M");
        self.s_pop();
        self.write(&format!("M=M{}D", operand));
        self.s_inc();
    }

    fn unary(&mut self, operand: &str) {
        self.write(&format!("// {}", operand));
        self.s_pop();
        self.write(&format!("M={}M", operand),);
        self.s_inc();
    }

    fn logical(&mut self, operand: &str) {
        self.write(&format!("// {}", operand));
        self.binary("-");
        self.s_pop();
        self.write_all(vec![
            "D=M",
            &format!("@TRUE_{}", self.label),
            &format!("D;J{}", operand),
            "@SP",
            "A=M",
            "M=0",
            &format!("@FINISH_{}", self.label),
            "1;JMP",
            &format!("(TRUE_{})", self.label),
            "@SP",
            "A=M",
            "M=-1",
            &format!("@FINISH_{}", self.label),
            "1;JMP",
            &format!("(FINISH_{})", self.label),
        ]);
        self.s_inc();
        self.label += 1;
    }

    fn push(&mut self, code: &SegmentMetadata) {
        self.write(&format!("// push segment {} to location {}", code.segment, code.i));
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
        self.write(&format!("// pop segment {} to location {}", code.segment, code.i));
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

    fn label(&mut self, label: &LabelMetadata) {}

    fn goto(&mut self, label: &LabelMetadata) {
        self.write_all(vec![
            &format!("@{}", label.name),
            "1;JMP",
        ]);
    }

    fn if_goto(&mut self, label: &LabelMetadata) {}
}


pub fn generate(opcodes: &Vec<Opcode>, filename: String) -> String {
    let mut cg = Codegen::new(filename);

    cg.generate(opcodes)
}
