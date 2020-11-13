#[derive(Debug)]
pub struct Trait {
    name: String,
    args: Vec<String>,
}

#[derive(Debug)]
pub enum Element {
    Text(String),
    Tag { traits: Vec<Trait>, children: Vec<Element> },
}

peg::parser!{
    pub grammar asml_parser() for str {
        rule whitespace() = quiet!{[c if c.is_whitespace()]+}

        rule word() -> String
            = s:$(['a'..='z' | '_' | '0'..='9']+) { s.to_string() }
        
        rule word_extra() -> String
            = s:$([c if !matches!(c, '<' | '>' | '(' | ')') && !c.is_whitespace() ]+) { s.to_string() }
        
        rule paragraph() -> String
            = s:$([c if c != '<']+) { s.trim().to_string() }

        rule args() -> Vec<String>
            = "(" args:word_extra() ** whitespace() ")" { args } 
        
        rule traits() -> Trait
            = name:word() args:args()? { Trait { name, args: args.unwrap_or_default() } }

        pub rule tag() -> Element
            = "<" traits:traits() ** whitespace() "/>" whitespace()* { Element::Tag {traits, children: Vec::new() }}
            / "<" traits:traits() ** whitespace() ">"  whitespace()* children:tag()* "</" close:word() ">" whitespace()* {?
                if traits.get(0).map_or(false, |x| x.name == close) {
                    Ok(Element::Tag { traits, children })
                } else {
                    Err("didn't find closing trait")
                }
            }
            / s:paragraph() { Element::Text(s) } 

    }
}