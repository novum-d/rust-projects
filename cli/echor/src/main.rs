use clap::{arg, Command};

fn main() {
    let mathces = Command::new("echor")
        .version("0.1.0")
        .author("novumd <novumd@gmail.com>")
        .about("Rust echo")
        .args([
            arg!(text: <TEXT> ... "Input text")
                .required(true)
                .num_args(1..),
            arg!(omit_newline: -n --omit_newline "Do not print newline").num_args(0),
        ])
        .get_matches();

    let text = mathces
        .get_many::<String>("text")
        .unwrap()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let omit_newline = mathces.get_flag("omit_newline");
    let ending = if omit_newline { "" } else { "\n" };
    print!("{}{}", text.join(" "), ending);
}
