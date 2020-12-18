use ass::display::{rgb_to_u32, Scene};
use ass::dom::{
    construct_style_tree, generate_render_tree, generate_variable_pool, solve_constraints,
};
use ass::parser::parser;
use cassowary::strength::{REQUIRED, STRONG, WEAK};
use cassowary::WeightedRelation::*;
use cassowary::{Solver, Variable};
use minifb::{Key, ScaleMode, Window, WindowOptions};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;
use std::process::exit;

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
    let code = if let Some(filename) = env::args().collect::<Vec<String>>().get(1) {
        if let Ok(contents) = read_to_string(filename) {
            parser(&contents)
        } else {
            eprintln!("error: {}: invalid file", filename);
            exit(1);
        }
    } else {
        eprintln!("Usage: ass [filename]");
        exit(1);
    }
    .unwrap();

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
        "y".to_string(),
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

    let mut styles_to_id = HashMap::new();

    let style_tree = construct_style_tree(
        &code.0,
        &code.1,
        &constraint_names,
        &property_names,
        0,
        &default_attributes,
        1,
        &mut styles_to_id,
    );

    let mut variable_pool = HashMap::new();

    generate_variable_pool(&style_tree, &code.1, &constraint_names, &mut variable_pool);
    println!("{:#?}", style_tree);

    solve_constraints(&style_tree, &mut variable_pool, &mut solver, &styles_to_id);
    println!("{:#?}", variable_pool);
    print_changes(&variable_pool, &solver);
    let render_tree = generate_render_tree(&style_tree, &solver, &mut variable_pool);
    println!("{:#?}", render_tree);

    let mut window = Window::new(
        "ASS",
        500,
        500,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::UpperLeft,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create window");

    let mut scene = Scene::new(500, 500);

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        scene.clear();
        scene.maybe_resize(window.get_size());
        // scene.add_rect(20, 20, 100, 100, rgb_to_u32(100, 200, 100));
        scene.process_render_tree(&render_tree);
        scene.update_window(&mut window);
    }
}
