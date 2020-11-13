#[cfg(test)]
mod asml_parser_tests {
    use ass::asml_parser::asml_parser;
    use ass::asml_parser::Element;
    use ass::asml_parser::Trait;
    use std::fs::read_to_string;

    fn gen_test(file: &str, expected_asml: Element) {
        let mut test_file_dir: String = "tests/asml_examples/".to_owned();
        test_file_dir.push_str(file);
        let asml = read_to_string(test_file_dir).unwrap();
        let parsed_asml = asml_parser::tag(&asml).unwrap();
        assert_eq!(parsed_asml, expected_asml);
    }

    #[test]
    fn test_one_trait() {
        gen_test(
            "asml_0",
            Element::Tag {
                traits: vec![Trait {
                    name: "h1".to_string(),
                    args: Vec::new(),
                }],
                children: Vec::new(),
            },
        );
    }

    #[test]
    fn test_one_trait_style() {
        gen_test(
            "asml_0",
            Element::Tag {
                traits: vec![Trait {
                    name: "h1".to_string(),
                    args: Vec::new(),
                }],
                children: ,
            },
        );
    }
}
