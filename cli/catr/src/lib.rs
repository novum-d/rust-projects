use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{arg, Command};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(_) => println!("Opened {}", filename),
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let mathces = Command::new("catr")
        .version("0.1.0")
        .author("novumd <novumd@gmail.com>")
        .about("Rust cat")
        .args(&[
            arg!(files: <FILE> "Input file(s)")
                .num_args(1..)
                .default_value("-"),
            arg!(number: -n --number "Number lines")
                .num_args(0)
                .conflicts_with("number_nonblank"),
            arg!(number_nonblank: -b --"number-nonblank" <TEXT> "Number non-blank lines")
                .num_args(0),
        ])
        .get_matches();

    let values_ref = mathces.get_many::<String>("files").unwrap();
    let string_vec = values_ref.cloned().collect::<Vec<_>>();

    Ok(Config {
        files: string_vec,
        number_lines: mathces.contains_id("number"),
        number_nonblank_lines: mathces.contains_id("number_nonblank"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
