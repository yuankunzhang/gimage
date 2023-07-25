use image::{Rgba, RgbaImage};

use crate::scanline::Scanline;

fn compute_color(
    target: &RgbaImage,
    current: &RgbaImage,
    lines: &[Scanline],
    alpha: u8,
) -> Rgba<u8> {
    todo!()
    // let mut rsum: i64 = 0;
    // let mut gsum: i64 = 0;
    // let mut bsum: i64 = 0;
    // let mut count: i64 = 0;
    // let a = 0x101 * 255 / (alpha as i64);

    // for line in lines {
    //     let mut i = (line.y * target.width() as i32 + line.x1) as usize;

    //     for _ in line.x1..=line.x2 {
    //         let tr = target.get_pixel(x, y) as i64;
    //         let tg = target.as_raw()[i + 1] as i64;
    //         let tb = target.as_raw()[i + 2] as i64;
    //         let cr = current.as_raw()[i] as i64;
    //         let cg = current.as_raw()[i + 1] as i64;
    //         let cb = current.as_raw()[i + 2] as i64;
    //         i += 4;
    //         rsum += ((tr - cr) * a + cr * 0x101) as i64;
    //         gsum += ((tg - cg) * a + cg * 0x101) as i64;
    //         bsum += ((tb - cb) * a + cb * 0x101) as i64;
    //         count += 1;
    //     }
    // }

    // if count == 0 {
    //     return Rgba([0, 0, 0, 0]);
    // }

    // let r = ((rsum / count) >> 8).clamp(0, 255) as u8;
    // let g = ((gsum / count) >> 8).clamp(0, 255) as u8;
    // let b = ((bsum / count) >> 8).clamp(0, 255) as u8;

    // Rgba([r, g, b, alpha])
}
