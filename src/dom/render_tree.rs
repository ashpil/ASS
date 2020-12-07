use super::style_tree::{retrieve_variable, StyleNode};
use cassowary::{AddConstraintError, Constraint, Solver, Variable, WeightedRelation};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct RenderNode<'a> {
    id: usize,
    attrs: RenderData<'a>,
    children: Vec<RenderNode<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct RenderData<'a> {
    constraints: Vec<(&'a String, f64)>,
    properties: Vec<(&'a String, &'a String)>,
}

pub fn generate_render_tree<'a>(
    root: &'a StyleNode,
    solver: &Solver,
    variable_pool: &mut HashMap<usize, HashMap<&String, Variable>>,
) -> RenderNode<'a> {
    RenderNode {
        id: root.id,
        children: root
            .children
            .iter()
            .map(|child| {
                return generate_render_tree(child, solver, variable_pool);
            })
            .collect(), // ::Vec<RenderNode>(),
        attrs: RenderData {
            constraints: root
                .styles
                .constraints
                .iter()
                .map(|(attr_name, _)| {
                    (
                        *attr_name,
                        solver.get_value(retrieve_variable(variable_pool, root.id, attr_name)),
                    )
                })
                .collect(),
            properties: root
                .styles
                .properties
                .iter()
                .map(|(attr_name, _)| (*attr_name, *attr_name))
                .collect(),
        },
    }
}
