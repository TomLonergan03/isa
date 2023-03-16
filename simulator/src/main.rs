mod instructions;
mod processor;
mod statemachine;
mod types;

fn main() {
    // let args: Vec<String> = std::env::args().collect();
    // let path_to_file: &String = &args[1];
    let path_to_file = "../example_bytecode/basic_addition.ayu";
    let mut processor: processor::Processor = processor::Processor::new(path_to_file.to_string());
    let mut running: bool = true;
    while running {
        running = processor.run();
    }
}
