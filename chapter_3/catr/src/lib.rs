// ----------------------------------------------------------------- //
// imports
// ----------------------------------------------------------------- //

use clap::{value_parser, Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// ----------------------------------------------------------------- //
// definitions
// ----------------------------------------------------------------- //

// structs
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

// types
type MyResult<T> = Result<T, Box<dyn Error>>;

// ----------------------------------------------------------------- //
// functions
// ----------------------------------------------------------------- //

// main
pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut line_counter: i8 = 0; // non-blank only
                for (line_count, line_result) in file.lines().enumerate() {
                    let line = line_result?; // get line content
                    if config.number_lines {
                        println!("{:>6}\t{}", line_count + 1, line); // print the line number, bumped to start @ 1
                    } else if config.number_nonblank_lines && !line.is_empty() {
                        line_counter += 1;
                        println!("{:>6}\t{}", line_counter, line); // print non-blank lines with numbers
                    } else {
                        println!("{}", line); // print lines
                    }
                }
            }
        }
    }
    Ok(())
}

// handle arguments
pub fn get_args() -> MyResult<Config> {
    let mut matches = Command::new("catr")
        .version("0.1.0")
        .author("Andy Baizon <dev@baizon.uk>")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .value_parser(value_parser!(String))
                .help("Input file(s)")
                .num_args(0..)
                .default_value("-"),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .help("Print line numbers")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .help("Print non-blank line numbers")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .remove_many("files")
            .expect("`files` is required")
            .into_iter()
            .collect(),
        number_lines: matches.get_flag("number_lines"),
        number_nonblank_lines: matches.get_flag("number_nonblank_lines"),
    })
}

// open files
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// ----------------------------------------------------------------- //
// EOF
// ----------------------------------------------------------------- //
