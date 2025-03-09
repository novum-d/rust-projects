use std::process::exit;

fn main() {
    if let Err(e) = catr::get_args().and_then(catr::run) {
        eprintln!("{}", e);
        exit(1)
    }
}
