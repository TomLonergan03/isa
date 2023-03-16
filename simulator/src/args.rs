pub struct Args {
    pub log_level: simplelog::LevelFilter,
    pub path_to_file: String,
    pub valid: bool,
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

pub fn parse_args() -> Args {
    let args: Vec<String> = std::env::args().collect();
    let mut parsed_args: Args = Args {
        log_level: simplelog::LevelFilter::Info,
        path_to_file: String::new(),
        valid: true,
    };

    let log_level: Vec<&String> = args.iter().filter(|x| x.contains("--log=")).collect();
    match log_level.len() {
        0 => {
            println!("No log level specified");
            parsed_args.log_level = simplelog::LevelFilter::Info;
        }
        1 => parsed_args.log_level = log_level_from_string(log_level[0]),
        _ => {
            println!("Multiple log levels specified, using first option");
            parsed_args.log_level = log_level_from_string(log_level[0])
        }
    }
    println!("Log level: {}", parsed_args.log_level);

    let path_to_file: Vec<String> = args
        .iter()
        .filter(|x| x.contains("--file="))
        .map(|x| x.replace("--file=", ""))
        .collect();
    match path_to_file.len() {
        0 => {
            if !std::path::Path::new("../example_bytecode/basic_addition.ayu").exists() {
                eprintln!("Default file not found at '../example_bytecode/basic_addition.ayu'");
                parsed_args.valid = false;
            };
            parsed_args.path_to_file = "../example_bytecode/basic_addition.ayu".to_string();
        }
        1 => {
            if !std::path::Path::new(&path_to_file[0]).exists() {
                eprintln!("No file at '{}'", path_to_file[0]);
                parsed_args.valid = false;
            };
            parsed_args.path_to_file = path_to_file[0].clone();
        }
        _ => {
            println!("Multiple files specified, using first option");
            if !std::path::Path::new(&path_to_file[0]).exists() {
                eprintln!("No file at '{}'", path_to_file[0]);
                parsed_args.valid = false;
            };
            parsed_args.path_to_file = path_to_file[0].clone();
        }
    }
    println!("File: {}", parsed_args.path_to_file);
    return parsed_args;
}
