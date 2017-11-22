extern crate failure;
extern crate filebuffer;
extern crate glob;
extern crate log4rs;
#[macro_use]
extern crate log;
extern crate rayon;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use failure::Error;
use filebuffer::FileBuffer;
use glob::glob;
use rayon::prelude::*;
use std::path::PathBuf;
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

fn run() -> Result<u64> {
    let config = MainConfig::from_args();
    log4rs::init_file(&config.log_config_path, Default::default())?;

    let matching_files: std::result::Result<Vec<PathBuf>, _> = glob(&config.glob_match)?.collect();
    let matching_files = matching_files?;

    let count_sum: Result<u64> = matching_files
        .into_par_iter()
        .map(|file_path| -> Result<u64> {
            let buf = FileBuffer::open(&file_path)?;
            let s = std::str::from_utf8(&buf)?;
            let value = s.parse()?;
            Ok(value)
        })
        .sum();

    let count_sum = count_sum?;

    Ok(count_sum)
}

fn main() {
    match run() {
        Ok(count_sum) => {
            // deliberate print to stdout
            println!("{}", count_sum);
            process::exit(0)
        }

        Err(ref e) => {
            error!("sum_count_files error: {}\n > {}", e.cause(), e.backtrace());
            process::exit(1);
        }
    }
}
