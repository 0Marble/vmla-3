use std::{fs::File, path::PathBuf, time::Instant};

use crate::{
    io::{read_mat, write_mat_complex, write_mat_f32, Either},
    measure,
    number::{NumNonRef, NumRef},
};

use super::matrix::*;

pub fn lu_decomposition<T>(mat: &Matrix<T>) -> Result<(Matrix<T>, Matrix<T>), MatrixError>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    if mat.width() != mat.height() {
        return Err(MatrixError::NotSquare);
    }
    let width = mat.width();

    let mut l = Vec::with_capacity(width * width);
    l.resize(width * width, 0.0.into());
    let mut u = l.clone();
    let mut d = mat.elems_raw().to_owned();

    for layer in 0..mat.width() {
        let a = d[layer * width + layer].clone();
        if a.clone() == 0.0.into() {
            return Err(MatrixError::NotRegular);
        }

        l[layer * width + layer] = 1.0.into();
        u[layer * width + layer] = a.clone();

        //this can be rewritten to run in parallel
        for i in layer + 1..width {
            l[i * width + layer] = &d[i * width + layer] / &a;
            u[layer * width + i] = d[layer * width + i].clone();

            for j in layer + 1..width {
                d[i * width + j] =
                    &d[i * width + j] - &(&d[layer * width + j] * &d[i * width + layer]) / &a;
            }
        }
    }

    let l = Matrix::from_vec(l, width)?;
    let u = Matrix::from_vec(u, width)?;

    Ok((l, u))
}

pub fn gauss_from_lu<T>(
    l: &Matrix<T>,
    u: &Matrix<T>,
    b: &Matrix<T>,
) -> Result<Matrix<T>, MatrixError>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    if l.width() != l.height()
        || u.width() != u.height()
        || b.width() != 1
        || b.height() != l.height()
        || l.width() != u.width()
    {
        return Err(MatrixError::SizeMismatch);
    }

    let v = l_gauss(l, b);
    let x = u_gauss(u, &v);

    Ok(x)
}

fn l_gauss<T>(l: &Matrix<T>, b: &Matrix<T>) -> Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    let mut x = Matrix::new(1, l.width());
    for i in 0..l.width() {
        let mut xi = b.get(i, 0).clone();
        for j in 0..i {
            xi = xi - l.get(i, j) * x.get(j, 0);
        }
        x.set(i, 0, xi);
    }

    x
}

fn u_gauss<T>(u: &Matrix<T>, b: &Matrix<T>) -> Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    let mut x = Matrix::new(1, u.width());
    for i in 0..u.width() {
        let mut xi = b.get(u.width() - i - 1, 0).clone();
        for j in 0..i {
            xi = xi - u.get(u.width() - i - 1, u.width() - j - 1) * x.get(u.width() - j - 1, 0);
        }
        x.set(
            u.width() - i - 1,
            0,
            &xi / u.get(u.width() - i - 1, u.width() - i - 1),
        );
    }
    x
}

pub fn make_lu(dir: &PathBuf, problem: usize) -> Result<(), MatrixError> {
    let file_path = dir.join(format!("Amat{problem}.m"));
    let l_path = dir.join(format!("Lmat{problem}.m"));
    let u_path = dir.join(format!("Umat{problem}.m"));

    println!("Problem {}", problem);
    let (mat, _) = read_mat(&mut File::open(&file_path)?)?;

    match mat {
        Either::Left(mat) => {
            let ((l, u), lu_duration) = measure!(lu_decomposition(&mat)?);
            write_mat_f32(&l, &l_path)?;
            write_mat_f32(&u, &u_path)?;

            println!(
                "\tTook {}μs, ∥LU - A∥ = {}",
                lu_duration.as_micros(),
                ((l * u)? - &mat)?.norm()
            );
        }
        Either::Right(mat) => {
            let ((l, u), lu_duration) = measure!(lu_decomposition(&mat)?);
            write_mat_complex(&l, &l_path)?;
            write_mat_complex(&u, &u_path)?;

            println!(
                "\tTook {}μs, ∥LU - A∥ = {}",
                lu_duration.as_micros(),
                ((l * u)? - &mat)?.norm()
            );
        }
    }

    Ok(())
}

pub fn lu_gauss(dir: &PathBuf, problem: usize) -> Result<(), MatrixError> {
    let (b, _) = read_mat(&mut File::open(dir.join(format!("bvec{problem}.m")))?)?;
    let (l, u) = match (
        File::open(dir.join(format!("Lmat{problem}.m"))),
        File::open(dir.join(format!("Umat{problem}.m"))),
    ) {
        (Ok(mut l), Ok(mut u)) => (read_mat(&mut l)?.0, read_mat(&mut u)?.0),
        _ => {
            let (a, _) = read_mat(&mut File::open(dir.join(format!("Amat{problem}.m")))?)?;
            match a {
                Either::Left(a) => {
                    let (l, u) = lu_decomposition(&a)?;
                    (Either::Left(l), Either::Left(u))
                }
                Either::Right(a) => {
                    let (l, u) = lu_decomposition(&a)?;
                    (Either::Right(l), Either::Right(u))
                }
            }
        }
    };

    println!("Problem {}", problem);

    match b {
        Either::Left(b) => {
            let l = l.unwrap_left();
            let u = u.unwrap_left();
            let (x, duration) = measure!(gauss_from_lu(l, u, &b)?);
            write_mat_f32(&x, &dir.join(format!("xvec{problem}.m")))?;

            println!(
                "\tTook {}μs, ∥LUx - b∥ = {}",
                duration.as_micros(),
                ((l * (u * x)?)? - b)?.norm()
            );
        }
        Either::Right(b) => {
            let l = l.unwrap_right();
            let u = u.unwrap_right();
            let (x, duration) = measure!(gauss_from_lu(l, u, &b)?);
            write_mat_complex(&x, &dir.join(format!("xvec{problem}.m")))?;
            println!(
                "\tTook {}μs, ∥LUx - b∥ = {}",
                duration.as_micros(),
                ((l * (u * x)?)? - b)?.norm()
            );
        }
    }

    Ok(())
}
