use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use crate::number::{NumNonRef, NumRef};

#[derive(Debug)]
pub struct Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    coefs: Vec<T>,
}

impl<T> Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    pub fn new() -> Self {
        Self { coefs: Vec::new() }
    }

    pub fn from_coefs(coefs: &[T]) -> Self {
        Self {
            coefs: coefs.to_vec(),
        }
    }

    pub fn degree(&self) -> usize {
        self.coefs.len() - 1
    }

    pub fn get(&self, power: usize) -> T {
        self.coefs.get(power).unwrap_or(&0.0.into()).clone()
    }

    pub fn set(&mut self, power: usize, val: T) {
        let needs_expanding;
        if self.coefs.len() <= power {
            needs_expanding = true;
        } else {
            needs_expanding = false;
        }

        if !needs_expanding {
            self.coefs[power] = val;
        } else if val != 0.0.into() {
            self.coefs.resize(power + 1, 0.0.into());
            self.coefs[power] = val;
        }
    }

    pub fn normalize(&self) -> Self {
        self / self.get(self.degree())
    }
}

impl<T> Display for Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.coefs.len() > 0 {
            // write!(f, "{}", self.coefs[0])?;
            // for i in 1..self.coefs.len() {
            //     if self.coefs[i] != 0.0.into() {
            //         write!(f, " + {}λ{}", self.coefs[i], superscript_number(i))?;
            //     }
            // }

            write!(f, "cvec = ...\n[")?;
            for i in 0..self.coefs.len() - 1 {
                let i = self.coefs.len() - i - 1;
                write!(f, "{}; ", self.coefs[i])?;
            }
            write!(f, "{}];", self.coefs.last().unwrap())?;
        }
        Ok(())
    }
}

fn add_poly<T>(a: &Polynome<T>, b: &Polynome<T>) -> Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    let len = usize::max(a.coefs.len(), b.coefs.len());
    let mut res = Polynome::new();
    for i in 0..len {
        res.set(i, a.get(i) + b.get(i));
    }

    res
}

fn sub_poly<T>(a: &Polynome<T>, b: &Polynome<T>) -> Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    let len = usize::max(a.coefs.len(), b.coefs.len());
    let mut res = Polynome::new();
    for i in 0..len {
        res.set(i, a.get(i) - b.get(i));
    }

    res
}

fn mul_poly<T>(a: &Polynome<T>, b: &Polynome<T>) -> Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    let mut res = Polynome::new();

    for i in 0..a.coefs.len() {
        let i = a.coefs.len() - i - 1;
        for j in 0..b.coefs.len() {
            let j = b.coefs.len() - j - 1;
            let c = res.get(i + j) + a.get(i) * b.get(j);
            // println!("{} = {} + {} * {}", c, res.get(i + j), a.get(i), b.get(j));
            res.set(i + j, c);
        }
    }

    res
}

impl<T> Add<Polynome<T>> for Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Polynome<T>;

    fn add(self, rhs: Polynome<T>) -> Self::Output {
        add_poly(&self, &rhs)
    }
}

impl<T> Sub<Polynome<T>> for Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Polynome<T>;

    fn sub(self, rhs: Polynome<T>) -> Self::Output {
        sub_poly(&self, &rhs)
    }
}

impl<T> Mul<Polynome<T>> for Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Polynome<T>;

    fn mul(self, rhs: Polynome<T>) -> Self::Output {
        mul_poly(&self, &rhs)
    }
}

impl<T> Add<Polynome<T>> for &Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Polynome<T>;

    fn add(self, rhs: Polynome<T>) -> Self::Output {
        add_poly(&self, &rhs)
    }
}

impl<T> Sub<Polynome<T>> for &Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Polynome<T>;

    fn sub(self, rhs: Polynome<T>) -> Self::Output {
        sub_poly(&self, &rhs)
    }
}

impl<T> Mul<Polynome<T>> for &Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Polynome<T>;

    fn mul(self, rhs: Polynome<T>) -> Self::Output {
        mul_poly(&self, &rhs)
    }
}

impl<T> Add<&Polynome<T>> for Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Polynome<T>;

    fn add(self, rhs: &Polynome<T>) -> Self::Output {
        add_poly(&self, &rhs)
    }
}

impl<T> Sub<&Polynome<T>> for Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Polynome<T>;

    fn sub(self, rhs: &Polynome<T>) -> Self::Output {
        sub_poly(&self, &rhs)
    }
}

impl<T> Mul<&Polynome<T>> for Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Polynome<T>;

    fn mul(self, rhs: &Polynome<T>) -> Self::Output {
        mul_poly(&self, &rhs)
    }
}

impl<T> Add<&Polynome<T>> for &Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Polynome<T>;

    fn add(self, rhs: &Polynome<T>) -> Self::Output {
        add_poly(&self, &rhs)
    }
}

impl<T> Sub<&Polynome<T>> for &Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Polynome<T>;

    fn sub(self, rhs: &Polynome<T>) -> Self::Output {
        sub_poly(&self, &rhs)
    }
}

impl<T> Mul<&Polynome<T>> for &Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Polynome<T>;

    fn mul(self, rhs: &Polynome<T>) -> Self::Output {
        mul_poly(&self, &rhs)
    }
}

impl<T> From<T> for Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    fn from(a0: T) -> Self {
        Polynome::from_coefs(&[a0])
    }
}

impl<T> Div<T> for Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Polynome<T>;

    fn div(self, rhs: T) -> Self::Output {
        let mut res = Vec::with_capacity(self.coefs.len());
        for c in &self.coefs {
            res.push(c / &rhs);
        }
        Self::Output::from_coefs(&res)
    }
}

impl<T> Div<T> for &Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Polynome<T>;

    fn div(self, rhs: T) -> Self::Output {
        let mut res = Vec::with_capacity(self.coefs.len());
        for c in &self.coefs {
            res.push(c / &rhs);
        }
        Self::Output::from_coefs(&res)
    }
}

impl<T> Div<&T> for Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Polynome<T>;

    fn div(self, rhs: &T) -> Self::Output {
        let mut res = Vec::with_capacity(self.coefs.len());
        for c in &self.coefs {
            res.push(c / rhs);
        }
        Self::Output::from_coefs(&res)
    }
}

impl<T> Mul<&T> for &Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Polynome<T>;

    fn mul(self, rhs: &T) -> Self::Output {
        let mut res = Vec::with_capacity(self.coefs.len());
        for c in &self.coefs {
            res.push(c * rhs);
        }
        Self::Output::from_coefs(&res)
    }
}

impl<T> Mul<T> for Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Polynome<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut res = Vec::with_capacity(self.coefs.len());
        for c in &self.coefs {
            res.push(c * &rhs);
        }
        Self::Output::from_coefs(&res)
    }
}

impl<T> Mul<T> for &Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Polynome<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut res = Vec::with_capacity(self.coefs.len());
        for c in &self.coefs {
            res.push(c * &rhs);
        }
        Self::Output::from_coefs(&res)
    }
}

impl<T> Mul<&T> for Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Polynome<T>;

    fn mul(self, rhs: &T) -> Self::Output {
        let mut res = Vec::with_capacity(self.coefs.len());
        for c in &self.coefs {
            res.push(c * rhs);
        }
        Self::Output::from_coefs(&res)
    }
}

impl<T> Div<&T> for &Polynome<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Polynome<T>;

    fn div(self, rhs: &T) -> Self::Output {
        let mut res = Vec::with_capacity(self.coefs.len());
        for c in &self.coefs {
            res.push(c / rhs);
        }
        Self::Output::from_coefs(&res)
    }
}
