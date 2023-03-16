mod args;
mod instructions;
mod processor;
mod statemachine;
mod types;

fn main() {
    let args = args::parse_args();
    simple_logger::init_with_level(args.log_level).unwrap();
    // let path_to_file: &String = &args[1];
    let mut processor: processor::Processor = processor::Processor::new(args.path_to_file);
    let mut running: bool = true;
    while running {
        running = processor.run();
    }
}
