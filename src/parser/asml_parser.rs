use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq)]
pub struct Trait {
    pub name: String,
    pub args: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum Element {
    Text(String),
    Tag {
        traits: Vec<Trait>,
        children: Vec<Element>,
    },
}

peg::parser! {
    pub grammar asml_parser() for str {
        rule comment() = "<#" (!"#>"[c])* "#>"

        rule whitespace() = quiet!{([c if c.is_whitespace()]+ / comment())+ }

        rule word() -> String
            = s:$(['a'..='z' | '_' | '0'..='9']+) { s.to_string() }

        rule word_extra() -> String
            = s:$([c if !matches!(c, '<' | '>' | '(' | ')') && !c.is_whitespace() ]+) { s.to_string() }

        rule paragraph() -> String
            = s:$([c if c != '<']+) { s.trim().to_string() }

        rule args() -> Vec<String>
            = "(" whitespace()* args:word_extra() ** whitespace() whitespace()* ")" { args }

        rule traits() -> Trait
            = name:word() args:args()? { Trait { name, args: args.unwrap_or_default() } }

        pub rule tag() -> Element
            = s:paragraph() { Element::Text(s) }
            / whitespace()* "<" whitespace()* traits:traits() ** whitespace() whitespace()* "/>" whitespace()* { Element::Tag {traits, children: Vec::new() }}
            / whitespace()* "<" whitespace()* traits:traits() ** whitespace() whitespace()* ">"  whitespace()* children:tag()* "</" close:word() whitespace()* ">" whitespace()* {?
                if traits.get(0).map_or(false, |x| x.name == close) {
                    Ok(Element::Tag { traits, children })
                } else {
                    Err("didn't find closing trait")
                }
            }

    }
}

#[cfg(test)]
mod asml_parser_tests {
    use super::*;

    fn h1_element() -> Result<Element, peg::error::ParseError<peg::str::LineCol>> {
        return Ok(Element::Tag {
            traits: vec![Trait {
                name: "h1".to_string(),
                args: Vec::new(),
            }],
            children: Vec::new(),
        });
    }

    #[test]
    fn test_one_trait() {
        assert_eq!(asml_parser::tag("<h1></h1>"), h1_element(),);
    }

    #[test]
    fn test_nested_tags() {
        assert_eq!(
            asml_parser::tag("<tag1><tag2></tag2></tag1>"),
            Ok(Element::Tag {
                traits: vec![Trait {
                    name: String::from("tag1"),
                    args: vec![],
                },],
                children: vec![Element::Tag {
                    traits: vec![Trait {
                        name: String::from("tag2"),
                        args: vec![],
                    },],
                    children: vec![],
                },],
            },),
        );
    }

    #[test]
    fn test_text_in_tags() {
        assert_eq!(
            asml_parser::tag("<h1>god</h1>"),
            Ok(Element::Tag {
                traits: vec![Trait {
                    name: "h1".to_string(),
                    args: Vec::new(),
                }],
                children: vec![Element::Text("god".to_string())],
            }),
        );
    }

    #[test]
    fn test_varied_children_in_tags() {
        assert_eq!(
            asml_parser::tag("<h1>god<h1></h1></h1>"),
            Ok(Element::Tag {
                traits: vec![Trait {
                    name: "h1".to_string(),
                    args: Vec::new(),
                }],
                children: vec![Element::Text("god".to_string()), h1_element().unwrap()],
            }),
        );
    }

    #[test]
    fn test_comment() {
        assert_eq!(asml_parser::tag("<h1><# god #></h1>"), h1_element())
    }

    #[test]
    fn test_empty_comment() {
        assert_eq!(
            asml_parser::tag(
                "<# #>
        <h1<##>>
            <##>
        </h1<##>>
        <##>"
            ),
            h1_element()
        )
    }

    #[test]
    fn test_commented_tag() {
        assert_eq!(asml_parser::tag("<h1><# <h1></h1> #></h1>"), h1_element())
    }

    #[test]
    fn test_single_tag() {
        assert_eq!(asml_parser::tag("<h1/>"), h1_element())
    }

    #[test]
    fn test_one_args() {
        assert_eq!(
            asml_parser::tag("<h1(god)/>"),
            Ok(Element::Tag {
                traits: vec![Trait {
                    name: "h1".to_string(),
                    args: vec!["god".to_string()],
                }],
                children: Vec::new(),
            })
        )
    }

    #[test]
    fn test_multi_args() {
        assert_eq!(
            asml_parser::tag("<h1(god o)/>"),
            Ok(Element::Tag {
                traits: vec![Trait {
                    name: "h1".to_string(),
                    args: vec!["god".to_string(), "o".to_string()],
                }],
                children: Vec::new(),
            })
        )
    }

    #[test]
    fn test_multi_args_with_comments() {
        assert_eq!(
            asml_parser::tag("<h1(<# #> god <# god #> o <# #>)/>"),
            Ok(Element::Tag {
                traits: vec![Trait {
                    name: "h1".to_string(),
                    args: vec!["god".to_string(), "o".to_string()],
                }],
                children: Vec::new(),
            })
        )
    }
    #[test]
    fn test_multi_args_with_comments_and_text() {
        assert_eq!(
            asml_parser::tag("<h1(god <# god #> o) <# #>>b3%!<h1(god)>()</h1>^423$%33(4)232</h1>"),
            Ok(Element::Tag {
                traits: vec![Trait {
                    name: "h1".to_string(),
                    args: vec!["god".to_string(), "o".to_string()],
                }],
                children: vec![
                    Element::Text("b3%!".to_string()),
                    Element::Tag {
                        traits: vec![Trait {
                            name: "h1".to_string(),
                            args: vec!["god".to_string()],
                        }],
                        children: vec![Element::Text("()".to_string())],
                    },
                    Element::Text("^423$%33(4)232".to_string())
                ],
            })
        )
    }

    #[test]
    #[should_panic]
    fn missing_single_start_tag_fail() {
        assert_eq!(
            asml_parser::tag("<h1(god)>"),
            Ok(Element::Tag {
                traits: vec![Trait {
                    name: "h1".to_string(),
                    args: vec!["god".to_string()],
                }],
                children: Vec::new(),
            })
        )
    }

    #[test]
    #[should_panic]
    fn missing_single_end_tag_fail() {
        assert_eq!(
            asml_parser::tag("</h1(god)>"),
            Ok(Element::Tag {
                traits: vec![Trait {
                    name: "h1".to_string(),
                    args: vec!["god".to_string()],
                }],
                children: Vec::new(),
            })
        )
    }
    // TODO: HTML does not worry about this, should we be as stringent?
    #[test]
    #[should_panic]
    fn incorrect_tag_order_fail() {
        assert_eq!(
            asml_parser::tag("<h1(god)><a></h1></a>"),
            Ok(Element::Tag {
                traits: vec![Trait {
                    name: "h1".to_string(),
                    args: vec!["god".to_string()],
                }],
                children: Vec::new(),
            })
        )
    }
    // #[test]
    // fn multiple_top_level_tags() {
    //     assert_eq!(
    //         asml_parser::tag("<h1></h1><h1></h1>"),
    //         Ok(Element::Tag {
    //             traints: vec![Trait {
    //                 name: "h1".to_string()
    //             }]
    //         })
    //     )
    // }
}
