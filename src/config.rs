use crate::verbosity::Verbosity;
use crate::colors::{RESET, YELLOW};
use std::env;

#[derive(Debug)]
pub struct Config {
    pub verbosity: Verbosity,
    pub files: Vec<String>,
}

pub fn print_config(config: &Config) {
    if config.verbosity == Verbosity::VERBOSE {
        println!("{YELLOW}verbosity: {RESET}{:?}", config.verbosity);
        println!("{YELLOW}files provided: {RESET}");
        for file in config.files.iter() {
            println!("\t{}", file);
        }
        println!{""};
    }
}

pub fn parse_args() -> Config {
    let mut verbosity = Verbosity::NORMAL;
    let mut files = vec![];

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-q" | "--quiet" => verbosity = Verbosity::QUIET,
            "-v" | "--verbose" => verbosity = Verbosity::VERBOSE,
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
