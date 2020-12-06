use minifb::{Key, ScaleMode, Window, WindowOptions};
use ass::parser::parser;
use ass::display::{Scene, rgb_to_u32};
use std::fs::read_to_string;
use std::env;
use std::process::exit;

fn main() {
    let ass = if let Some(filename) = env::args().collect::<Vec<String>>().get(1) {
        if let Ok(contents) = read_to_string(filename) {
            parser(&contents)
        } else {
            eprintln!("error: {}: invalid file", filename);
            exit(1);
        }
    } else {
        eprintln!("Usage: ass [filename]");
        exit(1);
    };

    let mut window = Window::new(
        "ASS",
        500,
        500,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::UpperLeft,
            ..WindowOptions::default()
        },
    ).expect("Unable to create window");

    let mut scene = Scene::new(500, 500);

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        scene.clear();
        scene.maybe_resize(window.get_size());
        scene.add_rect(20, 20, 100, 100, rgb_to_u32(100, 200, 100));
        scene.update_window(&mut window);
    }
}
