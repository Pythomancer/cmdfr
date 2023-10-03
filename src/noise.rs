use std::time::SystemTime;
use std::time::UNIX_EPOCH;
pub struct Noiser {
    count: usize,
}
use crate::complex::C32;
pub fn gen_map(count: usize) -> Vec<C32> {
    let mut n = Noiser { count: 0 };
    let mut out = Vec::<C32>::new();
    for _i in 0..count {
        out.push(C32 {
            a: n.rand() as f32,
            b: n.rand() as f32,
        })
    }
    out
}
impl Noiser {
    pub fn rand(&mut self) -> f64 {
        self.count += 1;
        ((SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("msg")
            .as_nanos() as f64
            / 100.0)
            % 10000.0
            + self.count as f64)
            .sin()
            .abs()
    }
}
