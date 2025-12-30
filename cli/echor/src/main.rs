use clap::{arg, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("novumd <novumd@gmail.com>")
        .about("Rust echo")
        .args([
            arg!(text: <TEXT> ... "Input text")
                .required(true)
                .num_args(1..),
            arg!(-n --"omit-newline" "Do not print newline").num_args(0),
        ])
        .get_matches();

    let text = matches
        .get_many::<String>("text")
        .unwrap()
        .cloned()
        .collect::<Vec<_>>();

    let omit_newline = matches.get_flag("omit-newline");
    let ending = if omit_newline { "" } else { "\n" };

    print!("{}{}", text.join(" "), ending);
}
