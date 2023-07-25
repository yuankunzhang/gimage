use image::{Rgba, RgbaImage};

pub struct Model {
    sw: i32,
    sh: i32,
    scale: f64,
    score: f64,
    background: Rgba<u8>,
    target: RgbaImage,
    current: RgbaImage,
}
