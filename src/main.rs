use ass::dom::{construct_style_tree, generate_variable_pool, solve_constraints};
use ass::parser::parser;
use cassowary::strength::{REQUIRED, STRONG, WEAK};
use cassowary::WeightedRelation::*;
use cassowary::{Solver, Variable};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read input");
    buffer
}

fn print_changes(variable_pool: &HashMap<usize, HashMap<&String, Variable>>, solver: &Solver) {
    println!("Changes:");
    for (id, attr_to_var) in variable_pool {
        for (attr_name, var) in attr_to_var {
            println!("{}[{}] = {}", id, attr_name, solver.get_value(*var))
        }
    }
}

fn main() {
    println!("Enter the path to the file:");
    let file_path = get_input();
    let code = read_to_string(file_path.trim()).expect("Failed to find file");
    let parsed_code = parser(&code).expect("Failed to parse file");
    // println!("{:#?}", parsed_code);
    let mut solver = Solver::new();
    let window_width = Variable::new();
    let window_height = Variable::new();

    let constraint_names: HashSet<String> = [
        "left".to_string(),
        "right".to_string(),
        "top".to_string(),
        "bottom".to_string(),
        "width".to_string(),
        "height".to_string(),
        "x".to_string(),
        "y".to_string()
    ]
    .iter()
    .cloned()
    .collect();

    let property_names: HashSet<String> =
        ["background-color".to_string()].iter().cloned().collect();

    solver.add_constraints(&[
        window_width | EQ(REQUIRED) | 800.0,
        window_height | EQ(REQUIRED) | 600.0,
    ]);

    let default_attributes = HashMap::new();

    let style_tree = construct_style_tree(
        &parsed_code.0,
        &parsed_code.1,
        &constraint_names,
        &property_names,
        0,
        &default_attributes,
    );

    let mut variable_pool = HashMap::new();

    generate_variable_pool(
        &style_tree,
        &parsed_code.1,
        &constraint_names,
        &mut variable_pool,
    );

    solve_constraints(&style_tree, &mut variable_pool, &mut solver);
    println!("{:#?}", style_tree);
    println!("{:#?}", variable_pool);
    print_changes(&variable_pool, &solver);

    // let mut names = HashMap::new();

    // let window_width = Variable::new();
    // names.insert(window_width, "window_width");

    // struct Element {
    //     left: Variable,
    //     right: Variable,
    // }

    // let box1 = Element {
    //     left: Variable::new(),
    //     right: Variable::new(),
    // };

    // names.insert(box1.left, "box1.left");
    // names.insert(box1.right, "box1.right");

    // let box2 = Element {
    //     left: Variable::new(),
    //     right: Variable::new(),
    // };
    // names.insert(box2.left, "box2.left");
    // names.insert(box2.right, "box2.right");

    // let mut solver = Solver::new();
    // solver
    //     .add_constraints(&[
    //         window_width | GE(REQUIRED) | 0.0,        // positive window width
    //         box1.left | EQ(REQUIRED) | 0.0,           // left align
    //         box2.right | EQ(REQUIRED) | window_width, // right align
    //         box2.left | GE(REQUIRED) | box1.right,    // no overlap
    //         box1.left | LE(REQUIRED) | box1.right,
    //         box2.left | LE(REQUIRED) | box2.right,
    //         box1.right - box1.left | EQ(WEAK) | 50.0,
    //         box2.right - box2.left | EQ(WEAK) | 100.0,
    //     ])
    //     .unwrap();
    // print_changes(&names, solver.fetch_changes());
    // solver.add_edit_variable(window_width, STRONG).unwrap();
    // solver.suggest_value(window_width, 300.0).unwrap();
    // print_changes(&names, solver.fetch_changes());
}
