use ass::asml_parser::asml_parser;

fn main() {
    println!("{:#?}", asml_parser::tag("<some tag with(traits)><howdy>how neat is this!<lmao/></howdy>words</some>"))
}
