use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use crate::number::{NumNonRef, NumRef};

#[derive(Debug, Clone, PartialEq)]
pub struct Fraction<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    num: T,
    den: T,
}

impl<T> Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    pub fn new(mut den: T, mut num: T) -> Self {
        let sign = (den >= 0.0.into()) == (num >= 0.0.into());
        den = den.absolute();
        num = num.absolute();

        let mut res = if sign {
            Self { num, den }
        } else {
            Self { num: -num, den }
        };

        res.simplify();
        res
    }

    fn gcd(mut a: T, mut b: T) -> T {
        while b != 0.0.into() {
            let t = b.clone();
            b = &a % &b;
            a = t;
        }
        a
    }

    fn simplify(&mut self) {
        let a = self.num.clone();
        let b = self.den.clone();

        let gcd = Self::gcd(a, b);
        self.num = &self.num / &gcd;
        self.den = &self.den / &gcd;
    }
}

fn add_frac<T>(a: &Fraction<T>, b: &Fraction<T>) -> Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    let den = &a.den * &b.den;
    let num = &a.num * &b.den + &b.num * &a.den;

    Fraction::new(den, num)
}

fn sub_frac<T>(a: &Fraction<T>, b: &Fraction<T>) -> Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    let den = &a.den * &b.den;
    let num = &a.num * &b.den - &b.num * &a.den;

    Fraction::new(den, num)
}

fn mul_frac<T>(a: &Fraction<T>, b: &Fraction<T>) -> Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    let den = &a.den * &b.den;
    let num = &a.num * &b.num;

    Fraction::new(den, num)
}

fn div_frac<T>(a: &Fraction<T>, b: &Fraction<T>) -> Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    let den = &a.den * &b.num;
    let num = &a.num * &b.den;

    Fraction::new(den, num)
}

impl<T> Add<Fraction<T>> for Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    type Output = Fraction<T>;

    fn add(self, rhs: Fraction<T>) -> Self::Output {
        add_frac(&self, &rhs)
    }
}

impl<T> Sub<Fraction<T>> for Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    type Output = Fraction<T>;

    fn sub(self, rhs: Fraction<T>) -> Self::Output {
        sub_frac(&self, &rhs)
    }
}

impl<T> Mul<Fraction<T>> for Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    type Output = Fraction<T>;

    fn mul(self, rhs: Fraction<T>) -> Self::Output {
        mul_frac(&self, &rhs)
    }
}

impl<T> Div<Fraction<T>> for Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    type Output = Fraction<T>;

    fn div(self, rhs: Fraction<T>) -> Self::Output {
        div_frac(&self, &rhs)
    }
}

impl<T> Add<&Fraction<T>> for Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    type Output = Fraction<T>;

    fn add(self, rhs: &Fraction<T>) -> Self::Output {
        add_frac(&self, &rhs)
    }
}

impl<T> Sub<&Fraction<T>> for Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    type Output = Fraction<T>;

    fn sub(self, rhs: &Fraction<T>) -> Self::Output {
        sub_frac(&self, &rhs)
    }
}

impl<T> Mul<&Fraction<T>> for Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    type Output = Fraction<T>;

    fn mul(self, rhs: &Fraction<T>) -> Self::Output {
        mul_frac(&self, &rhs)
    }
}

impl<T> Div<&Fraction<T>> for Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    type Output = Fraction<T>;

    fn div(self, rhs: &Fraction<T>) -> Self::Output {
        div_frac(&self, &rhs)
    }
}

impl<T> Add<&Fraction<T>> for &Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    type Output = Fraction<T>;

    fn add(self, rhs: &Fraction<T>) -> Self::Output {
        add_frac(&self, &rhs)
    }
}

impl<T> Sub<&Fraction<T>> for &Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    type Output = Fraction<T>;

    fn sub(self, rhs: &Fraction<T>) -> Self::Output {
        sub_frac(&self, &rhs)
    }
}

impl<T> Mul<&Fraction<T>> for &Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    type Output = Fraction<T>;

    fn mul(self, rhs: &Fraction<T>) -> Self::Output {
        mul_frac(&self, &rhs)
    }
}

impl<T> Div<&Fraction<T>> for &Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    type Output = Fraction<T>;

    fn div(self, rhs: &Fraction<T>) -> Self::Output {
        div_frac(&self, &rhs)
    }
}

impl<T> Add<Fraction<T>> for &Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    type Output = Fraction<T>;

    fn add(self, rhs: Fraction<T>) -> Self::Output {
        add_frac(&self, &rhs)
    }
}

impl<T> Sub<Fraction<T>> for &Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    type Output = Fraction<T>;

    fn sub(self, rhs: Fraction<T>) -> Self::Output {
        sub_frac(&self, &rhs)
    }
}

impl<T> Mul<Fraction<T>> for &Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    type Output = Fraction<T>;

    fn mul(self, rhs: Fraction<T>) -> Self::Output {
        mul_frac(&self, &rhs)
    }
}

impl<T> Div<Fraction<T>> for &Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    type Output = Fraction<T>;

    fn div(self, rhs: Fraction<T>) -> Self::Output {
        div_frac(&self, &rhs)
    }
}

impl<T> From<f32> for Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    fn from(x: f32) -> Self {
        let den = T::from(100.0 / x.fract());

        Fraction {
            num: T::from(x.trunc()),
            den: T::from(1.0),
        } + Fraction {
            num: T::from(100.0),
            den,
        }
    }
}

impl<T> Into<f32> for Fraction<T>
where
    T: NumNonRef + PartialOrd + Into<f32>,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    fn into(self) -> f32 {
        let whole = &self.num / &self.den;
        let remainder = &self.num % &self.den;

        whole.into() + 1.0 / remainder.into()
    }
}

impl<T> Neg for Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    type Output = Fraction<T>;

    fn neg(self) -> Self::Output {
        Self::Output {
            num: -self.num,
            den: self.den,
        }
    }
}

impl<T> Neg for &Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    type Output = Fraction<T>;

    fn neg(self) -> Self::Output {
        Self::Output {
            num: -self.num.clone(),
            den: self.den.clone(),
        }
    }
}

impl<T> Display for Fraction<T>
where
    T: NumNonRef + PartialOrd,
    for<'a> &'a T: NumRef<T> + Rem<Output = T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}
