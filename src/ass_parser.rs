pub enum Property {
    Top,
    Bottom,
    Left, 
    Right,
    Center,
}

pub enum Value {
    Ref (String, Property),
    Num (float),
}

pub enum Constraint {
    EQ,
    LE,
    LT,
    GE,
    GT,
}

pub struct Style {
    top: Option<(Constriant, Value)>,
    bottom: Option<(Constriant, Value)>,
    left: Option<(Constriant, Value)>,
    right: Option<(Constriant, Value)>,
    center: Option<(Constriant, Value)>,
    color: u32,
}

peg::parser! {
    pub grammar ass_parser() for str {
        
    }
}