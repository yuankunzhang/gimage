use std::sync::Arc;

use image::RgbaImage;

pub struct Worker {
    pub rng: rand::rngs::ThreadRng,
    pub target: Arc<RgbaImage>,
    pub w: u32,
    pub h: u32,
}

impl Worker {
    pub fn new(target: Arc<RgbaImage>) -> Self {
        let (w, h) = (target.width(), target.height());

        Self {
            rng: rand::thread_rng(),
            target,
            w,
            h,
        }
    }
}
