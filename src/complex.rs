use crate::pages::SILENT;
use std::ops::Add;
use std::ops::Mul;
const DIV_LEN: usize = 200;
const DIV_LIM: usize = 1000;
type C32Fn = fn(C32) -> C32;
#[derive(Clone, Copy)]
pub struct C32 {
    pub a: f32,
    pub b: f32,
}
impl C32 {
    pub fn len(&self) -> f32 {
        (self.a * self.a + self.b * self.b).sqrt()
    }
    pub fn ang(&self) -> f32 {
        self.b.atan2(self.a)
    }
    pub fn from_rad(len: f32, ang: f32) -> C32 {
        C32 {
            a: len * ang.cos(),
            b: len * ang.sin(),
        }
    }
    pub fn dt_fullrange(dt: usize) -> usize {
        (dt as f32 / DIV_LEN as f32 * usize::MAX as f32) as usize
    }
    pub fn diverge_time(input: C32, fun: C32Fn) -> usize {
        let mut a: C32 = input;
        for i in 0..DIV_LEN {
            a = fun(a);
            // println!("{}, {}", a.len(), i);
            if a.len().abs() as usize > DIV_LIM {
                // return 0;
                return Self::dt_fullrange(i);
            }
        }
        if SILENT {
            // println!("{}", a.len());
        }
        return usize::MAX;
    }
    pub fn distance(&self, rhs: &Self) -> f32 {
        ((self.a - rhs.a).powf(2.0) + (self.b - rhs.b).powf(2.0)).sqrt()
    }
}
impl Add for C32 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        C32 {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}
impl Add for &C32 {
    type Output = C32;
    fn add(self, rhs: Self) -> Self::Output {
        C32 {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}
impl Mul<Self> for C32 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        C32::from_rad(self.len() * rhs.len(), self.ang() + rhs.ang())
    }
}
impl Mul<f32> for C32 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        C32 {
            a: self.a * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<Self> for &C32 {
    type Output = C32;
    fn mul(self, rhs: Self) -> Self::Output {
        C32::from_rad(self.len() * rhs.len(), self.ang() + rhs.ang())
    }
}
