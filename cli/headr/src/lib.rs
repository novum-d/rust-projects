use std::error::Error;
use std::fmt::format;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use clap::{arg, Arg, Command};

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

const BYTES: &str = "bytes";
const LINES: &str = "lines";
const FILES: &str = "files";

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("novumd <novumd@gmail.com>")
        .about("Rust head")
        .arg(
            Arg::new(BYTES)
                .help("Number of bytes-n")
                .long(BYTES)
                .required(true)
                .value_name(BYTES.to_uppercase())
        )
        .arg(
            Arg::new(LINES)
                .help("Number of lines")
                .long(LINES)
                .required(true)
                .value_name(LINES.to_uppercase())
                .default_value("10")
        )
        .arg(
            Arg::new(FILES)
                .help("Input file(s)")
                .value_name(FILES.to_uppercase())
                .required(true)
                .num_args(1..)
                .default_value("-")
        )
        .get_matches();

    let files = matches
        .get_many::<String>(FILES)
        .ok_or("Failed to get files")?
        .cloned()
        .collect::<Vec<_>>();

    let lines = matches.get_one::<String>(LINES)
        .map(|s| parse_positive_Int(s.as_str()))
        .transpose()
        .map_err(|e| { format!("Invalid value for{} ", e) })?
        .unwrap();

    let bytes = matches.get_one::<String>(BYTES)
        .map(|s| parse_positive_Int(s.as_str()))
        .transpose()
        .map_err(|e| { format!("Invalid value for{} ", e) })?;

    Ok(Config {
        files,
        lines,
        bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(e) => { eprintln!("Failed to open {}: {}", filename, e) }
            Ok(input) => {
                for line in input.lines().take(config.lines) {
                    println!("{}", line?)
                }
            }
        }
    }
    dbg!(&config);
    Ok(())
}

pub fn parse_positive_Int(s: &str) -> MyResult<usize> {
    // unimplemented!() // 未実装であることを示すプレースホルダ
    match s.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(s)),
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}