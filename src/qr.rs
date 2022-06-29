use std::{fs::File, path::PathBuf, time::Instant};

use crate::{
    io::{read_mat, write_mat_complex, write_mat_f32, Either, QRMethod},
    matrix::{Matrix, MatrixError},
    measure,
    number::{NumNonRef, NumRef},
};

pub fn qr_householder<T>(mat: &Matrix<T>) -> Result<(Matrix<T>, Matrix<T>), MatrixError>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    let width = mat.width();
    if width != mat.height() {
        return Err(MatrixError::NotSquare);
    }

    let mut r = mat.clone();
    let mut q = Matrix::identity(width);

    for layer in 0..width {
        let mut column_norm = 0.0;
        for i in layer..width {
            column_norm += r.get(i, layer).norm_squared();
        }

        let mut v = Matrix::new(1, width);

        let a = r.get(layer, layer).clone();
        if a.norm() != 0.0 {
            v.set(
                layer,
                0,
                &a + a.clone() / a.norm().into() * column_norm.sqrt().into(),
            );
        }
        for i in layer + 1..width {
            v.set(i, 0, r.get(i, layer).clone());
        }
        v = &v / v.norm().into();

        mirror_vecs(&mut r, &v);
        mirror_vecs(&mut q, &v);
    }

    Ok((q.transpose(), r))
}

fn mirror_vecs<T>(vecs: &mut Matrix<T>, mirror_direction: &Matrix<T>)
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    for i in 0..vecs.width() {
        let mut dot: T = 0.0.into();
        for j in 0..vecs.height() {
            dot = dot + &mirror_direction.get(j, 0).conjugate() * vecs.get(j, i);
        }

        for j in 0..vecs.height() {
            vecs.set(
                j,
                i,
                &(mirror_direction.get(j, 0) * &dot * T::from(-2.0)) + vecs.get(j, i),
            );
        }
    }
}

pub fn qr_givens(mat: &Matrix<f32>) -> Result<(Matrix<f32>, Matrix<f32>), MatrixError> {
    let width = mat.width();
    if width != mat.height() {
        return Err(MatrixError::NotSquare);
    }

    let mut r = mat.clone();
    let mut q = Matrix::identity(width);

    for i in 1..width {
        for j in 0..i {
            zero_by_rotation(&mut q, &mut r, i, j);
        }
    }

    Ok((q.transpose(), r))
}

fn zero_by_rotation(
    q: &mut Matrix<f32>,
    r: &mut Matrix<f32>,
    element_to_zero_row: usize,
    element_to_zero_column: usize,
) {
    let a = r.get(element_to_zero_column, element_to_zero_column);
    let b = r.get(element_to_zero_row, element_to_zero_column);
    let ab = (a * a + b * b).sqrt();
    let cos = a / ab;
    let sin = -b / ab;

    for i in 0..r.width() {
        let a = r.get(element_to_zero_column, i).clone();
        let b = r.get(element_to_zero_row, i).clone();
        r.set(element_to_zero_column, i, a * cos - b * sin);
        r.set(element_to_zero_row, i, a * sin + b * cos);

        let a = q.get(element_to_zero_column, i).clone();
        let b = q.get(element_to_zero_row, i).clone();
        q.set(element_to_zero_column, i, a * cos - b * sin);
        q.set(element_to_zero_row, i, a * sin + b * cos);
    }
}

pub fn qr_gram_schmidt<T>(
    mat: &Matrix<T>,
    reortho_epsilon: f32,
) -> Result<(Matrix<T>, Matrix<T>), MatrixError>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    let width = mat.width();
    if mat.width() != mat.height() {
        return Err(MatrixError::NotSquare);
    }

    let mut q = Matrix::new(width, width);
    let mut r = Matrix::new(width, width);

    for j in 0..width {
        let mut p = mat.column(j);

        loop {
            let mut delta = 0.0;
            for i in 0..j {
                let mut dot = 0.0.into();
                for k in 0..width {
                    dot = dot + &q.get(k, i).conjugate() * p.get(k, 0);
                }

                for k in 0..width {
                    let a = p.get(k, 0) - q.get(k, i) * &dot;
                    delta += (&a - p.get(k, 0)).norm_squared();
                    p.set(k, 0, a);
                }
            }
            if delta < reortho_epsilon {
                break;
            }
        }

        for i in 0..width {
            q.set(i, j, p.get(i, 0) / &p.norm().into());
        }

        for i in 0..j + 1 {
            let mut dot = 0.0.into();
            for k in 0..width {
                dot = dot + &q.get(k, i).conjugate() * mat.get(k, j);
            }
            r.set(i, j, dot);
        }
    }

    Ok((q, r))
}

pub fn gauss_from_qr<T>(
    q: &Matrix<T>,
    r: &Matrix<T>,
    b: &Matrix<T>,
) -> Result<Matrix<T>, MatrixError>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    if q.width() != q.height()
        || r.width() != r.height()
        || b.width() != 1
        || b.height() != q.height()
        || r.width() != q.width()
    {
        return Err(MatrixError::SizeMismatch);
    }
    let v = (q.hermetian_transpose() * b)?;
    Ok(r_gauss(r, &v))
}

fn r_gauss<T>(r: &Matrix<T>, b: &Matrix<T>) -> Matrix<T>
where
    T: NumNonRef,
    for<'a> &'a T: NumRef<T>,
{
    let mut x = Matrix::new(1, r.width());
    for i in 0..r.width() {
        let mut xi = b.get(r.width() - i - 1, 0).clone();
        for j in 0..i {
            xi = xi - r.get(r.width() - i - 1, r.width() - j - 1) * x.get(r.width() - j - 1, 0);
        }
        x.set(
            r.width() - i - 1,
            0,
            &xi / r.get(r.width() - i - 1, r.width() - i - 1),
        );
    }
    x
}

pub fn make_qr(dir: &PathBuf, problem: usize) -> Result<(), MatrixError> {
    let (mat, method) = read_mat(&mut File::open(dir.join(format!("Amat{problem}.m")))?)?;
    println!("Problem {}", problem);

    match method {
        Some(method) => match method {
            QRMethod::Householder => match mat {
                Either::Left(mat) => {
                    let ((q, r), duration) = measure!(qr_householder(&mat)?);
                    write_mat_f32(&q, &dir.join(format!("Qmat{problem}.m")))?;
                    write_mat_f32(&r, &dir.join(format!("Rmat{problem}.m")))?;

                    println!(
                        "\tTook {}μs, ∥QR - A∥ = {}",
                        duration.as_micros(),
                        ((q * r)? - mat)?.norm()
                    );
                }
                Either::Right(mat) => {
                    let ((q, r), duration) = measure!(qr_householder(&mat)?);
                    write_mat_complex(&q, &dir.join(format!("Qmat{problem}.m")))?;
                    write_mat_complex(&r, &dir.join(format!("Rmat{problem}.m")))?;

                    println!(
                        "\tTook {}μs, ∥QR - A∥ = {}",
                        duration.as_micros(),
                        ((q * r)? - mat)?.norm()
                    );
                }
            },
            QRMethod::Givens => match mat {
                Either::Left(mat) => {
                    let ((q, r), duration) = measure!(qr_givens(&mat)?);
                    write_mat_f32(&q, &dir.join(format!("Qmat{problem}.m")))?;
                    write_mat_f32(&r, &dir.join(format!("Rmat{problem}.m")))?;

                    println!(
                        "\tTook {}μs, ∥QR - A∥ = {}",
                        duration.as_micros(),
                        ((q * r)? - mat)?.norm()
                    );
                }
                Either::Right(_) => return Err(MatrixError::UnsopportedOperation),
            },
            QRMethod::GramSchmidt => match mat {
                Either::Left(mat) => {
                    let ((q, r), duration) = measure!(qr_gram_schmidt(&mat, 0.1)?);
                    write_mat_f32(&q, &dir.join(format!("Qmat{problem}.m")))?;
                    write_mat_f32(&r, &dir.join(format!("Rmat{problem}.m")))?;

                    println!(
                        "\tTook {}μs, ∥QR - A∥ = {}",
                        duration.as_micros(),
                        ((q * r)? - mat)?.norm()
                    );
                }
                Either::Right(mat) => {
                    let ((q, r), duration) = measure!(qr_gram_schmidt(&mat, 0.1)?);
                    write_mat_complex(&q, &dir.join(format!("Qmat{problem}.m")))?;
                    write_mat_complex(&r, &dir.join(format!("Rmat{problem}.m")))?;

                    println!(
                        "\tTook {}μs, ∥QR - A∥ = {}",
                        duration.as_micros(),
                        ((q * r)? - mat)?.norm()
                    );
                }
            },
        },
        None => {
            println!("No method given! Assuming Gram-Shmidt");
            match mat {
                Either::Left(mat) => {
                    let ((q, r), duration) = measure!(qr_gram_schmidt(&mat, 0.1)?);
                    write_mat_f32(&q, &dir.join(format!("Qmat{problem}.m")))?;
                    write_mat_f32(&r, &dir.join(format!("Rmat{problem}.m")))?;

                    println!(
                        "\tTook {}μs, ∥QR - A∥ = {}",
                        duration.as_micros(),
                        ((q * r)? - mat)?.norm()
                    );
                }
                Either::Right(mat) => {
                    let ((q, r), duration) = measure!(qr_gram_schmidt(&mat, 0.1)?);
                    write_mat_complex(&q, &dir.join(format!("Qmat{problem}.m")))?;
                    write_mat_complex(&r, &dir.join(format!("Rmat{problem}.m")))?;

                    println!(
                        "\tTook {}μs, ∥QR - A∥ = {}",
                        duration.as_micros(),
                        ((q * r)? - mat)?.norm()
                    );
                }
            }
        }
    }

    Ok(())
}

pub fn qr_gauss(dir: &PathBuf, problem: usize) -> Result<(), MatrixError> {
    let (b, _) = read_mat(&mut File::open(dir.join(format!("bvec{problem}.m")))?)?;

    println!("Problem {}", problem);

    let (q, r) = match (
        File::open(dir.join(format!("Qmat{problem}.m"))),
        File::open(dir.join(format!("Rmat{problem}.m"))),
    ) {
        (Ok(mut q), Ok(mut r)) => (read_mat(&mut q)?.0, read_mat(&mut r)?.0),
        _ => {
            let (mat, method) = read_mat(&mut File::open(dir.join(format!("Amat{problem}.m")))?)?;
            match mat {
                Either::Left(mat) => {
                    let (q, r) = match method {
                        Some(m) => match m {
                            QRMethod::Householder => qr_householder(&mat)?,
                            QRMethod::Givens => qr_givens(&mat)?,
                            QRMethod::GramSchmidt => qr_gram_schmidt(&mat, 0.1)?,
                        },
                        None => qr_householder(&mat)?,
                    };
                    (Either::Left(q), Either::Left(r))
                }
                Either::Right(mat) => {
                    let (q, r) = match method {
                        Some(m) => match m {
                            QRMethod::Householder => qr_householder(&mat)?,
                            QRMethod::GramSchmidt => qr_gram_schmidt(&mat, 0.1)?,
                            _ => panic!("No Givens method for complex matrices"),
                        },
                        None => qr_householder(&mat)?,
                    };
                    (Either::Right(q), Either::Right(r))
                }
            }
        }
    };

    match b {
        Either::Left(b) => {
            let q = q.unwrap_left();
            let r = r.unwrap_left();
            let (x, duration) = measure!(gauss_from_qr(q, r, &b)?);
            write_mat_f32(&x, &dir.join(format!("xvec{problem}.m")))?;

            println!(
                "\tTook {}μs, ∥QRx - b∥ = {}",
                duration.as_micros(),
                ((q * (r * x)?)? - b)?.norm()
            );
        }
        Either::Right(b) => {
            let q = q.unwrap_right();
            let r = r.unwrap_right();
            let (x, duration) = measure!(gauss_from_qr(q, r, &b)?);
            write_mat_complex(&x, &dir.join(format!("xvec{problem}.m")))?;

            println!(
                "\tTook {}μs, ∥QRx - b∥ = {}",
                duration.as_micros(),
                ((q * (r * x)?)? - b)?.norm()
            );
        }
    }

    Ok(())
}
