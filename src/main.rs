use ass::asml_parser::asml_parser;

fn main() {
    println!("{:#?}", asml_parser::tag("<tag1><tag2><#A comment lol#></tag2></tag1>"))
}
