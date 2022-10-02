mod code_writer;
mod parser;
mod vm_instruction;

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    let vmfile_path_option = std::env::args().nth(1);
    if vmfile_path_option.is_none() {
        println!("Usage: vm-tanslator <vmfile>");
        return Ok(());
    }
    let vm_file_path = vmfile_path_option.unwrap();
    println!("Open HACK VM file: {}", vm_file_path);
    let mut parser = parser::Parser::new(vm_file_path.clone());
    let asm_file_path = vm_file_path.clone().replace(".vm", ".asm");
    let mut writer = code_writer::CodeWriter::new(Box::new(
        std::fs::File::create(asm_file_path).expect("Failed to create asm file"),
    ));
    while parser.has_more_commands() {
        let instruction = parser.next_command()?;
        writer.write_instruction(&instruction);
    }
    return Ok(());
}
