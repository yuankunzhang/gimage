pub struct Scanline {
    pub y: i32,
    pub x1: i32,
    pub x2: i32,
    pub alpha: u32,
}

fn crop_scanlines(lines: &mut [Scanline], w: i32, h: i32) {
    for line in lines {
        if line.y < 0 || line.y >= h || line.x1 >= w || line.x2 < 0 {
            continue;
        }

        let x1 = line.x1.clamp(0, w - 1);
        let x2 = line.x2.clamp(0, w - 1);
        if x1 > x2 {
            continue;
        }

        line.x1 = x1;
        line.x2 = x2;
    }
}
