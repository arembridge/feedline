use crate::verbosity::Verbosity;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub verbosity: Verbosity,
    pub files: Vec<String>,
}

pub fn print_config(config: &Config) {
    if config.verbosity == Verbosity::Verbose {
        println!("Verbosity: {:?}", config.verbosity);
        println!("Files: {:?}", config.files);
    }
}

pub fn parse_args() -> Config {
    let mut verbosity = Verbosity::Normal;
    let mut files = vec![];

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-q" | "--quiet" => verbosity = Verbosity::Quiet,
            "-v" | "--verbose" => verbosity = Verbosity::Verbose,
            "-h" | "--help" => {
                super::help::print_help();
                std::process::exit(0);
            }
            _ => files.push(arg),
        }
    }

    
    let config = Config { verbosity, files };

    print_config(&config);

    return config;
}
