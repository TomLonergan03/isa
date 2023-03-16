mod instructions;
mod processor;
mod statemachine;
mod types;

fn invalid_log_level() -> log::Level {
    println!("Invalid log level, defaulting to info");
    return log::Level::Info;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // args.iter().for_each(|x| x.contains("log="));

    let log_level = match &args[1] as &str {
        "debug" => log::Level::Debug,
        "info" => log::Level::Info,
        "warn" => log::Level::Warn,
        "error" => log::Level::Error,
        _ => invalid_log_level(),
    };
    simple_logger::init_with_level(log_level).unwrap();
    // let path_to_file: &String = &args[1];
    let path_to_file = "../example_bytecode/basic_addition.ayu";
    let mut processor: processor::Processor = processor::Processor::new(path_to_file.to_string());
    let mut running: bool = true;
    while running {
        running = processor.run();
    }
}
