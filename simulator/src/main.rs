use std::fs::File;

use log::info;
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, LevelPadding, TargetPadding,
    TermLogger, TerminalMode, WriteLogger,
};

use simulator::{args, processor::Processor, types::RunState};

fn main() {
    let args = args::parse_args();
    if args.is_none() {
        return;
    }
    let args = args.unwrap();
    println!("------------------------------------------------------------------------");
    let config = ConfigBuilder::new()
        .set_level_padding(LevelPadding::Right)
        .set_thread_level(LevelFilter::Off)
        .set_location_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Error)
        .set_target_padding(TargetPadding::Right(30))
        .build();
    CombinedLogger::init(vec![
        TermLogger::new(
            args.log_level,
            config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Trace,
            config,
            File::create("processor.log").unwrap(),
        ),
    ])
    .unwrap();
    let mut processor: Processor =
        Processor::new_from_file(args.path_to_file, args.breakpoint, true);
    let mut running: RunState = RunState::Continue;
    info!("Beginning execution");
    while running == RunState::Continue {
        running = processor.run();
    }
    info!("Execution complete");
}
