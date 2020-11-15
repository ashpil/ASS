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
    attr: HashMap<String, Vec<(Relation, Arith)>>,
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
            = whitespace() r:relation() whitespace() a:arith() { (r, a) }

        rule spec() -> (String, Vec<(Relation, Arith)>)
            = attr:word() c:constraint() ** ", else" { (attr, c) } 

        rule style() -> Style
            = name:word() whitespace()* "{" whitespace()* attr:spec() ** whitespace() whitespace()* "}" { Style { name, attr: attr.into_iter().collect::<HashMap<String, Vec<(Relation, Arith)>>>() } }

        pub rule stylesheet() -> Vec<Style>
            = whitespace()* s:style() ** whitespace() { s }
        
    }
}

