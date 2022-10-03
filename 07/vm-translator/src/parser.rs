use crate::vm_instruction::VMInstruction;

pub struct Parser {
    filename: String,
    reader: Box<dyn std::io::BufRead>,
    last_line: String,
}

impl Parser {
    pub fn new(vmfile_path: String) -> Self {
        let file = std::fs::File::open(vmfile_path.clone()).unwrap();
        let reader = Box::new(std::io::BufReader::new(file));
        let filename = vmfile_path
            .clone()
            .split("/")
            .last()
            .unwrap()
            .split(".")
            .next()
            .unwrap()
            .to_string();
        Self {
            filename,
            reader,
            last_line: String::new(),
        }
    }

    pub fn has_more_commands(&mut self) -> bool {
        let mut line = String::new();
        let mut valid_line = false;

        while !valid_line {
            line.clear();
            let bytes_read = self.reader.read_line(&mut line).unwrap();
            if bytes_read == 0 {
                return false;
            }
            valid_line = Self::valid_vm_instruction(&line);
        }
        self.last_line = line;
        return true;
    }

    fn valid_vm_instruction(line: &str) -> bool {
        if line.trim().is_empty() {
            return false;
        }
        if line.starts_with("//") {
            return false;
        }
        return true;
    }

    pub fn next_command(&self) -> anyhow::Result<VMInstruction> {
        VMInstruction::parse_from_line(self.last_line.trim(), &self.filename)
    }
}
