use std::{fmt::Debug, fs::File, io::Write, path::PathBuf, time::Instant};

use crate::{
    io::read_mat,
    longint::LongInt,
    matrix::{Matrix, MatrixError},
    measure,
    number::{from_f32_mat, NumNonRef, NumRef},
    poly::Polynome,
};

pub fn characteristic_polynomial<T>(mat: &Matrix<T>) -> Result<Polynome<T>, MatrixError>
where
    T: NumNonRef + Debug,
    for<'a> &'a T: NumRef<T>,
{
    if mat.width() != mat.height() {
        return Err(MatrixError::NotSquare);
    }

    if !is_tridiagonal(mat, 0.0001) {
        return Err(MatrixError::NotTridiagonal);
    }

    let width = mat.width();
    match width {
        0 => Ok(Polynome::from_coefs(&[0.0.into()])),
        1 => Ok(Polynome::from_coefs(&[-mat.get(0, 0).clone(), 1.0.into()])),
        2 => {
            let a = mat.get(0, 0);
            let b = mat.get(0, 1);
            let c = mat.get(1, 0);
            let d = mat.get(1, 1);
            Ok(Polynome::from_coefs(&[
                a * d - c * b,
                (d + a) * T::from(-1.0),
                1.0.into(),
            ]))
        }
        width => {
            let a = mat.get(0, 0);
            let b = mat.get(0, 1);
            let c = mat.get(1, 0);
            let d = mat.get(1, 1);
            let mut p = Vec::with_capacity(width);
            p.push(Polynome::from_coefs(&[a.clone(), (-1.0).into()]));
            p.push(Polynome::from_coefs(&[
                a * d - c * b,
                (d + a) * T::from(-1.0),
                1.0.into(),
            ]));

            for i in 2..width {
                let p1 = Polynome::from_coefs(&[mat.get(i, i).clone(), (-1.0).into()]);
                let p2 = mat.get(i, i - 1) * mat.get(i - 1, i);

                let p3 = &p[i - 1] * p1 - &p[i - 2] * p2;

                p.push(p3);
            }

            Ok(p.pop().unwrap())
        }
    }
}

fn is_tridiagonal<T>(mat: &Matrix<T>, close_enough_to_zero: f32) -> bool
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    for i in 0..mat.height() {
        for j in 0..mat.width() {
            if i == j || (j > 0 && j - 1 == i) || (i > 0 && i - 1 == j) {
                continue;
            }
            if mat.get(i, j).norm_squared() > close_enough_to_zero {
                return false;
            }
        }
    }
    return true;
}

pub fn find_poly(dir: &PathBuf, problem: usize) -> Result<(), MatrixError> {
    let mat_file = dir.join(format!("Amat{problem}.m"));
    println!("Problem {problem}");
    let m = read_mat(&mut File::open(&mat_file)?)?
        .0
        .unwrap_left()
        .clone();
    let m = from_f32_mat::<LongInt>(&m);
    let (p, duration) = measure!(characteristic_polynomial(&m)?);
    println!("\tTook {}μs", duration.as_micros());
    write!(File::create(dir.join(format!("cvec{problem}.m")))?, "{}", p)?;

    Ok(())
}
