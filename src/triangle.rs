use std::borrow::BorrowMut;

use rand::Rng;

use crate::worker::Worker;

pub struct Triangle {
    pub worker: Worker,
    pub p1: (i32, i32),
    pub p2: (i32, i32),
    pub p3: (i32, i32),
}

impl Triangle {
    pub fn new(mut worker: Worker) -> Self {
        let rng = worker.rng.borrow_mut();
        let x1 = rng.gen_range(0..worker.w);
        let y1 = rng.gen_range(0..worker.h);
        let x2 = x1 + rng.gen_range(-15..16);
        let y2 = y1 + rng.gen_range(-15..16);
        let x3 = x1 + rng.gen_range(-15..16);
        let y3 = y1 + rng.gen_range(-15..16);

        Self {
            worker,
            p1: (x1, y1),
            p2: (x2, y2),
            p3: (x3, y3),
        }
    }

    pub fn is_valid(&self) -> bool {
        let (x1, y1) = self.p1;
        let (x2, y2) = self.p2;
        let (x3, y3) = self.p3;
        let (w, h) = (self.worker.w, self.worker.h);

        if x1 < 0 || x1 >= w || y1 < 0 || y1 >= h {
            return false;
        }

        if x2 < 0 || x2 >= w || y2 < 0 || y2 >= h {
            return false;
        }

        if x3 < 0 || x3 >= w || y3 < 0 || y3 >= h {
            return false;
        }

        true
    }
}
