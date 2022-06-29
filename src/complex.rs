use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Complex {
    pub re: f32,
    pub im: f32,
}

impl Complex {
    pub fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }

    pub fn conjugate(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    pub fn abs(&self) -> f32 {
        self.abs_squared().sqrt()
    }

    pub fn abs_squared(&self) -> f32 {
        self.re * self.re + self.im * self.im
    }
}

impl From<f32> for Complex {
    fn from(re: f32) -> Self {
        Self { re, im: 0.0 }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Mul for Complex {
    type Output = Self;
    // (a + ib) * (c + id) = ac + iad + ibc - bd
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Div for Complex {
    type Output = Self;

    // (a + ib) / (c + id) = (a + ib) * (c - id) / (c^2 + d^2)
    fn div(self, rhs: Self) -> Self::Output {
        let mul = self * rhs.conjugate();
        let abs = rhs.abs_squared();
        Self {
            re: mul.re / abs,
            im: mul.im / abs,
        }
    }
}

impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl Add<Complex> for &Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Self::Output {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Sub<Complex> for &Complex {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Self::Output {
        Self::Output {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Mul<Complex> for &Complex {
    type Output = Complex;
    // (a + ib) * (c + id) = ac + iad + ibc - bd
    fn mul(self, rhs: Complex) -> Self::Output {
        Self::Output {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Div<Complex> for &Complex {
    type Output = Complex;

    // (a + ib) / (c + id) = (a + ib) * (c - id) / (c^2 + d^2)
    fn div(self, rhs: Complex) -> Self::Output {
        let mul = self * rhs.conjugate();
        let abs = rhs.abs_squared();
        Self::Output {
            re: mul.re / abs,
            im: mul.im / abs,
        }
    }
}

impl Neg for &Complex {
    type Output = Complex;

    fn neg(self) -> Self::Output {
        Self::Output {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl Add<&Complex> for &Complex {
    type Output = Complex;

    fn add(self, rhs: &Complex) -> Self::Output {
        Self::Output {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Sub<&Complex> for &Complex {
    type Output = Complex;

    fn sub(self, rhs: &Complex) -> Self::Output {
        Self::Output {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Mul<&Complex> for &Complex {
    type Output = Complex;
    // (a + ib) * (c + id) = ac + iad + ibc - bd
    fn mul(self, rhs: &Complex) -> Self::Output {
        Self::Output {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Div<&Complex> for &Complex {
    type Output = Complex;

    // (a + ib) / (c + id) = (a + ib) * (c - id) / (c^2 + d^2)
    fn div(self, rhs: &Complex) -> Self::Output {
        let mul = self * rhs.conjugate();
        let abs = rhs.abs_squared();
        Self::Output {
            re: mul.re / abs,
            im: mul.im / abs,
        }
    }
}

impl Add<&Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: &Complex) -> Self::Output {
        Self::Output {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Sub<&Complex> for Complex {
    type Output = Complex;

    fn sub(self, rhs: &Complex) -> Self::Output {
        Self::Output {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Mul<&Complex> for Complex {
    type Output = Complex;
    // (a + ib) * (c + id) = ac + iad + ibc - bd
    fn mul(self, rhs: &Complex) -> Self::Output {
        Self::Output {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Div<&Complex> for Complex {
    type Output = Complex;

    // (a + ib) / (c + id) = (a + ib) * (c - id) / (c^2 + d^2)
    fn div(self, rhs: &Complex) -> Self::Output {
        let mul = self * rhs.conjugate();
        let abs = rhs.abs_squared();
        Self::Output {
            re: mul.re / abs,
            im: mul.im / abs,
        }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.re == 0.0 {
            if self.im == 0.0 {
                write!(f, "0")
            } else {
                write!(f, "{}i", self.im)
            }
        } else {
            if self.im == 0.0 {
                write!(f, "{}", self.re)
            } else if self.im > 0.0 {
                write!(f, "{}+{}i", self.re, self.im)
            } else {
                write!(f, "{}{}i", self.re, self.im)
            }
        }
    }
}

impl Add<f32> for Complex {
    type Output = Complex;

    fn add(self, rhs: f32) -> Self::Output {
        Self::Output {
            re: self.re + rhs,
            im: self.im,
        }
    }
}
impl Sub<f32> for Complex {
    type Output = Complex;

    fn sub(self, rhs: f32) -> Self::Output {
        Self::Output {
            re: self.re - rhs,
            im: self.im,
        }
    }
}
impl Mul<f32> for Complex {
    type Output = Complex;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}
impl Div<f32> for Complex {
    type Output = Complex;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

impl Add<f32> for &Complex {
    type Output = Complex;

    fn add(self, rhs: f32) -> Self::Output {
        Self::Output {
            re: self.re + rhs,
            im: self.im,
        }
    }
}
impl Sub<f32> for &Complex {
    type Output = Complex;

    fn sub(self, rhs: f32) -> Self::Output {
        Self::Output {
            re: self.re - rhs,
            im: self.im,
        }
    }
}
impl Mul<f32> for &Complex {
    type Output = Complex;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}
impl Div<f32> for &Complex {
    type Output = Complex;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}
