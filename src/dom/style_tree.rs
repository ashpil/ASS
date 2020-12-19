use crate::parser::asml_parser::{Element, Trait};
use crate::parser::ass_parser::{Arith, Entity, Relation, Style};
use cassowary::strength::{MEDIUM, REQUIRED, STRONG, WEAK};
use cassowary::{AddConstraintError, Constraint, Solver, Variable, WeightedRelation};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
pub struct StyleGroups<'a> {
    pub constraints: Vec<(&'a String, &'a Vec<(Relation, Arith)>)>,
    pub properties: Vec<(&'a String, &'a Vec<(Relation, Arith)>)>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StyleNode<'a> {
    pub id: usize,
    pub pid: usize,
    pub element: &'a Element,
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
        None => panic!("{} ID Not Mapped in Variable Pool", node_id),
    }
}

fn collate_styles<'a>(
    traits: &'a Vec<Trait>,
    stylesheet: &'a Vec<Style>,
    constraint_names: &'a HashSet<String>,
    property_names: &'a HashSet<String>,
    default_attributes: &'a HashMap<String, Vec<(Relation, Arith)>>,
    id: usize,
    styles_to_id: &mut HashMap<&'a String, usize>,
) -> StyleGroups<'a> {
    let mut constraints = vec![];
    let mut properties = vec![];
    for trait_ in traits.iter() {
        if styles_to_id.contains_key(&trait_.name) {
            panic!("A style with constraints cannot be used with more than one node.")
        }
        styles_to_id.insert(&trait_.name, id);
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
    parent_id: usize,
    id: &mut usize,
    default_attributes: &'a HashMap<String, Vec<(Relation, Arith)>>,
    styles_to_id: &mut HashMap<&'a String, usize>,
    id_to_style_node: &mut HashMap<usize, StyleNode<'a>>,
) -> StyleNode<'a> {
    *id += 1;
    let old_id = *id;
    let new_node = match root {
        Element::Tag { traits, children } =>
        // Loop through traits
        {
            StyleNode {
                id: *id,
                pid: parent_id,
                element: root,
                children: children
                    .iter()
                    .enumerate()
                    .map(|(_, child)| {
                        construct_style_tree(
                            child,
                            stylesheet,
                            constraint_names,
                            property_names,
                            old_id,
                            id,
                            default_attributes,
                            styles_to_id,
                            id_to_style_node,
                        )
                    })
                    .collect(),
                styles: collate_styles(
                    traits,
                    stylesheet,
                    constraint_names,
                    property_names,
                    default_attributes,
                    old_id,
                    styles_to_id,
                ),
            }
        }
        Element::Text(_) => {
            let mut constraints = vec![];
            for attr in default_attributes {
                constraints.push(attr);
            }
            StyleNode {
                id: *id,
                pid: parent_id,
                element: root,
                children: vec![],
                styles: StyleGroups {
                    constraints: constraints,
                    properties: vec![],
                },
            }
        }
    };
    id_to_style_node.insert(new_node.id, new_node.clone());
    return new_node;
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

    for constraint_name in constraint_names {
        if !attr_to_variable.contains_key(constraint_name) {
            attr_to_variable.insert(constraint_name, Variable::new());
        }
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
    styles_to_id: &HashMap<&'a String, usize>,
) {
    let id = root.id;
    let pid = root.pid;

    let mut new_constraints = vec![];

    new_constraints.append(&mut vec![
        retrieve_variable(variable_pool, id, &"left".to_string())
            | WeightedRelation::LE(REQUIRED)
            | retrieve_variable(variable_pool, id, &"right".to_string()),
        retrieve_variable(variable_pool, id, &"top".to_string())
            | WeightedRelation::LE(REQUIRED)
            | retrieve_variable(variable_pool, id, &"bottom".to_string()),
        retrieve_variable(variable_pool, id, &"top".to_string())
            | WeightedRelation::GE(REQUIRED)
            | 0.0,
        retrieve_variable(variable_pool, id, &"left".to_string())
            | WeightedRelation::GE(REQUIRED)
            | 0.0,
    ]);

    if pid != 0 {
        new_constraints.append(&mut vec![
            retrieve_variable(variable_pool, id, &"left".to_string())
                | WeightedRelation::GE(REQUIRED)
                | retrieve_variable(variable_pool, pid, &"left".to_string()),
            retrieve_variable(variable_pool, id, &"right".to_string())
                | WeightedRelation::LE(REQUIRED)
                | retrieve_variable(variable_pool, pid, &"right".to_string()),
            retrieve_variable(variable_pool, id, &"bottom".to_string())
                | WeightedRelation::LE(REQUIRED)
                | retrieve_variable(variable_pool, pid, &"bottom".to_string()),
            retrieve_variable(variable_pool, id, &"top".to_string())
                | WeightedRelation::GE(REQUIRED)
                | retrieve_variable(variable_pool, pid, &"top".to_string()),
        ])
    }
    match root.element {
        Element::Text(_) => new_constraints.append(&mut vec![
            retrieve_variable(variable_pool, id, &"left".to_string())
                | WeightedRelation::EQ(REQUIRED)
                | retrieve_variable(variable_pool, pid, &"left".to_string()),
            retrieve_variable(variable_pool, id, &"right".to_string())
                | WeightedRelation::EQ(REQUIRED)
                | retrieve_variable(variable_pool, pid, &"right".to_string()),
            retrieve_variable(variable_pool, id, &"bottom".to_string())
                | WeightedRelation::EQ(REQUIRED)
                | retrieve_variable(variable_pool, pid, &"bottom".to_string()),
            retrieve_variable(variable_pool, id, &"top".to_string())
                | WeightedRelation::EQ(REQUIRED)
                | retrieve_variable(variable_pool, pid, &"top".to_string()),
        ]),
        _ => (),
    }

    for (attr_name, terms) in &root.styles.constraints {
        for (rel, arith) in *terms {
            let left_hand_variable = retrieve_variable(variable_pool, id, attr_name);
            let constraint_operator = relation_to_operator(&rel);
            match arith {
                Arith::Num(n) => {
                    if *attr_name == "width" {
                        new_constraints.push(
                            retrieve_variable(variable_pool, id, &"right".to_string())
                                - retrieve_variable(variable_pool, id, &"left".to_string())
                                | WeightedRelation::EQ(REQUIRED)
                                | *n as f64,
                        );
                    } else if *attr_name == "height" {
                        new_constraints.push(
                            retrieve_variable(variable_pool, id, &"bottom".to_string())
                                - retrieve_variable(variable_pool, id, &"top".to_string())
                                | WeightedRelation::EQ(REQUIRED)
                                | *n as f64,
                        );
                    } else {
                        new_constraints.push(left_hand_variable | constraint_operator | *n as f64)
                    }
                }
                Arith::Ref(e, other_attr_name) => match e {
                    Entity::Other(other_style) => {
                        let target_id = *styles_to_id.get(&other_style).unwrap();
                        if other_attr_name == "height" {
                            new_constraints.push(
                                retrieve_variable(variable_pool, id, &"bottom".to_string())
                                    - retrieve_variable(variable_pool, id, &"top".to_string())
                                    | WeightedRelation::EQ(STRONG)
                                    | retrieve_variable(
                                        variable_pool,
                                        target_id,
                                        &"bottom".to_string(),
                                    ) - retrieve_variable(
                                        variable_pool,
                                        target_id,
                                        &"top".to_string(),
                                    ),
                            )
                        } else if other_attr_name == "width" {
                            new_constraints.push(
                                retrieve_variable(variable_pool, id, &"right".to_string())
                                    - retrieve_variable(variable_pool, id, &"left".to_string())
                                    | WeightedRelation::EQ(STRONG)
                                    | retrieve_variable(
                                        variable_pool,
                                        target_id,
                                        &"right".to_string(),
                                    ) - retrieve_variable(
                                        variable_pool,
                                        target_id,
                                        &"left".to_string(),
                                    ),
                            )
                        } else {
                            new_constraints.push(
                                left_hand_variable
                                    | constraint_operator
                                    | retrieve_variable(variable_pool, target_id, &other_attr_name),
                            );
                        }
                    }
                    _ => panic!("STRANGE ENTITY"),
                },
                _ => panic!("Invalid Expression"),
            }
        }
    }

    match solver.add_constraints(&new_constraints) {
        Ok(_) => println!("Constraint Added"),
        Err(e) => match e {
            AddConstraintError::DuplicateConstraint => println!("Duplicate Constraint"),
            AddConstraintError::UnsatisfiableConstraint => println!("Unsatisfiable Constraint"),
            AddConstraintError::InternalSolverError(s) => println!("{}", s),
        },
    }

    for child in &root.children {
        solve_constraints(child, variable_pool, solver, styles_to_id);
    }
}
