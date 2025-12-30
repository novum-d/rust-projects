use clap::{Arg, Args, Command};
use std::env::args;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

const COMMAND: &str = "wcr";
const FILES: &str = "files";
const BYTES: &str = "bytes";
const CHARS: &str = "chars";
const LINES: &str = "lines";
const WORDS: &str = "words";
// -c, --bytes            バイト数を表示する
// -m, --chars            文字数を表示する
// -l, --lines            改行の数を表示する
// --files0-from=F    入力として NULL 文字で区切られたファイル F を使用

#[derive(Debug, PartialEq)]
pub struct Config {
    files: Vec<String>,
    bytes: bool,
    chars: bool,
    lines: bool,
    words: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("wcr")
        .args([
            Arg::new(FILES).num_args(1..).default_value("-"),
            Arg::new(BYTES).short('c').long(BYTES),
            Arg::new(CHARS).short('m').long(CHARS),
            Arg::new(LINES).short('l').long(LINES),
            Arg::new(WORDS).short('w').long(WORDS),
        ])
        .get_matches();

    let files = if let Some(v) = matches.get_many::<String>(FILES) {
        v.cloned().collect::<Vec<_>>()
    } else {
        return Err(From::from("Failed to get files"));
    };

    let config = Config {
        files,
        bytes: matches.get_one::<String>(BYTES).is_none(),
        chars: matches.get_one::<String>(CHARS).is_none(),
        lines: matches.get_one::<String>(LINES).is_none(),
        words: matches.get_one::<String>(WORDS).is_none(),
    };

    Ok(config)
}
