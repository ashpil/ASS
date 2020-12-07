use crate::parser::asml_parser::{asml_parser::tag, Element};
use crate::parser::ass_parser::{ass_parser::stylesheet, Style};

peg::parser! {
    pub grammar parser() for str {
        rule comment() = "<#" (!"#>"[c])* "#>"

        rule whitespace() = quiet!{([c if c.is_whitespace()]+ / comment())+ }

        rule parse_styles() -> Vec<Style>
            = "<style>" s:$([c if c != '<']+) "</style>" { stylesheet(s.trim()).expect("bad style") }

        rule parse_body() -> Element
            = "<body>" b:$([c]+) { tag(&(String::from("<body>") + b)).expect("bad body") }

        pub rule parser() -> (Element, Vec<Style>)
            = whitespace()* styles:parse_styles()? whitespace()* body:parse_body() whitespace()* { (body, styles.unwrap()) }
    }
}
