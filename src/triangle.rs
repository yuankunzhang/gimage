// use std::borrow::BorrowMut;

// use rand::Rng;

// use crate::worker::Worker;

// pub struct Triangle<'a> {
//     pub worker: &'a Worker<'a>,
//     pub p1: (i32, i32),
//     pub p2: (i32, i32),
//     pub p3: (i32, i32),
// }

// impl<'a> Triangle<'a> {
//     pub fn new(mut worker: Worker) -> Self {
//         let rng = worker.rng.borrow_mut();
//         let x1 = rng.gen_range(0..worker.w);
//         let y1 = rng.gen_range(0..worker.h);
//         let x2 = x1 + rng.gen_range(-15..16);
//         let y2 = y1 + rng.gen_range(-15..16);
//         let x3 = x1 + rng.gen_range(-15..16);
//         let y3 = y1 + rng.gen_range(-15..16);

//         Self {
//             worker,
//             p1: (x1, y1),
//             p2: (x2, y2),
//             p3: (x3, y3),
//         }
//     }

//     pub fn is_valid(&self) -> bool {
//         const MIN_DEGREE: f64 = 15.0;

//         let x1 = (self.p2.0 - self.p1.0) as f64;
//         let x2 = (self.p3.0 - self.p2.0) as f64;
//         let x3 = (self.p1.0 - self.p3.0) as f64;

//         let y1 = (self.p2.1 - self.p1.1) as f64;
//         let y2 = (self.p3.1 - self.p2.1) as f64;
//         let y3 = (self.p1.1 - self.p3.1) as f64;

//         let d1 = (x1 * x1 + y1 * y1).sqrt();
//         let d2 = (x2 * x2 + y2 * y2).sqrt();
//         let d3 = (x3 * x3 + y3 * y3).sqrt();

//         let a1 = degrees(((x1 * x2 + y1 * y2) / (d1 * d2)).acos());
//         let a2 = degrees(((x2 * x3 + y2 * y3) / (d2 * d3)).acos());
//         let a3 = degrees(((x3 * x1 + y3 * y1) / (d3 * d1)).acos());

//         a1 > MIN_DEGREE && a2 > MIN_DEGREE && a3 > MIN_DEGREE
//     }
// }

// fn degrees(radians: f64) -> f64 {
//     radians * 180.0 / std::f64::consts::PI
// }
