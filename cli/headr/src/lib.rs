use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::str::{from_utf8};
use clap::{Arg, Command};

/*
デフォルトで１０件で表示
* -nオプションは指定された行数だけを表示
-cオプションは指定されたバイト分だけを表示
ファイルが存在しない場合、エラーメッセージを出力（head: blargh: No such file or directory）
ファイルが権限がない場合、エラーメッセージを出力（head: can't touch this: Permission denied）
ファイルに標準入力を与えない場合は、標準入力からコマンドを読み込む
複数ファイルがある場合は、ヘッダーが表示される(==> input/hoge.txt <==)
n, cの両方を指定するとエラーになる(head: can't combine line and byte counts)
n, cともに不正な値が入力されたときは、プログラムの実行を停止（head: illegal count -- {}）
 */

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
            Arg::new(LINES)
                .help("Number of lines")
                .short('n')
                .long(LINES)
                .value_name(LINES.to_uppercase())
                .conflicts_with(BYTES)
                .default_value("10")
        )
        .arg(
            Arg::new(BYTES)
                .help("Number of bytes-n")
                .short('c')
                .long(BYTES)
                .value_name(BYTES.to_uppercase())
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
        .map(|s| parse_positive_int(s.as_str()))
        .transpose()
        .map_err(|e| { format!("Invalid value for{} ", e) })?
        .unwrap();

    let bytes = matches.get_one::<String>(BYTES)
        .map(|s| parse_positive_int(s.as_str()))
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
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(mut input) => {
                let output: String = match config {
                    Config { bytes: Some(bytes_count), .. } => {
                        let mut buffer: Vec<u8> = vec![0; bytes_count];
                        let bytes_end_index = input.read(&mut buffer)?;
                        // utf-8文字列が途中で、切れてしまう（8で割り切れない）場合は切り詰める
                        let bytes_end_index = match from_utf8(&buffer[..bytes_end_index]) {
                            Ok(_) => bytes_end_index,
                            Err(e) => e.valid_up_to()
                        };
                        String::from_utf8_lossy(&buffer[..bytes_end_index]).into()
                    }
                    Config { lines, .. } if lines > 0 => {
                        let mut line_count: usize = 1;
                        let mut output = "".to_string();
                        for line in input.lines().take(config.lines) {
                            output.push_str(&format!("{:>6} \t{}\n", line_count, line?));
                            line_count += 1;
                        }
                        output
                    }
                    _ => { "".to_string() }
                };
                print!("{}", output)
            }
        }
    }
    dbg!(&config);
    Ok(())
}

pub fn parse_positive_int(s: &str) -> MyResult<usize> {
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