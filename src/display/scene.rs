use super::color::rgb_to_u32;
use crate::dom::render_tree::RenderNode;
use crate::parser::asml_parser::Element;
use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use fontdue::Font;

pub struct Scene {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    font: Font,
}

impl Scene {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![u32::MAX; width * height];
        let font = include_bytes!("../../resources/fonts/Roboto-Regular.ttf") as &[u8];
        let settings = fontdue::FontSettings::default();
        let font = fontdue::Font::from_bytes(font, settings).unwrap();
        Scene {
            width,
            height,
            buffer,
            font,
        }
    }

    pub fn update_window(&self, window: &mut minifb::Window) {
        window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }

    pub fn clear(&mut self) {
        self.buffer.iter_mut().for_each(|x| *x = u32::MAX);
    }

    pub fn maybe_resize(&mut self, new_size: (usize, usize)) {
        if new_size != (self.width, self.height) {
            self.width = new_size.0;
            self.height = new_size.1;
            self.buffer.resize(self.width * self.height, u32::MAX);
        }
    }

    pub fn add_text(&mut self, content: &str, px: f32, left: f32, top: f32, right: f32, bottom: f32) {
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(&LayoutSettings {
            x: left,
            y: top,
            max_width: Some(right - left),
            max_height: Some(bottom - top),
            ..LayoutSettings::default()
        });
        layout.append(&[&self.font], &TextStyle::new(&content, px, 0));
        for glyph in layout.glyphs() {
            let (_, bitmap) = self.font.rasterize(glyph.key.c, px);
            if glyph.height != 0 {
                for j in 0..=glyph.height - 1 {
                    let start_x = (j + glyph.y as usize) * self.width + glyph.x as usize;
                    for i in 0..=glyph.width - 1 {
                        let gray = bitmap[j * glyph.width + i] as usize;
                        let color = rgb_to_u32(gray, gray, gray);
                        self.buffer[start_x + i] = self.buffer[start_x + i].saturating_sub(color)
                    }
                }
            }
        }
    }

    pub fn add_rect(&mut self, left: f32, top: f32, right: f32, bottom: f32, color: u32) {
        for line in (top as usize)..=(bottom as usize) {
            for pixel in
                self.buffer[(line * self.width + left as usize)..=(line * self.width + right as usize)].iter_mut()
            {
                *pixel = color;
            }
        }
    }

    pub fn process_render_tree(&mut self, root: &RenderNode) {
        match root.element {
            Element::Tag {
                traits: _,
                children: _,
            } => {
                if !root.attrs.constraints.is_empty() {
                    self.add_rect(
                        root.attrs.constraints.get(&"x".to_string()).copied().unwrap_or_default() as f32,
                        root.attrs.constraints.get(&"y".to_string()).copied().unwrap_or_default() as f32,
                        root.attrs.constraints.get(&"width".to_string()).copied().unwrap_or_default() as f32,
                        root.attrs.constraints.get(&"height".to_string()).copied().unwrap_or_default() as f32,
                        root.attrs.constraints.get(&"color".to_string()).copied().map_or(rgb_to_u32(100, 100, 200), |f| f as u32),
                    )
                }
            }
            Element::Text(_) => {}
        }
        for child in &root.children {
            self.process_render_tree(child);
        }
    }
}
