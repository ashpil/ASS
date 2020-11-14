pub enum Property {
    Top,
    Bottom,
    Left, 
    Right,
    Center,
    VCenter,
    HCenter,
}

pub enum Entity {
    Parent,
    Window,
    Any(String),
}

pub enum Value {
    Ref (Entity, Property),
    Num (f32),
}

pub enum Relation {
    GE,
    EQ,
    LE,
}

pub struct Constraint {
    require: Option<(Relation, Value)>,
    want: Option<(Relation, Value)>,
    prefer: Option<(Relation, Value)>,
    r#try: Option<(Relation, Value)>,
}

pub struct Style {
    top: Constraint,
    bottom: Constraint,
    left: Constraint,
    right: Constraint,
    center: Constraint,
    color: u32,
}

/*
TODO: Some of this is done, but I think I still want to rework some of the data structures above
peg::parser! {
    pub grammar ass_parser() for str {
        rule word() -> String
            = s:$(['a'..='z' | '_' | '0'..='9']+) { s.to_string() }

        rule relation() -> Relation
            = " " { Relation::EQ } / " <= " { Relation::LE } / " >= " { Relation::GE }

        rule entity() -> Entity
            = "$parent" { Entity::Parent } / "$window" / { Entity::Window } / w:word() { Entity::AnyPw) }

        rule constraints() ->
            "require" r:relation() t:entity()

        rule rule() -> Constriant 
            = s:word() ":" 

        rule style() -> Style
            = w:word() "{" "}" 
        
    }
}

*/
