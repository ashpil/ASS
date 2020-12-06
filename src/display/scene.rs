pub struct Scene {
    width: usize,
    height: usize,
    buffer: Vec<u32>
}

impl Scene {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![u32::MAX; width * height];
        Scene {
            width,
            height,
            buffer,
        }
    }

    pub fn update_window(&self, window: &mut minifb::Window) {
        window.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
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

    pub fn add_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        for line in y..=y+height {
            for pixel in self.buffer[(line * self.width + x)..=(line*self.width + x + width)].iter_mut() {
                *pixel = color;
            }
        }
    }
}

