use ass::ass_parser::ass_parser;

fn main() {
    println!("{:#?}", ass_parser::stylesheet("
    test {
        width = lmao[test], else = 50
        length <= $window[test] + 5
    }
    again {
        property = 40, else = $parent[height]
    }"))
}
