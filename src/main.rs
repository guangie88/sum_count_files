extern crate failure;
extern crate glob;
extern crate log4rs;
#[macro_use]
extern crate log;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use failure::Error;
use glob::glob;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;
use structopt::StructOpt;

/// Main program configuration structure
#[derive(StructOpt, Debug)]
#[structopt(name = "Sum Count Files", about = "To sum all integers in the contents of files")]
struct MainConfig {
    /// Log configuration file path
    #[structopt(short = "l", long = "log", help = "Log configuration file path")]
    log_config_path: String,

    /// Glob pattern to apply for matching of files for acceptance
    #[structopt(short = "g", long = "glob",
                help = "Glob pattern to apply for matching of files for acceptance")]
    glob_match: String,
}

type Result<T> = std::result::Result<T, Error>;

fn run() -> Result<()> {
    let config = MainConfig::from_args();
    log4rs::init_file(&config.log_config_path, Default::default())?;

    let matching_files = glob(&config.glob_match)?;

    let count_sum: Result<u64> = matching_files
        .map(|file_path| -> Result<u64> {
                 let mut file = File::open(&file_path?)?;
                 let mut contents = String::new();
                 file.read_to_string(&mut contents)?;
                 Ok(contents.parse()?)
             })
        .sum();

    // deliberate print to stdout with no newline
    print!("{}", count_sum?);
    io::stdout().flush()?;

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => process::exit(0),
        Err(ref e) => {
            error!("sum_count_files error: {}\n > {}", e.cause(), e.backtrace());
            process::exit(1);
        }
    }
}
