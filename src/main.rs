use ass::ass_parser::ass_parser;

fn main() {
    println!("{:#?}", ass_parser::stylesheet("test { width = lmao[test], else = 50\nlength <= $window[test] + 5 }\nagain { property = 40, else = $parent[height] }"))
}
