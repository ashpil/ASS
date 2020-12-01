use ass::parser::parser;
use std::fs::read_to_string;


fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed to read input");
    buffer
}

fn main() {
    println!("Enter the path to the file:");
    let file_path = get_input();
    let code = read_to_string(file_path.trim()).expect("Failed to find file");
    println!("{:#?}", parser(&code).expect("Failed to parse file"))
}

