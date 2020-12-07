use minifb::clamp;

pub fn rgb_to_u32(red: usize, green: usize, blue: usize) -> u32 {
    let r = clamp(0, red, 255);
    let g = clamp(0, green, 255);
    let b = clamp(0, blue, 255);
    ((r << 16) | (g << 8) | b) as u32
}

