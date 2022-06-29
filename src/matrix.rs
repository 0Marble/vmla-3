use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use crate::number::{NumNonRef, NumRef};

#[derive(Debug)]
pub enum MatrixError {
    NotSquare,
    NotRegular,
    IOError(String),
    InvalidFileFormat,
    SizeMismatch,
    UnexpectedAnswer,
    NotTridiagonal,
    UnsopportedOperation,
}

impl Display for MatrixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatrixError::NotSquare => write!(f, "NotSquare"),
            MatrixError::NotRegular => write!(f, "NotRegular"),
            MatrixError::IOError(e) => write!(f, "IOError {}", e),
            MatrixError::InvalidFileFormat => write!(f, "InvalidFileFormat"),
            MatrixError::SizeMismatch => write!(f, "SizeMismatch"),
            MatrixError::UnexpectedAnswer => write!(f, "UnexpectedAnswer"),
            MatrixError::NotTridiagonal => write!(f, "NotTridiagnoal"),
            MatrixError::UnsopportedOperation => write!(f, "Unsopported Operation"),
        }
    }
}

impl From<std::io::Error> for MatrixError {
    fn from(e: std::io::Error) -> Self {
        MatrixError::IOError(format!("{}", e))
    }
}

#[derive(Clone, Debug)]
pub struct Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    elems: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    pub fn new(width: usize, height: usize) -> Self {
        let mut elems = Vec::with_capacity(width * height);
        elems.resize(width * height, 0.0.into());

        Self {
            elems,
            width,
            height,
        }
    }

    pub fn from_vec(elems: Vec<T>, width: usize) -> Result<Self, MatrixError> {
        if elems.len() % width != 0 {
            Err(MatrixError::SizeMismatch)
        } else {
            Ok(Self {
                height: elems.len() / width,
                elems,
                width,
            })
        }
    }

    pub fn identity(width: usize) -> Self {
        let mut elems = Vec::with_capacity(width * width);
        elems.resize(width * width, 0.0.into());
        for i in 0..width {
            elems[i * width + i] = 1.0.into();
        }
        Self {
            elems,
            width,
            height: width,
        }
    }

    pub fn scalar(x: T, width: usize) -> Self {
        let mut elems = Vec::with_capacity(width * width);
        elems.resize(width * width, x);
        Self {
            elems,
            width,
            height: width,
        }
    }

    #[inline(always)]
    pub fn get(&self, row: usize, column: usize) -> &T {
        &self.elems[row * self.width + column]
    }

    #[inline(always)]
    pub fn set(&mut self, row: usize, column: usize, val: T) {
        self.elems[row * self.width + column] = val;
    }

    #[inline(always)]
    pub fn width(&self) -> usize {
        self.width
    }
    #[inline(always)]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline(always)]
    pub fn elems_raw(&self) -> &[T] {
        &self.elems
    }

    pub fn transpose(&self) -> Self {
        let mut a = Vec::with_capacity(self.width * self.height);
        a.resize(self.width * self.height, 0.0.into());

        for i in 0..self.height {
            for j in 0..self.width {
                a[j * self.height + i] = self.get(i, j).clone();
            }
        }

        Matrix::from_vec(a, self.height).unwrap()
    }

    pub fn hermetian_transpose(&self) -> Self {
        let mut a = Vec::with_capacity(self.width * self.height);
        a.resize(self.width * self.height, 0.0.into());

        for i in 0..self.height {
            for j in 0..self.width {
                a[j * self.height + i] = self.get(i, j).conjugate();
            }
        }

        Matrix::from_vec(a, self.height).unwrap()
    }

    pub fn norm_squared(&self) -> f32 {
        let mut sum = 0.0;
        for i in 0..self.width * self.height {
            sum += self.elems[i].norm_squared();
        }
        sum
    }

    pub fn norm(&self) -> f32 {
        self.norm_squared().sqrt()
    }

    pub fn row(&self, row: usize) -> Self {
        let mut elems = Vec::with_capacity(self.width);
        for i in 0..self.width {
            elems.push(self.get(row, i).clone());
        }

        Self {
            elems,
            width: self.width,
            height: 1,
        }
    }

    pub fn column(&self, column: usize) -> Self {
        let mut elems = Vec::with_capacity(self.height);
        for i in 0..self.height {
            elems.push(self.get(i, column).clone());
        }

        Self {
            elems,
            width: 1,
            height: self.height,
        }
    }
}

impl<T> Display for Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.width > 0 && self.height > 0 {
            for i in 0..self.height {
                write!(f, "| ")?;
                for j in 0..self.width {
                    write!(f, "{} ", self.get(i, j))?;
                }
                write!(f, "|\n")?;
            }
            write!(f, "")
        } else {
            write!(f, "[ ]")
        }
    }
}

impl<T> Add<Matrix<T>> for Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Result<Matrix<T>, MatrixError>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        if self.width != rhs.width || self.height != rhs.height {
            return Err(MatrixError::SizeMismatch);
        }
        let mut c = Vec::with_capacity(self.width * self.height);
        c.resize(self.width * self.height, 0.0.into());
        let a = self.elems_raw();
        let b = rhs.elems_raw();

        for i in 0..self.width * self.height {
            c[i] = &a[i] + &b[i];
        }

        Ok(Matrix::from_vec(c, self.width)?)
    }
}

impl<T> Sub<Matrix<T>> for Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Result<Matrix<T>, MatrixError>;

    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        if self.width != rhs.width || self.height != rhs.height {
            return Err(MatrixError::SizeMismatch);
        }
        let mut c = Vec::with_capacity(self.width * self.height);
        c.resize(self.width * self.height, 0.0.into());
        let a = self.elems_raw();
        let b = rhs.elems_raw();

        for i in 0..self.width * self.height {
            c[i] = &a[i] - &b[i];
        }

        Ok(Matrix::from_vec(c, self.width)?)
    }
}

impl<T> Mul<Matrix<T>> for Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Result<Matrix<T>, MatrixError>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        if self.width != rhs.height {
            return Err(MatrixError::SizeMismatch);
        }
        let mut c = Vec::with_capacity(rhs.width * self.height);
        c.resize(rhs.width * self.height, 0.0.into());
        let a = self.elems_raw();
        let b = rhs.elems_raw();

        for i in 0..self.height {
            for j in 0..rhs.width {
                for k in 0..self.width {
                    c[i * rhs.width + j] =
                        &c[i * rhs.width + j] + &a[i * self.width + k] * &b[k * rhs.width + j];
                }
            }
        }

        Ok(Matrix::from_vec(c, rhs.width)?)
    }
}

impl<T> Add<&Matrix<T>> for Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Result<Matrix<T>, MatrixError>;

    fn add(self, rhs: &Matrix<T>) -> Self::Output {
        if self.width != rhs.width || self.height != rhs.height {
            return Err(MatrixError::SizeMismatch);
        }
        let mut c = Vec::with_capacity(self.width * self.height);
        c.resize(self.width * self.height, 0.0.into());
        let a = self.elems_raw();
        let b = rhs.elems_raw();

        for i in 0..self.width * self.height {
            c[i] = &a[i] + &b[i];
        }

        Ok(Matrix::from_vec(c, self.width)?)
    }
}

impl<T> Sub<&Matrix<T>> for Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Result<Matrix<T>, MatrixError>;

    fn sub(self, rhs: &Matrix<T>) -> Self::Output {
        if self.width != rhs.width || self.height != rhs.height {
            return Err(MatrixError::SizeMismatch);
        }
        let mut c = Vec::with_capacity(self.width * self.height);
        c.resize(self.width * self.height, 0.0.into());
        let a = self.elems_raw();
        let b = rhs.elems_raw();

        for i in 0..self.width * self.height {
            c[i] = &a[i] - &b[i];
        }

        Ok(Matrix::from_vec(c, self.width)?)
    }
}

impl<T> Mul<&Matrix<T>> for Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Result<Matrix<T>, MatrixError>;

    fn mul(self, rhs: &Matrix<T>) -> Self::Output {
        if self.width != rhs.height {
            return Err(MatrixError::SizeMismatch);
        }
        let mut c = Vec::with_capacity(rhs.width * self.height);
        c.resize(rhs.width * self.height, 0.0.into());
        let a = self.elems_raw();
        let b = rhs.elems_raw();

        for i in 0..self.height {
            for j in 0..rhs.width {
                for k in 0..self.width {
                    c[i * rhs.width + j] =
                        &c[i * rhs.width + j] + &a[i * self.width + k] * &b[k * rhs.width + j];
                }
            }
        }

        Ok(Matrix::from_vec(c, rhs.width)?)
    }
}

impl<T> Add<Matrix<T>> for &Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Result<Matrix<T>, MatrixError>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        if self.width != rhs.width || self.height != rhs.height {
            return Err(MatrixError::SizeMismatch);
        }
        let mut c = Vec::with_capacity(self.width * self.height);
        c.resize(self.width * self.height, 0.0.into());
        let a = self.elems_raw();
        let b = rhs.elems_raw();

        for i in 0..self.width * self.height {
            c[i] = &a[i] + &b[i];
        }

        Ok(Matrix::from_vec(c, self.width)?)
    }
}

impl<T> Sub<Matrix<T>> for &Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Result<Matrix<T>, MatrixError>;

    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        if self.width != rhs.width || self.height != rhs.height {
            return Err(MatrixError::SizeMismatch);
        }
        let mut c = Vec::with_capacity(self.width * self.height);
        c.resize(self.width * self.height, 0.0.into());
        let a = self.elems_raw();
        let b = rhs.elems_raw();

        for i in 0..self.width * self.height {
            c[i] = &a[i] - &b[i];
        }

        Ok(Matrix::from_vec(c, self.width)?)
    }
}

impl<T> Mul<Matrix<T>> for &Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Result<Matrix<T>, MatrixError>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        if self.width != rhs.height {
            return Err(MatrixError::SizeMismatch);
        }
        let mut c = Vec::with_capacity(rhs.width * self.height);
        c.resize(rhs.width * self.height, 0.0.into());
        let a = self.elems_raw();
        let b = rhs.elems_raw();

        for i in 0..self.height {
            for j in 0..rhs.width {
                for k in 0..self.width {
                    c[i * rhs.width + j] =
                        &c[i * rhs.width + j] + &a[i * self.width + k] * &b[k * rhs.width + j];
                }
            }
        }

        Ok(Matrix::from_vec(c, rhs.width)?)
    }
}

impl<T> Add<&Matrix<T>> for &Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Result<Matrix<T>, MatrixError>;

    fn add(self, rhs: &Matrix<T>) -> Self::Output {
        if self.width != rhs.width || self.height != rhs.height {
            return Err(MatrixError::SizeMismatch);
        }
        let mut c = Vec::with_capacity(self.width * self.height);
        c.resize(self.width * self.height, 0.0.into());
        let a = self.elems_raw();
        let b = rhs.elems_raw();

        for i in 0..self.width * self.height {
            c[i] = &a[i] + &b[i];
        }

        Ok(Matrix::from_vec(c, self.width)?)
    }
}

impl<T> Sub<&Matrix<T>> for &Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Result<Matrix<T>, MatrixError>;

    fn sub(self, rhs: &Matrix<T>) -> Self::Output {
        if self.width != rhs.width || self.height != rhs.height {
            return Err(MatrixError::SizeMismatch);
        }
        let mut c = Vec::with_capacity(self.width * self.height);
        c.resize(self.width * self.height, 0.0.into());
        let a = self.elems_raw();
        let b = rhs.elems_raw();

        for i in 0..self.width * self.height {
            c[i] = &a[i] - &b[i];
        }

        Ok(Matrix::from_vec(c, self.width)?)
    }
}

impl<T> Mul<&Matrix<T>> for &Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Result<Matrix<T>, MatrixError>;

    fn mul(self, rhs: &Matrix<T>) -> Self::Output {
        if self.width != rhs.height {
            return Err(MatrixError::SizeMismatch);
        }
        let mut c = Vec::with_capacity(rhs.width * self.height);
        c.resize(rhs.width * self.height, 0.0.into());
        let a = self.elems_raw();
        let b = rhs.elems_raw();

        for i in 0..self.height {
            for j in 0..rhs.width {
                for k in 0..self.width {
                    c[i * rhs.width + j] =
                        &c[i * rhs.width + j] + &a[i * self.width + k] * &b[k * rhs.width + j];
                }
            }
        }

        Ok(Matrix::from_vec(c, rhs.width)?)
    }
}

impl<T> Mul<T> for Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut elems = Vec::with_capacity(self.width * self.height);
        for i in 0..self.width * self.height {
            elems.push(&rhs * &self.elems[i]);
        }

        Matrix::from_vec(elems, self.width).unwrap()
    }
}

impl<T> Mul<T> for &Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut elems = Vec::with_capacity(self.width * self.height);
        for i in 0..self.width * self.height {
            elems.push(&rhs * &self.elems[i]);
        }

        Matrix::from_vec(elems, self.width).unwrap()
    }
}

impl<T> Div<T> for Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Matrix<T>;

    fn div(self, rhs: T) -> Self::Output {
        let mut elems = Vec::with_capacity(self.width * self.height);
        for i in 0..self.width * self.height {
            elems.push(&self.elems[i] / &rhs);
        }

        Matrix::from_vec(elems, self.width).unwrap()
    }
}

impl<T> Div<T> for &Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    type Output = Matrix<T>;

    fn div(self, rhs: T) -> Self::Output {
        let mut elems = Vec::with_capacity(self.width * self.height);
        for i in 0..self.width * self.height {
            elems.push(&self.elems[i] / &rhs);
        }

        Matrix::from_vec(elems, self.width).unwrap()
    }
}
