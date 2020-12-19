use super::style_tree::{retrieve_variable, StyleNode};
use crate::parser::asml_parser::Element;
use crate::parser::ass_parser::{Arith, Entity, Relation, Style};
use cassowary::{AddConstraintError, Constraint, Solver, Variable, WeightedRelation};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct RenderNode<'a> {
    pub id: usize,
    pub attrs: RenderData<'a>,
    pub children: Vec<RenderNode<'a>>,
    pub element: &'a Element,
}

#[derive(Debug, PartialEq)]
pub struct RenderData<'a> {
    pub constraints: HashMap<&'a String, f64>,
    pub properties: HashMap<&'a String, f64>,
}

pub fn generate_render_tree<'a>(
    root: &'a StyleNode,
    solver: &Solver,
    variable_pool: &mut HashMap<usize, HashMap<&'a String, Variable>>,
) -> RenderNode<'a> {
    RenderNode {
        id: root.id,
        element: root.element,
        children: root
            .children
            .iter()
            .map(|child| {
                return generate_render_tree(child, solver, variable_pool);
            })
            .collect(), // ::Vec<RenderNode>(),
        attrs: RenderData {
            constraints: variable_pool
                .get(&root.id)
                .unwrap()
                .iter()
                .map(|(attr_name, var)| (*attr_name, solver.get_value(*var)))
                .collect(),
            properties: root
                .styles
                .properties
                .iter()
                .map(|(attr_name, attr_list)| {
                    let mut v = 0.0;
                    for (_, arith) in *attr_list {
                        match arith {
                            Arith::Num(n) => {
                                v = *n as f64;
                            }
                            _ => panic!("Invalid Arith"),
                        }
                    }
                    (*attr_name, v)
                })
                .collect(),
        },
    }
}
