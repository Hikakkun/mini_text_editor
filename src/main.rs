use std::env;
fn main() {
    // コマンドライン引数を取得する
    let args: Vec<String> = env::args().collect();

    if args.len() != 2{
        eprintln!("Usage: {} <edit_file_path>", &args[0]);
        std::process::exit(1);
    }

    let edit_file_path = &args[1];

    println!("edit_file_path: {}", edit_file_path);
}
