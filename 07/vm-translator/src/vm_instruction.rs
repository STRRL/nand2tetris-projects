use std::vec;

use anyhow::anyhow;
use uuid::Uuid;

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
            VMInstruction::Push(memory_access) => match &memory_access.segment {
                MemorySegment::Argument
                | MemorySegment::Local
                | MemorySegment::This
                | MemorySegment::That
                | MemorySegment::Temp => {
                    let base_address = match &memory_access.segment {
                        MemorySegment::Argument => "ARG",
                        MemorySegment::Local => "LCL",
                        MemorySegment::This => "THIS",
                        MemorySegment::That => "THAT",
                        MemorySegment::Temp => "R5",
                        _ => unreachable!(),
                    };

                    let load_address_to_d = match &memory_access.segment {
                        MemorySegment::Temp => vec![
                            format!("@{}", base_address),
                            "D=A".to_string(),
                            format!("@{}", memory_access.index),
                            "D=D+A".to_string(),
                        ],
                        _ => vec![
                            format!("@{}", base_address),
                            "D=M".to_string(),
                            format!("@{}", memory_access.index),
                            "D=D+A".to_string(),
                        ],
                    };
                    return vec![
                        load_address_to_d,
                        vec!["A=D", "D=M", "@SP", "A=M", "M=D", "@SP", "M=M+1"]
                            .iter()
                            .map(|s| s.to_string())
                            .collect(),
                    ]
                    .concat()
                    .iter()
                    .map(|s| s.to_string())
                    .collect();
                }
                MemorySegment::Constant => {
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
                MemorySegment::Pointer => match &memory_access.index {
                    0 => {
                        return vec!["@THIS", "D=M", "@SP", "A=M", "M=D", "@SP", "M=M+1"]
                            .iter()
                            .map(|s| s.to_string())
                            .collect();
                    }
                    1 => {
                        return vec!["@THAT", "D=M", "@SP", "A=M", "M=D", "@SP", "M=M+1"]
                            .iter()
                            .map(|s| s.to_string())
                            .collect();
                    }
                    _ => unreachable!(
                        "pointer index must be 0 or 1, current: {}",
                        memory_access.index
                    ),
                },
                MemorySegment::Static(filename) => vec![
                    format!("@{}.{}", filename, memory_access.index).as_str(),
                    "D=M",
                    "@SP",
                    "A=M",
                    "M=D",
                    "@SP",
                    "M=M+1",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            },
            VMInstruction::Pop(memory_access) => match &memory_access.segment {
                MemorySegment::Argument
                | MemorySegment::Local
                | MemorySegment::This
                | MemorySegment::That
                | MemorySegment::Temp => {
                    let base_address = match memory_access.segment {
                        MemorySegment::Argument => "ARG",
                        MemorySegment::Local => "LCL",
                        MemorySegment::This => "THIS",
                        MemorySegment::That => "THAT",
                        MemorySegment::Temp => "R5",
                        _ => unreachable!(
                            "could not get base address for {:?}",
                            memory_access.segment
                        ),
                    };

                    let load_address_to_d = match &memory_access.segment {
                        MemorySegment::Temp => vec![
                            format!("@{}", base_address),
                            "D=A".to_string(),
                            format!("@{}", memory_access.index),
                            "D=D+A".to_string(),
                        ],
                        _ => vec![
                            format!("@{}", base_address),
                            "D=M".to_string(),
                            format!("@{}", memory_access.index),
                            "D=D+A".to_string(),
                        ],
                    };

                    return vec![
                        load_address_to_d,
                        vec!["@R13", "M=D", "@SP", "AM=M-1", "D=M", "@R13", "A=M", "M=D"]
                            .iter()
                            .map(|s| s.to_string())
                            .collect(),
                    ]
                    .concat()
                    .iter()
                    .map(|s| s.to_string())
                    .collect();
                }
                MemorySegment::Static(filename) => vec![
                    "@SP",
                    "AM=M-1",
                    "D=M",
                    format!("@{}.{}", filename, memory_access.index).as_str(),
                    "M=D",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                MemorySegment::Pointer => match &memory_access.index {
                    0 => {
                        return vec!["@SP", "AM=M-1", "D=M", "@THIS", "M=D"]
                            .iter()
                            .map(|s| s.to_string())
                            .collect();
                    }
                    1 => {
                        return vec!["@SP", "AM=M-1", "D=M", "@THAT", "M=D"]
                            .iter()
                            .map(|s| s.to_string())
                            .collect();
                    }
                    _ => unreachable!(
                        "pointer index must be 0 or 1, current: {}",
                        memory_access.index
                    ),
                },
                MemorySegment::Constant => unreachable!("could not pop to constant segment"),
            },
        }
    }
}

impl VMInstruction {
    pub fn parse_from_line(line: &str, filename: &str) -> Result<Self, anyhow::Error> {
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
                "static" => MemorySegment::Static(filename.to_string()),
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
            ArithmeticInstruction::Sub => vec![
                "@SP", "A=M-1", "D=M", "@SP", "A=M-1", "A=A-1", "M=M-D", "@SP", "M=M-1",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            ArithmeticInstruction::Neg => vec!["@SP", "A=M-1", "M=-M"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ArithmeticInstruction::Eq => {
                let label = Uuid::new_v4().to_string();
                vec![
                    "@SP",
                    "A=M-1",
                    "D=M",
                    "A=A-1",
                    "D=M-D",
                    format!("@EQ_TRUE_{}", label).as_str(),
                    "D;JEQ",
                    "@SP",
                    "A=M-1",
                    "A=A-1",
                    "M=0",
                    format!("@EQ_END_{}", label).as_str(),
                    "0;JMP",
                    format!("(EQ_TRUE_{})", label).as_str(),
                    "@SP",
                    "A=M-1",
                    "A=A-1",
                    "M=-1",
                    format!("(EQ_END_{})", label).as_str(),
                    "@SP",
                    "M=M-1",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect()
            }
            ArithmeticInstruction::Gt => {
                let label = Uuid::new_v4().to_string();
                vec![
                    "@SP",
                    "A=M-1",
                    "D=M",
                    "A=A-1",
                    "D=M-D",
                    format!("@GT_TRUE_{}", label).as_str(),
                    "D;JGT",
                    "@SP",
                    "A=M-1",
                    "A=A-1",
                    "M=0",
                    format!("@GT_END_{}", label).as_str(),
                    "0;JMP",
                    format!("(GT_TRUE_{})", label).as_str(),
                    "@SP",
                    "A=M-1",
                    "A=A-1",
                    "M=-1",
                    format!("(GT_END_{})", label).as_str(),
                    "@SP",
                    "M=M-1",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect()
            }
            ArithmeticInstruction::Lt => {
                let label = Uuid::new_v4().to_string();
                vec![
                    "@SP",
                    "A=M-1",
                    "D=M",
                    "A=A-1",
                    "D=M-D",
                    format!("@LT_TRUE_{}", label).as_str(),
                    "D;JLT",
                    "@SP",
                    "A=M-1",
                    "A=A-1",
                    "M=0",
                    format!("@LT_END_{}", label).as_str(),
                    "0;JMP",
                    format!("(LT_TRUE_{})", label).as_str(),
                    "@SP",
                    "A=M-1",
                    "A=A-1",
                    "M=-1",
                    format!("(LT_END_{})", label).as_str(),
                    "@SP",
                    "M=M-1",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect()
            }
            ArithmeticInstruction::And => {
                vec!["@SP", "A=M-1", "D=M", "A=A-1", "M=D&M", "@SP", "M=M-1"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            }
            ArithmeticInstruction::Or => {
                vec!["@SP", "A=M-1", "D=M", "A=A-1", "M=D|M", "@SP", "M=M-1"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            }
            ArithmeticInstruction::Not => vec!["@SP", "A=M-1", "M=!M"]
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
    Static(String),
    Constant,
    This,
    That,
    Pointer,
    Temp,
}
