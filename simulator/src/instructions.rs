pub fn parse_instruction(instruction: &str) -> Option<u16> {
    let instruction_cleaned: String = instruction.replace(" ", "");
    let instruction_string = instruction_cleaned
        .split("#")
        .next()
        .expect("Couldn't remove comment");
    if instruction_string == "" {
        println!("Empty line");
        return None;
    }
    let instruction: u16 = u16::from_str_radix(instruction_string, 16).expect("Invalid hex number");
    println!("{:#06X}", instruction);
    return Some(instruction);
}
