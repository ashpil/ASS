use crate::parser::asml_parser::{Element, asml_parser::tag};
use crate::parser::ass_parser::{Style, ass_parser::stylesheet};

peg::parser! {
    pub grammar parser() for str {
        rule comment() = "<#" (!"#>"[c])* "#>"

        rule whitespace() = quiet!{([c if c.is_whitespace()]+ / comment())+ }

        rule parse_styles() -> Vec<Style> 
            = "<style>" s:$([c if c != '<']+) "</style>" { stylesheet(s.trim()).expect("bad style") }

        rule parse_body() -> Element
            = "<body>" b:$([c]+) { tag(&(String::from("<body>") + b)).expect("bad body") }

        pub rule parser() -> (Element, Option<Vec<Style>>)
            = whitespace()* styles:parse_styles()? whitespace()* body:parse_body() whitespace()* { (body, styles) }
    }
}

