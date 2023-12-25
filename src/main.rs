use std::fs::File;
use std::io::{self, BufRead, Write};
use std::env;

fn read_file(edit_file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(edit_file_path)?;    

    let reader = io::BufReader::new(file);

    let lines = reader.lines().collect::<Result<_, _>>()?;

    return Ok(lines)
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
fn main() {
    // コマンドライン引数を取得する
    let args: Vec<String> = env::args().collect();

    if args.len() != 2{
        eprintln!("Usage: {} <edit_file_path>", &args[0]);
        std::process::exit(1);
    }

    let edit_file_path = &args[1];

    println!("edit_file_path: {}", edit_file_path);

    let lines = match read_file(edit_file_path) {
        Ok(lines) => lines,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            std::process::exit(1);
        }
    };

    println!("Contents");
    lines.iter().enumerate().for_each(|(index, line)|{
        println!("{:02}|{}", index, line)
    });

    let edit_line_number = match input("input edit line number>").trim().parse::<i32>() {
        Ok(number) => number,
        Err(_) => {
            eprintln!("Invalid input for line number");
            std::process::exit(1);
        }        
    };
}
