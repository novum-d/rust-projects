use std::error::Error;
use std::fmt::format;
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
    dbg!(config);
    Ok(())
}

pub fn parse_positive_Int(s: &str) -> MyResult<usize> {
    // unimplemented!() // 未実装であることを示すプレースホルダ
    match s.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(s)),
    }
}