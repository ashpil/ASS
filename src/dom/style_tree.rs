use crate::parser::asml_parser::{Element, Trait};
use crate::parser::ass_parser::{Arith, Entity, Relation, Style};
use cassowary::strength::{MEDIUM, REQUIRED, STRONG, WEAK};
use cassowary::{AddConstraintError, Constraint, Solver, Variable, WeightedRelation};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct StyleGroups<'a> {
    pub constraints: Vec<(&'a String, &'a Vec<(Relation, Arith)>)>,
    pub properties: Vec<(&'a String, &'a Vec<(Relation, Arith)>)>,
}

#[derive(Debug, PartialEq)]
pub struct StyleNode<'a> {
    pub id: usize,
    element: &'a Element,
    pub children: Vec<StyleNode<'a>>,
    pub styles: StyleGroups<'a>,
}

fn relation_to_operator(rel: &Relation) -> WeightedRelation {
    match rel {
        Relation::EQ => WeightedRelation::EQ(REQUIRED),
        Relation::GE => WeightedRelation::GE(REQUIRED),
        Relation::LE => WeightedRelation::LE(REQUIRED),
    }
}

pub fn retrieve_variable(
    variable_pool: &mut HashMap<usize, HashMap<&String, Variable>>,
    node_id: usize,
    attr_name: &String,
) -> Variable {
    match variable_pool.get(&node_id) {
        Some(attr_to_variable) => match attr_to_variable.get(attr_name) {
            Some(var) => *var,
            None => panic!("Attribute Name {} Not Mapped in Variable Pool", attr_name),
        },
        None => panic!("Style Name Not Mapped in Variable Pool"),
    }
}

fn collate_styles<'a>(
    traits: &Vec<Trait>,
    stylesheet: &'a Vec<Style>,
    constraint_names: &'a HashSet<String>,
    property_names: &'a HashSet<String>,
    default_attributes: &'a HashMap<String, Vec<(Relation, Arith)>>,
) -> StyleGroups<'a> {
    let mut constraints = vec![];
    let mut properties = vec![];
    for trait_ in traits.iter() {
        for style in stylesheet.iter() {
            if trait_.name == style.name {
                for attr in &style.attrs {
                    if constraint_names.contains::<str>(&attr.0) {
                        constraints.push(attr);
                    } else if property_names.contains::<str>(&attr.0) {
                        properties.push(attr);
                    }
                }
                for attr in default_attributes {
                    constraints.push(attr);
                }
            }
        }
    }
    return StyleGroups {
        constraints,
        properties,
    };
}

pub fn construct_style_tree<'a>(
    root: &'a Element,
    stylesheet: &'a Vec<Style>,
    constraint_names: &'a HashSet<String>,
    property_names: &'a HashSet<String>,
    id: usize,
    default_attributes: &'a HashMap<String, Vec<(Relation, Arith)>>,
) -> StyleNode<'a> {
    match root {
        Element::Tag { traits, children } => {
            // Loop through traits
            return StyleNode {
                id: id,
                element: root,
                children: children
                    .iter()
                    .enumerate()
                    .map(|(i, child)| {
                        construct_style_tree(
                            child,
                            stylesheet,
                            constraint_names,
                            property_names,
                            id + 1 + i,
                            default_attributes,
                        )
                    })
                    .collect(),
                styles: collate_styles(
                    traits,
                    stylesheet,
                    constraint_names,
                    property_names,
                    default_attributes,
                ),
            };
        }
        Element::Text(_) => StyleNode {
            id: id + 1,
            element: root,
            children: vec![],
            styles: StyleGroups {
                constraints: vec![],
                properties: vec![],
            },
        },
    }
}

pub fn generate_variable_pool<'a>(
    root: &'a StyleNode,
    stylesheet: &'a Vec<Style>,
    constraint_names: &'a HashSet<String>,
    variable_pool: &mut HashMap<usize, HashMap<&'a String, Variable>>,
) {
    let id = root.id;
    let mut attr_to_variable = HashMap::new();
    for (attr_name, _) in &root.styles.constraints {
        attr_to_variable.insert(*attr_name, Variable::new());
    }
    variable_pool.insert(id, attr_to_variable);
    for child in &root.children {
        generate_variable_pool(child, stylesheet, constraint_names, variable_pool);
    }
}

pub fn solve_constraints<'a>(
    root: &'a StyleNode,
    variable_pool: &mut HashMap<usize, HashMap<&String, Variable>>,
    solver: &mut Solver,
) {
    let id = root.id;
    for (attr_name, terms) in &root.styles.constraints {
        for (rel, arith) in *terms {
            let left_hand_variable = retrieve_variable(variable_pool, id, attr_name);
            let constraint_operator = relation_to_operator(rel);
            let new_constraint = match arith {
                Arith::Num(n) => left_hand_variable | constraint_operator | *n as f64,
                // Arith::Ref(entity, other_attr) => {
                //     left_hand_variable
                //         | constraint_operator
                //         | match entity {
                //             Entity::Other(other_style) => {
                //                 retrieve_variable(variable_pool, id, &other_attr)
                //             }
                //             _ => panic!("STRANGE ENTITY"),
                //         }
                // }
                _ => panic!("Invalid Expression"),
            };
            println!("h{:#?}", new_constraint);
            match solver.add_constraint(new_constraint) {
                Ok(_) => println!("Constraint Added"),
                Err(e) => match e {
                    AddConstraintError::DuplicateConstraint => println!("Duplicate Constraint"),
                    AddConstraintError::UnsatisfiableConstraint => {
                        println!("Unsatisfiable Constraint")
                    }
                    AddConstraintError::InternalSolverError(s) => println!("{}", s),
                },
            }
        }
    }
    for child in &root.children {
        solve_constraints(child, variable_pool, solver);
    }
}
