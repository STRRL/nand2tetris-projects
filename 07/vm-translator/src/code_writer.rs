use crate::vm_instruction::{ToASM, VMInstruction};

pub struct CodeWriter {
    writer: Box<dyn std::io::Write>,
}

impl CodeWriter {
    pub fn new(writer: Box<dyn std::io::Write>) -> Self {
        Self { writer }
    }

    pub fn write_instruction(&mut self, instruction: &VMInstruction) {
        for line in instruction.to_asm() {
            self.writer.write_all(line.as_bytes()).unwrap();
            self.writer.write("\n".as_bytes()).unwrap();
        }
    }
}
