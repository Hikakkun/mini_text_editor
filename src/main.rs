use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn read_file(edit_file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(edit_file_path)?;

    let reader = io::BufReader::new(file);

    let lines = reader.lines().collect::<Result<_, _>>()?;

    Ok(lines)
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn write_to_file(file_path: &str, lines: &Vec<String>) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    lines
        .iter()
        .try_for_each(|line| writeln!(file, "{}", line))?;

    Ok(())
}
fn main() {
    // コマンドライン引数を取得する
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <edit_file_path>", &args[0]);
        std::process::exit(1);
    }

    let edit_file_path = &args[1];

    let mut lines =
        read_file(edit_file_path).expect(&format!("Error reading file: {}", edit_file_path));

    println!("edit_file_path: {}", edit_file_path);
    println!("Contents");
    lines
        .iter()
        .enumerate()
        .for_each(|(index, line)| println!("{:02}|{}", index, line));

    let edit_line_number = input("input edit line number>")
        .trim()
        .parse::<usize>()
        .expect("Invalid input for line number");

    // インデックス1の要素を変更
    if let Some(edit_string) = lines.get_mut(edit_line_number) {
        *edit_string = input(&format!("line{:02}>", edit_line_number).to_string());
    } else {
        println!("line Index out of bounds");
        std::process::exit(1);
    }

    write_to_file(edit_file_path, &lines)
        .expect(&format!("Error writing file: {}", edit_file_path));
    println!("Data has been written to {} successfully.", edit_file_path);
}
