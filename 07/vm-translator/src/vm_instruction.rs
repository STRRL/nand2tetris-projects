use std::vec;

use anyhow::anyhow;

#[derive(Debug)]
pub enum VMInstruction {
    Arithmetic(ArithmeticInstruction),
    Push(MemoryAccess),
    Pop(MemoryAccess),
    // TODO with project 8
    // Label,
    // Goto,
    // If,
    // Function,
    // Return,
    // Call,
}
pub trait ToASM {
    fn to_asm(&self) -> Vec<String>;
}

impl ToASM for VMInstruction {
    fn to_asm(&self) -> Vec<String> {
        match self {
            VMInstruction::Arithmetic(instruction) => instruction.to_asm(),
            VMInstruction::Push(memory_access) => {
                if let MemorySegment::Constant = memory_access.segment {
                    return vec![
                        format!("@{}", memory_access.index).as_str(),
                        "D=A",
                        "@SP",
                        "A=M",
                        "M=D",
                        "@SP",
                        "M=M+1",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect();
                }
                todo!()
            }
            VMInstruction::Pop(memory_access) => {
                todo!()
            }
        }
    }
}

impl TryFrom<&str> for VMInstruction {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();

        if tokens.len() == 1 {
            return match line {
                "add" => Ok(VMInstruction::Arithmetic(ArithmeticInstruction::Add)),
                "sub" => Ok(VMInstruction::Arithmetic(ArithmeticInstruction::Sub)),
                "neg" => Ok(VMInstruction::Arithmetic(ArithmeticInstruction::Neg)),
                "eq" => Ok(VMInstruction::Arithmetic(ArithmeticInstruction::Eq)),
                "gt" => Ok(VMInstruction::Arithmetic(ArithmeticInstruction::Gt)),
                "lt" => Ok(VMInstruction::Arithmetic(ArithmeticInstruction::Lt)),
                "and" => Ok(VMInstruction::Arithmetic(ArithmeticInstruction::And)),
                "or" => Ok(VMInstruction::Arithmetic(ArithmeticInstruction::Or)),
                "not" => Ok(VMInstruction::Arithmetic(ArithmeticInstruction::Not)),
                _ => Err(anyhow!("Invalid instruction: {}", line)),
            };
        }

        if tokens.len() == 3 {
            let segment = match tokens[1] {
                "local" => MemorySegment::Local,
                "argument" => MemorySegment::Argument,
                "this" => MemorySegment::This,
                "that" => MemorySegment::That,
                "constant" => MemorySegment::Constant,
                "static" => MemorySegment::Static,
                "pointer" => MemorySegment::Pointer,
                "temp" => MemorySegment::Temp,
                _ => return Err(anyhow!("Invalid segment: {}", tokens[1])),
            };

            let index = tokens[2].parse::<u32>()?;

            return match tokens[0] {
                "push" => Ok(VMInstruction::Push(MemoryAccess::new(segment, index))),
                "pop" => Ok(VMInstruction::Pop(MemoryAccess::new(segment, index))),
                _ => Err(anyhow!("Invalid instruction: {}", line)),
            };
        }

        todo!()
    }
}

#[derive(Debug)]
pub enum ArithmeticInstruction {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

impl ToASM for ArithmeticInstruction {
    fn to_asm(&self) -> Vec<String> {
        match self {
            ArithmeticInstruction::Add => vec![
                "@SP", "A=M-1", "D=M", "@SP", "A=M-1", "A=A-1", "M=M+D", "@SP", "M=M-1",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            ArithmeticInstruction::Sub => {
                vec!["@SP", "A=A-1", "D=M", "A=A-1", "M=M-D", "@SP", "M=M-1"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            }
            ArithmeticInstruction::Neg => vec!["@SP", "A=A-1", "M=-M"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ArithmeticInstruction::Eq => vec![
                "@SP",
                "A=A-1",
                "D=M",
                "A=A-1",
                "D=M-D",
                "@EQ_TRUE",
                "D;JEQ",
                "@SP",
                "A=A-2",
                "M=0",
                "@EQ_END",
                "0;JMP",
                "(EQ_TRUE)",
                "@SP",
                "A=A-2",
                "M=1",
                "(EQ_END)",
                "@SP",
                "M=M-1",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            ArithmeticInstruction::Gt => vec![
                "@SP",
                "A=A-1",
                "D=M",
                "A=A-1",
                "D=M-D",
                "@GT_TRUE",
                "D;JGT",
                "@SP",
                "A=A-2",
                "M=0",
                "@GT_END",
                "0;JMP",
                "(GT_TRUE)",
                "@SP",
                "A=A-2",
                "M=1",
                "(GT_END)",
                "@SP",
                "M=M-1",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            ArithmeticInstruction::Lt => vec![
                "@SP",
                "A=A-1",
                "D=M",
                "A=A-1",
                "D=M-D",
                "@LT_TRUE",
                "D;JLT",
                "@SP",
                "@SP",
                "A=A-2",
                "@LT_END",
                "0;JMP",
                "(LT_TRUE)",
                "@SP",
                "A=A-2",
                "M=1",
                "(LT_END)",
                "@SP",
                "M=M-1",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            ArithmeticInstruction::And => {
                vec!["@SP", "A=A-1", "D=M", "A=A-1", "M=D&M", "@SP", "M=M-1"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            }
            ArithmeticInstruction::Or => {
                vec!["@SP", "A=A-1", "D=M", "A=A-1", "M=D|M", "@SP", "M=M-1"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            }
            ArithmeticInstruction::Not => vec!["@SP", "A=A-1", "M=!M"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct MemoryAccess {
    pub segment: MemorySegment,
    pub index: u32,
}

impl MemoryAccess {
    fn new(segment: MemorySegment, index: u32) -> Self {
        Self {
            segment,
            index: index,
        }
    }
}

#[derive(Debug)]
pub enum MemorySegment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

impl TryFrom<&str> for MemorySegment {
    type Error = anyhow::Error;
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        match line {
            "argument" => Ok(MemorySegment::Argument),
            "local" => Ok(MemorySegment::Local),
            "static" => Ok(MemorySegment::Static),
            "constant" => Ok(MemorySegment::Constant),
            "this" => Ok(MemorySegment::This),
            "that" => Ok(MemorySegment::That),
            "pointer" => Ok(MemorySegment::Pointer),
            "temp" => Ok(MemorySegment::Temp),
            value => Err(anyhow!("Invalid memory segment: {}", value)),
        }
    }
}
