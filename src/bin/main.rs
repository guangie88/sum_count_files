#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", deny(warnings))]

extern crate failure;
extern crate glob;
extern crate log4rs;
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use failure::Error;
use glob::glob;
use log::LogLevel;
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
    log_config_path: Option<String>,

    /// Glob pattern to apply for matching of files for acceptance
    #[structopt(short = "g", long = "glob",
                help = "Glob pattern to apply for matching of files for acceptance")]
    glob_match: String,
}

type Result<T> = std::result::Result<T, Error>;

fn run_impl<S: AsRef<str>>(glob_match: S) -> Result<u64> {
    let matching_files = glob(glob_match.as_ref())?;

    let count_sum: Result<u64> = matching_files
        .map(|file_path| -> Result<u64> {
            let mut file = File::open(&file_path?)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(contents.parse()?)
        })
        .sum();

    Ok(count_sum?)
}

fn run() -> Result<u64> {
    let config = MainConfig::from_args();

    if let Some(log_config_path) = config.log_config_path {
        log4rs::init_file(log_config_path, Default::default())?;
    } else {
        simple_logger::init_with_level(LogLevel::Error)?;
    }

    run_impl(&config.glob_match)
}

fn main() {
    match run() {
        Ok(count_sum) => {
            // deliberate print to stdout with no newline
            print!("{}", count_sum);

            io::stdout()
                .flush()
                .expect("Unable to flush count_sum into stdout");

            process::exit(0)
        }
        Err(ref e) => {
            error!(
                "sum_count_files error: {}\n > BACKTRACE: {}",
                e.cause(),
                e.backtrace()
            );
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // all TRUTH_VALUE values are obtained from
    // `find . -name '*.count' -exec cat {} \; -exec echo \;`
    // and the values are then tabulated and summed in a spreadsheet

    #[test]
    fn st20170901() {
        const TRUTH_VALUE: u64 = 902_988_741;

        let count_sum = run_impl("data/st20170901/*.count")
            .expect("Make sure that data/st20170901 has all the .count files");

        assert_eq!(TRUTH_VALUE, count_sum);
    }

    #[test]
    fn st20170902() {
        const TRUTH_VALUE: u64 = 912_080_400;

        let count_sum = run_impl("data/st20170902/*.count")
            .expect("Make sure that data/st20170902 has all the .count files");

        assert_eq!(TRUTH_VALUE, count_sum);
    }

    #[test]
    fn error_files() {
        if run_impl("data/error-files/*.count").is_ok() {
            panic!(
                "Make sure that data/error-files is present and has at least one error .count file"
            );
        }
    }
}
