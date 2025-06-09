use clap::{Arg, Command};
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Read};
use std::str::from_utf8;
use clap::error::ErrorKind::InvalidValue;
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
    let mut cmd = Command::new("headr");
    let matches = cmd
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
        .unwrap_or_else(|e| {
            let err_msg = format!(
                "invalid value '{}' for \
                '--lines <LINES>': invalid digit found in string",
                matches.get_one::<String>(LINES).unwrap(),
            );
            eprintln!("{}", clap::Error::raw(InvalidValue, err_msg));
            e.exit();
        })
        .unwrap();

    let bytes = matches.get_one::<String>(BYTES)
        .map(|s| parse_positive_int(s.as_str()))
        .transpose()
        .unwrap_or_else(|e| {
            let err_msg = format!(
                "invalid value '{}' for \
                '--bytes <BYTES>': invalid digit found in string",
                matches.get_one::<String>(BYTES).unwrap(),
            );
            eprintln!("{}", clap::Error::raw(InvalidValue, err_msg));
            e.exit();
        });

    Ok(Config {
        files,
        lines,
        bytes,
    })
}


pub fn run(config: Config) -> MyResult<()> {
    let files_len = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Err(e) => {
                let io_err = e.downcast_ref::<io::Error>();
                let err = if let Some(io_err) = io_err {
                    let io_err = match io_err.kind() {
                        ErrorKind::NotFound => io_err,
                        ErrorKind::PermissionDenied => io_err,
                        _ => io_err
                    };
                    Box::new(io_err)
                } else {
                    e
                };
                eprintln!("{}: {}", filename, err);
            }
            Ok(mut input) => {
                if files_len > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename,
                    );
                }
                let output: String = match config {
                    Config { bytes: Some(bytes_count), .. } => {
                        // utf-8文字列が途中で、切れてしまう（8で割り切れない）場合は切り詰める
                        let bytes = input.bytes().take(bytes_count).collect::<Result<Vec<_>, _>>();
                        String::from_utf8_lossy(&bytes?).into()
                        // let mut buffer: Vec<u8> = vec![0; bytes_count];
                        // let bytes_end_index = input.read(&mut buffer)?;
                        // let bytes_end_index = match from_utf8(&buffer[..bytes_end_index]) {
                        //     Ok(_) => bytes_end_index,
                        //     Err(e) => e.valid_up_to()
                        // };
                        // String::from_utf8_lossy(&buffer[..bytes_end_index]).into()
                    }
                    Config { lines, .. } if lines > 0 => {
                        let mut line = String::from("");
                        // for _ in input.clone().lines().take(config.lines) {
                        for _ in 0..config.lines {
                            let bytes = input.read_line(&mut line)?;
                            if bytes == 0 {
                                break;
                            }
                        }
                        line
                    }
                    _ => { "".to_string() }
                };
                print!("{}", output)
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}


pub fn parse_positive_int(s: &str) -> Result<usize, clap::error::Error> {
    // unimplemented!() // 未実装であることを示すプレースホルダ
    match s.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(clap::error::Error::new(InvalidValue)),
    }
}
