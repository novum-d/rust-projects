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

    let values_ref = mathces.get_many::<String>("text").unwrap();
    let string_vec = values_ref.cloned().collect::<Vec<_>>();
    let text = string_vec.join("");

    let omit_newline = mathces.contains_id("omit_newline ");
    let ending = if omit_newline { "" } else { "\n" };

    print!("{}{}", text, ending);
}
