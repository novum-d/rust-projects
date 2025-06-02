use std::{
    error::Error,
    fs,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{arg, Arg, Command};
use rand::{distr::Alphanumeric, Rng};

type MyResult<T> = Result<T, Box<dyn Error>>;

const NUMBER: &str = "number";
const NUMBER_NON_BLANK: &str = "number_nonblank";
const FORMAT: &str = "{:>6}{:>2}\t";

fn create_format(number: usize, line: &str) -> String {
    format!("{:>6}\t{}", number, line)
}

pub fn run(config: Config) -> MyResult<()> {
    // println!("{:#?}", &config);
    dbg!(&config);
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(input) => {
                let mut i: usize = 0;
                for line in input.lines() {
                    let line = match config {
                        // 空行にも番号を付ける
                        Config {
                            number_lines: true, ..
                        } => {
                            let line = create_format(i + 1, &line?);
                            line
                        }

                        // 空行には番号をつけない
                        Config {
                            number_nonblank_lines: true,
                            ..
                        } => {
                            let line = line?;
                            if line.trim().is_empty() {
                                i -= 1;
                                String::new()
                            } else {
                                create_format(i + 1, &line)
                            }
                        }
                        _ => line?,
                    };
                    i += 1;
                    println!("{}", line);
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
            Arg::new(NUMBER)
                .short('n')
                .long(NUMBER)
                .help("Number lines")
                .num_args(0)
                .conflicts_with(NUMBER_NON_BLANK),
        )
        .arg(
            Arg::new(NUMBER_NON_BLANK)
                .short('b')
                .long(&NUMBER_NON_BLANK.replace('_', "-"))
                .help("Number non-blank lines")
                .num_args(0),
        )
        .get_matches();

    let values_ref = matches
        .get_many::<String>("files")
        .ok_or("Failed to get files")?;
    let string_vec = values_ref.cloned().collect::<Vec<_>>();

    Ok(Config {
        files: string_vec,
        number_lines: matches.get_flag("number"),
        number_nonblank_lines: matches.get_flag("number_nonblank"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
