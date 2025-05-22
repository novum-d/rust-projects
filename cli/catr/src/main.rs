use std::process::exit;

fn main() {
    if let Err(e) = catr::get_args().and_then(catr::run) {
        eprintln!("{}", e); // 標準エラー出力にエラーメッセージを表示
        exit(1)
    }
}
