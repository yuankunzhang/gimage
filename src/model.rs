use std::sync::Arc;

use image::{Rgba, RgbaImage};

use crate::worker::Worker;

pub struct Model {
    sw: u32,
    sh: u32,
    scale: f64,
    score: f64,
    background: Rgba<u8>,
    target: Arc<RgbaImage>,
    current: RgbaImage,
    workers: Vec<Worker>,
}

impl Model {
    pub fn new(target: RgbaImage, background: Rgba<u8>, size: u32, num_workers: usize) -> Self {
        let w = target.width();
        let h = target.height();
        let aspect = w as f64 / h as f64;

        let (sw, sh, scale) = if w > h {
            (size, (size as f64 / aspect) as u32, size as f64 / w as f64)
        } else {
            ((size as f64 * aspect) as u32, size, size as f64 / h as f64)
        };

        let target = Arc::new(target);

        let mut workers = Vec::with_capacity(num_workers);
        for _ in 0..num_workers {
            workers.push(Worker::new(target.clone()));
        }

        Self {
            sw,
            sh,
            scale,
            score: 0.0,
            background,
            target,
            current: RgbaImage::new(sw, sh),
            workers,
        }
    }
}
