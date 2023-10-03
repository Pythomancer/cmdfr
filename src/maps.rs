use crate::complex::C32;
use crate::pages::Map;
pub fn say_hello() {
    println!("ello mate");
}
fn f2u(i: f32) -> usize {
    (i.max(0.0).min(1.0) * usize::MAX as f32).round() as usize
}
pub fn const_dist(_x: f32, _y: f32) -> usize {
    return 1;
}
pub fn lin_dist(x: f32, y: f32) -> usize {
    f2u(x + y)
}
pub fn sin_dist(x: f32, y: f32) -> usize {
    f2u((x.sin() + y.cos()) / 4.0 + 0.5)
}
pub fn julia(c: C32) -> C32 {
    let unit = C32 { a: 0.0, b: -1.0 };
    return &c * &c + unit;
}
pub fn fractal2(c: C32) -> C32 {
    let unit = C32 { a:  -0.5251993, b:  -0.5251993 };
    return &c * &c + unit;
}
pub fn mandelbrot(c: C32) -> C32 {
    let mut z = c; 
    todo!();
}

pub fn fractal_pixel(x:f32, y:f32) -> usize {
    let p = C32 { a: x, b: y };
    C32::diverge_time(p, fractal2)
}

pub fn angle_dist(x: f32, y: f32) -> usize {
    f2u(y.atan2(x)/6.28+0.5)
}

// pub fn fuzz(x: f32, y: f32, pts: Vec<C32>) -> Map{
//     let q = pts.into_iter().min_by_key(|r| (r.distance(&C32{a:x, b:y})*100.0).round() as i32).expect("msg");

//     |x, y| f2u(C32{a:x, b:y}.distance(&pts.clone().into_iter().min_by_key(|r| (r.distance(&C32{a:x, b:y})*100.0).round() as i32).expect("msg")))
//     // |x, y| 8_usize

// }