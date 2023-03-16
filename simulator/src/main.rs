use std::fs::File;

use log::info;
use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};

mod args;
mod instructions;
mod processor;
mod statemachine;
mod types;

fn main() {
    let args = args::parse_args();
    if !args.valid {
        println!("Invalid arguments, terminating");
        return;
    }
    println!("------------------------------------------------------------------------");
    CombinedLogger::init(vec![
        TermLogger::new(
            args.log_level,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Trace,
            simplelog::Config::default(),
            File::create("processor.log").unwrap(),
        ),
    ])
    .unwrap();
    // let path_to_file: &String = &args[1];
    let mut processor: processor::Processor = processor::Processor::new(args.path_to_file);
    let mut running: bool = true;
    info!("Beginning execution");
    while running {
        running = processor.run();
    }
    info!("Execution complete");
}
