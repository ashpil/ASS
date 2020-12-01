use ass::parser::ass_parser::ass_parser;
use ass::parser::asml_parser::asml_parser;
use std::fs::read_to_string;


fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    println!("==========================");
    println!("0: Parse Styling Language\n1: Parse Markup Language");
    println!("==========================");
    let choice = get_input().trim().parse::<u32>().unwrap();
    println!("Enter the path to the file:");
    let file_path = get_input();
    let code = read_to_string(file_path.trim()).unwrap().replace('\n', "");
    if choice == 0 {
        println!("{:#?}", ass_parser::stylesheet(&code).unwrap())
    } else {
        println!("{:#?}", asml_parser::tag(&code).unwrap())
    }
}
