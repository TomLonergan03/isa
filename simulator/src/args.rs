#[derive(Clone)]
pub struct Args {
    pub log_level: simplelog::LevelFilter,
    pub path_to_file: String,
    pub help_set: bool,
}

fn log_level_from_string(log_level: &String) -> simplelog::LevelFilter {
    match log_level.replace("--log=", "").as_str() {
        "trace" => simplelog::LevelFilter::Trace,
        "debug" => simplelog::LevelFilter::Debug,
        "info" => simplelog::LevelFilter::Info,
        "warn" => simplelog::LevelFilter::Warn,
        "error" => simplelog::LevelFilter::Error,
        _ => {
            println!("Invalid log level, defaulting to info");
            return simplelog::LevelFilter::Info;
        }
    }
}

fn parse_arg(arg: &String, mut current_args: Args) -> Option<Args> {
    match arg {
        x if x.contains("--log=") => {
            current_args.log_level = log_level_from_string(&x);
            println!("Log level: {}", current_args.log_level);
        }
        x if x.contains("--file=") => {
            let file_path = parse_file_path(x);
            if file_path.is_none() {
                return None;
            }
            current_args.path_to_file = file_path.unwrap();
            println!("File: {}", current_args.path_to_file);
        }
        x if x.contains("--help") => {
            print_help();
            current_args.help_set = true;
        }
        _ => {
            println!("Invalid argument: {}", arg);
            return None;
        }
    }
    return Some(current_args);
}

fn parse_file_path(path_to_file: &String) -> Option<String> {
    let path_to_file = path_to_file.replace("--file=", "");
    if !std::path::Path::new(&path_to_file).exists() {
        eprintln!("No file at '{}'", path_to_file);
        return None;
    };
    return Some(path_to_file);
}

fn print_help() {
    println!("Usage: simulator [options]");
    println!("Options:");
    println!("--help                              - Print this help message");
    println!("--log=[trace|debug|info|warn|error] - Set the log level                       - Default = info");
    println!("--file=[path]                       - Set the path to the file to be executed - Default = ../example_bytecode/basic_addition.ayu");
}

pub fn parse_args() -> Option<Args> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let initial_args: Args = Args {
        log_level: simplelog::LevelFilter::Info,
        path_to_file: String::from("../example_bytecode/basic_addition.ayu"),
        help_set: false,
    };
    let parsed_args: Args = args.iter().fold(initial_args, |current_args, arg| {
        let previous_args: Args = current_args.clone();
        match parse_arg(arg, current_args) {
            Some(args) => args,
            None => previous_args,
        }
    });
    if parsed_args.help_set {
        return None;
    }
    return Some(parsed_args);
}
