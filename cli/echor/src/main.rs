use clap::{arg, Command};

fn main() {
    let mathces = Command::new("echor")
        .version("0.1.0")
        .author("novumd <novumd@gmail.com>")
        .about("Rust echo")
        .args([
            arg!(-t --text <TEXT> ... "Input text")
                .required(true)
                .num_args(1..),
            arg!(-n --omit_newline "Do not print newline").num_args(0),
        ])
        .get_matches();
    println!("{:#?}", mathces);
}
