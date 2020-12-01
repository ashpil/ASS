use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Entity {
    Parent,
    Window,
    Other(String),
}

#[derive(Debug, PartialEq)]
pub enum Relation {
    GE,
    EQ,
    LE,
}

#[derive(Debug, PartialEq)]
pub enum Arith {
    Ref(Entity, String),
    Num(u32),
    Add(Box<Arith>, Box<Arith>),
    Sub(Box<Arith>, Box<Arith>),
}

#[derive(Debug, PartialEq)]
pub struct Style {
    name: String,
    attrs: HashMap<String, Vec<(Relation, Arith)>>,
}

peg::parser! {
    pub grammar ass_parser() for str {
        use self::Arith::*;
        use self::Relation::*;

        rule whitespace() = quiet!{ [c if c.is_whitespace()]+ }

        rule word() -> String
            = s:$(['a'..='z' | '_' | '0'..='9']+) { s.to_string() }

        rule number() -> u32
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule relation() -> Relation
            = "=" { EQ } / "<=" { LE } / ">=" { GE }

        rule entity() -> Entity
            = "$parent" { Entity::Parent }
            / "$window" { Entity::Window }
            / w:word() { Entity::Other(w) }

        rule attribute() -> Arith
            = e:entity() "[" w:word() "]" { Ref(e, w) }

        rule attr_or_num() -> Arith
            = attribute() / n:number() { Num(n) }

        rule arith() -> Arith
            = p1:attribute() whitespace()* "-" whitespace()* p2:attr_or_num() { Sub(Box::new(p1), Box::new(p2)) }
            / p1:attribute() whitespace()* "+" whitespace()* p2:attr_or_num() { Add(Box::new(p1), Box::new(p2)) }
            / p:attribute() whitespace()* "-" whitespace()* a:arith() { Sub(Box::new(p), Box::new(a)) }
            / p:attribute() whitespace()* "+" whitespace()* a:arith() { Sub(Box::new(p), Box::new(a)) }
            / attr_or_num()

        rule constraint() -> (Relation, Arith)
            = whitespace()* r:relation() whitespace()* a:arith() { (r, a) }

        rule spec() -> (String, Vec<(Relation, Arith)>)
            = attr:word() c:constraint() ** ", else" { (attr, c) }

        rule style() -> Style
            = name:word() whitespace()* "{" whitespace()* attr:spec() ** whitespace() whitespace()* "}" { Style { name, attrs: attr.into_iter().collect::<HashMap<String, Vec<(Relation, Arith)>>>() } }

        pub rule stylesheet() -> Vec<Style>
            = whitespace()* s:style() ** (whitespace()*) { s }
    }
}

#[cfg(test)]
mod ass_parser_tests {
    use super::*;

    macro_rules! hashmap {
        ($( $key: expr => $val: expr ),*) => {{
             let mut map = ::std::collections::HashMap::new();
             $( map.insert($key, $val); )*
             map
        }}
    }

    #[test]
    fn basic_style_attr() {
        let expected = Ok(vec![Style {
            name: "div".to_string(),
            attrs: hashmap!["width".to_string() => vec![(Relation::EQ, Arith::Num(32))]],
        }]);
        let output = ass_parser::stylesheet("div{width= 32}");
        assert_eq!(output, expected);
    }

    #[test]
    fn empty_style() {
        let expected = Ok(vec![Style {
            name: "div".to_string(),
            attrs: HashMap::new(),
        }]);
        let output = ass_parser::stylesheet("div{}");
        assert_eq!(output, expected);
    }

    #[test]
    fn two_empty_style() {
        let expected = Ok(vec![
            Style {
                name: "div".to_string(),
                attrs: HashMap::new(),
            },
            Style {
                name: "div2".to_string(),
                attrs: HashMap::new(),
            },
        ]);

        let output = ass_parser::stylesheet("div{}div2{}");
        assert_eq!(output, expected);
    }

    #[test]
    fn entity_attr() {
        let expected = Ok(vec![Style {
            name: "div".to_string(),
            attrs: hashmap!["width".to_string() => vec![(Relation::EQ, Arith::Ref(Entity::Parent, "width".to_string()))]],
        }]);
        let output = ass_parser::stylesheet("div{width=$parent[width]}");
        assert_eq!(output, expected);
    }

    #[test]
    fn multiple_attr() {
        let expected = Ok(vec![Style {
            name: "div".to_string(),
            attrs: hashmap!["width".to_string() => vec![
                (Relation::EQ, Arith::Ref(Entity::Parent, "width".to_string())
            )

            ], "height".to_string() => vec![
                (Relation::GE, Arith::Ref(Entity::Other("hello".to_string()), "world".to_string()))
            ]
            ],
        }]);

        let output = ass_parser::stylesheet(
            "div{
            width=$parent[width] 
            height>=hello[world]
        }",
        );
        assert_eq!(output, expected);
    }

    #[test]
    fn multiple_style_and_attr() {
        let expected = Ok(vec![
            Style {
                name: "div".to_string(),
                attrs: hashmap!["width".to_string() => vec![
                    (Relation::EQ, Arith::Ref(Entity::Parent, "width".to_string())
                )

                ], "height".to_string() => vec![
                    (Relation::GE, Arith::Ref(Entity::Other("hello".to_string()), "world".to_string()))
                ]
                ],
            },
            Style {
                name: "god".to_string(),
                attrs: hashmap!["strength".to_string() => vec![
                    (Relation::LE, Arith::Num(30))
                ]],
            },
        ]);

        let output = ass_parser::stylesheet(
            "div {
                width=$parent[width] 
                height>=hello[world]
            } god {
                strength<=30    
            }",
        );
        assert_eq!(output, expected);
    }
}
