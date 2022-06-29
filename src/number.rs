use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use crate::{complex::Complex, fraction::Fraction, longint::LongInt, matrix::Matrix};

pub trait NumNonRef:
    Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Neg<Output = Self>
    + PartialEq<Self>
    + From<f32>
    + Display
    + Clone
{
    fn norm_squared(&self) -> f32;
    fn norm(&self) -> f32 {
        self.norm_squared().sqrt()
    }
    fn conjugate(&self) -> Self;
    fn absolute(&self) -> Self;
}

pub trait NumRef<T>:
    Sized
    + Add<Self, Output = T>
    + Sub<Self, Output = T>
    + Mul<Self, Output = T>
    + Div<Self, Output = T>
    + Add<T, Output = T>
    + Sub<T, Output = T>
    + Mul<T, Output = T>
    + Div<T, Output = T>
    + Display
where
    T: NumNonRef,
{
}

impl NumNonRef for f32 {
    fn norm_squared(&self) -> f32 {
        self * self
    }

    fn conjugate(&self) -> Self {
        self.clone()
    }

    fn absolute(&self) -> Self {
        self.abs()
    }
}
impl NumNonRef for Complex {
    fn norm_squared(&self) -> f32 {
        self.abs_squared()
    }

    fn conjugate(&self) -> Self {
        self.conjugate()
    }

    fn absolute(&self) -> Self {
        self.abs().into()
    }
}
impl NumRef<f32> for &f32 {}
impl NumRef<Complex> for &Complex {}

impl From<f32> for LongInt {
    fn from(x: f32) -> Self {
        LongInt::from(x as i32)
    }
}

impl Mul<f32> for LongInt {
    type Output = LongInt;

    fn mul(self, rhs: f32) -> Self::Output {
        self * LongInt::from(rhs)
    }
}
impl Div<f32> for LongInt {
    type Output = LongInt;

    fn div(self, rhs: f32) -> Self::Output {
        self * LongInt::from(rhs)
    }
}

impl NumNonRef for LongInt {
    fn norm_squared(&self) -> f32 {
        let c = u32::from_le_bytes([self.get(0), self.get(1), self.get(2), self.get(3)]);

        c as f32
    }

    fn conjugate(&self) -> Self {
        self.clone()
    }

    fn absolute(&self) -> Self {
        self.abs()
    }
}
impl NumRef<LongInt> for &LongInt {}

impl<T> NumNonRef for Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    fn norm_squared(&self) -> f32 {
        todo!()
    }

    fn conjugate(&self) -> Self {
        todo!()
    }

    fn absolute(&self) -> Self {
        todo!()
    }
}

impl<T> NumRef<Fraction<T>> for &Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
}

pub fn from_f32_mat<T: From<f32>>(a: &Matrix<f32>) -> Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    let mut m = Matrix::new(a.width(), a.height());

    for i in 0..a.height() {
        for j in 0..a.width() {
            m.set(i, j, T::from(a.get(i, j).clone()));
        }
    }

    m
}
