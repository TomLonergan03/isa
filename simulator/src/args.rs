use log::error;

pub struct Args {
    pub log_level: log::Level,
    pub path_to_file: String,
}

fn log_level_from_string(log_level: &String) -> log::Level {
    match log_level.replace("log=", "").as_str() {
        "trace" => log::Level::Trace,
        "debug" => log::Level::Debug,
        "info" => log::Level::Info,
        "warn" => log::Level::Warn,
        "error" => log::Level::Error,
        _ => {
            println!("Invalid log level, defaulting to info");
            return log::Level::Info;
        }
    }
}

pub fn parse_args() -> Args {
    let args: Vec<String> = std::env::args().collect();
    let mut parsed_args: Args = Args {
        log_level: log::Level::Info,
        path_to_file: String::new(),
    };

    let log_level: Vec<&String> = args.iter().filter(|x| x.contains("log=")).collect();
    match log_level.len() {
        0 => {
            println!("No log level specified, defaulting to info");
            parsed_args.log_level = log::Level::Info;
        }
        1 => parsed_args.log_level = log_level_from_string(log_level[0]),
        _ => {
            println!("Multiple log levels specified, using first option");
            parsed_args.log_level = log_level_from_string(log_level[0])
        }
    }

    let path_to_file: Vec<&String> = args.iter().filter(|x| x.contains("file=")).collect();
    match path_to_file.len() {
        0 => {
            println!("No file specified, defaulting to example_bytecode/basic_addition.ayu");
            if !std::path::Path::new("../example_bytecode/basic_addition.ayu").exists() {
                error!("Default file not found at '../example_bytecode/basic_addition.ayu'");
            };
            parsed_args.path_to_file = "../example_bytecode/basic_addition.ayu".to_string();
        }
        1 => {
            if !std::path::Path::new(path_to_file[0]).exists() {
                error!("No file at '{}'", path_to_file[0]);
            };
            parsed_args.path_to_file = path_to_file[0].clone()
        }
        _ => {
            println!("Multiple files specified, using first option");
            if !std::path::Path::new(path_to_file[0]).exists() {
                error!("No file at '{}'", path_to_file[0]);
            };
            parsed_args.path_to_file = path_to_file[0].clone();
        }
    }
    return parsed_args;
}
