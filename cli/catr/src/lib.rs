use std::{
    error::Error,
    fs,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{arg, Command};
use rand::{distr::Alphanumeric, Rng};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    // println!("{:#?}", &config);
    // dbg!(&config);
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(input) => {
                for (i, line) in input.lines().enumerate() {
                    let number_str = if config.number_lines {
                        format!("{:>6}\t", i + 1)
                    } else {
                        String::new()
                    };
                    println!("{} {}", number_str, line?);
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,          // Input files
    number_lines: bool,          // Number lines
    number_nonblank_lines: bool, // Number non-blank lines
}

fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        if fs::metadata(&filename).is_err() {
            // ファイルが存在しない場合
            return filename;
        }
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("novumd <novumd@gmail.com>")
        .about("Rust cat")
        .arg(
            arg!(files: [FILES] ... "Input file(s)") // <FILES>ではなく、[FILES]とすることで必須引数ではなくなる、
                .default_values(["-"])
                .num_args(1..),
        )
        .arg(
            arg!(number: -n --number "Number lines")
                .num_args(0)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            arg!(number_nonblank: -b --"number-nonblank" <TEXT> "Number non-blank lines")
                .num_args(0),
        )
        .get_matches();

    let values_ref = matches
        .get_many::<String>("files")
        .ok_or("Failed to get files")?;
    let string_vec = values_ref.cloned().collect::<Vec<_>>();

    Ok(Config {
        files: string_vec,
        number_lines: matches.contains_id("number"),
        number_nonblank_lines: matches.contains_id("number_nonblank"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
